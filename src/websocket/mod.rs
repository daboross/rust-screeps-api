//! Handling of socket connections to screeps using ws-rs as a backend.
use std::borrow::Cow;
use std::str;

use {serde_json, url};
use rand::{self, Rng};

use Token;

pub mod parsing;
mod error;

pub use self::error::UrlError;

/// Different channels one can subscribe to.
pub enum Channel<'a> {
    /// Server messages (TODO: find message here).
    ServerMessages,
    /// User CPU and memory usage updates. Sent at the end of each tick.
    UserCpu {
        /// The user ID of the subscription.
        user_id: Cow<'a, str>,
    },
    /// User message updates. Sent when the user receives any new message.
    UserMessages {
        /// The user ID of the subscription.
        user_id: Cow<'a, str>,
    },
    /// Specific conversation alerts. Updates when a new message is received from a particular user.
    UserConversation {
        /// The user ID of the connected user.
        user_id: Cow<'a, str>,
        /// The user ID on the other side of the conversation to listen to.
        target_user_id: Cow<'a, str>,
    },
    /// User credit alerts. Updates whenever the user's credit changes.
    UserCredits {
        /// The user ID of the subscription.
        user_id: Cow<'a, str>,
    },
    /// Memory path alerts. Updates whenever this specific memory path changes.
    UserMemoryPath {
        /// The user ID of the subscription.
        user_id: Cow<'a, str>,
        /// The memory path, separated with '.'.
        path: Cow<'a, str>,
    },
    /// Console alerts. Updates at the end of every tick with all console messages during that tick.
    UserConsole {
        /// The user ID of the subscription.
        user_id: Cow<'a, str>,
    },
    /// User active branch changes: updates whenever the active branch changes.
    UserActiveBranch {
        /// The user ID of the subscription.
        user_id: Cow<'a, str>,
    },
    /// Room overview updates. Updates at the end of every tick with all room positions for each nondescript
    /// type of structure (road, wall, energy, or player owned).
    RoomMapView {
        /// The room name of the subscription.
        room_name: Cow<'a, str>,
    },
    /// Detailed room updates. Updates at the end of every tick with all room object properties which have
    /// changed since the last tick.
    ///
    /// Note: this is limited to 2 per user account at a time, and if there are more than 2 room subscriptions active,
    /// it is random which 2 will received updates on any given ticks. Rooms which are not updated do receive an error
    /// message on "off" ticks.
    RoomDetail {
        /// The room name of the subscription.
        room_name: Cow<'a, str>,
    },
    /// A channel specified by the exact channel id.
    Other {
        /// The channel protocol string.
        channel: Cow<'a, str>,
    },
}

impl<'a> Channel<'a> {
    /// Creates a channel subscribing to server messages.
    pub fn server_messages() -> Self {
        Channel::ServerMessages
    }

    /// Creates a channel subscribing to a user's CPU and memory.
    pub fn user_cpu<T: Into<Cow<'a, str>>>(user_id: T) -> Self {
        Channel::UserCpu { user_id: user_id.into() }
    }

    /// Creates a channel subscribing to a user's new message notifications.
    pub fn user_messages<T: Into<Cow<'a, str>>>(user_id: T) -> Self {
        Channel::UserMessages { user_id: user_id.into() }
    }

    /// Creates a channel subscribing to new messages in a user's specific conversation.
    pub fn user_conversation<T, U>(user_id: T, target_user_id: U) -> Self
        where T: Into<Cow<'a, str>>,
              U: Into<Cow<'a, str>>
    {
        Channel::UserConversation {
            user_id: user_id.into(),
            target_user_id: target_user_id.into(),
        }
    }

    /// Creates a channel subscribing to a user's credit count.
    pub fn user_credits<T: Into<Cow<'a, str>>>(user_id: T) -> Self {
        Channel::UserCredits { user_id: user_id.into() }
    }

    /// Creates a channel subscribing to a path in a user's memory.
    pub fn user_memory_path<T, U>(user_id: T, path: U) -> Self
        where T: Into<Cow<'a, str>>,
              U: Into<Cow<'a, str>>
    {
        Channel::UserMemoryPath {
            user_id: user_id.into(),
            path: path.into(),
        }
    }

    /// Creates a channel subscribing to a user's console output.
    pub fn user_console<T: Into<Cow<'a, str>>>(user_id: T) -> Self {
        Channel::UserConsole { user_id: user_id.into() }
    }

    /// Creates a channel subscribing to when a user's active code branch changes.
    pub fn user_active_branch<T: Into<Cow<'a, str>>>(user_id: T) -> Self {
        Channel::UserActiveBranch { user_id: user_id.into() }
    }

    /// Creates a channel subscribing to map-view updates of a room.
    pub fn room_map_view<T: Into<Cow<'a, str>>>(room_name: T) -> Self {
        Channel::RoomMapView { room_name: room_name.into() }
    }

    /// Creates a channel subscribing to detailed updates of a room's contents.
    ///
    /// Note: this is limited to 2 per user account at a time, and if there are more than 2 room subscriptions active,
    /// it is random which 2 will received updates on any given ticks. Rooms which are not updated do receive an error
    /// message on "off" ticks.
    pub fn room_detail<T: Into<Cow<'a, str>>>(room_name: T) -> Self {
        Channel::RoomDetail { room_name: room_name.into() }
    }

    /// Creates a channel using the fully specified channel name.
    pub fn other<T: Into<Cow<'a, str>>>(channel: T) -> Self {
        Channel::Other { channel: channel.into() }
    }

    /// This is a really wonky scheme, but it is probably the best one right now.
    ///
    /// Adds the channel description to the message (does not add preceding space) and collects to a vec.
    fn chain_and_complete_message<T: Iterator<Item = char>>(&self, start: T) -> String {
        match *self {
            Channel::ServerMessages => start.chain("server-message".chars()).collect(),
            Channel::UserCpu { ref user_id } => {
                start.chain("user:".chars()).chain(user_id.as_ref().chars()).chain("/cpu".chars()).collect()
            }
            Channel::UserMessages { ref user_id } => {
                start.chain("user:".chars()).chain(user_id.as_ref().chars()).chain("/newMessage".chars()).collect()
            }
            Channel::UserConversation { ref user_id, ref target_user_id } => {
                start.chain("user:".chars())
                    .chain(user_id.as_ref().chars())
                    .chain("/message:".chars())
                    .chain(target_user_id.as_ref().chars())
                    .collect()
            }
            Channel::UserCredits { ref user_id } => {
                start.chain("user:".chars()).chain(user_id.as_ref().chars()).chain("/money".chars()).collect()
            }
            Channel::UserMemoryPath { ref user_id, ref path } => {
                start.chain("user:".chars())
                    .chain(user_id.as_ref().chars())
                    .chain("/memory/".chars())
                    .chain(path.as_ref().chars())
                    .collect()
            }
            Channel::UserConsole { ref user_id } => {
                start.chain("user:".chars()).chain(user_id.as_ref().chars()).chain("/console".chars()).collect()
            }
            Channel::UserActiveBranch { ref user_id } => {
                start.chain("user:".chars())
                    .chain(user_id.as_ref().chars())
                    .chain("/set-active-branch".chars())
                    .collect()
            }
            Channel::RoomMapView { ref room_name } => {
                start.chain("roomMap2:".chars()).chain(room_name.as_ref().chars()).collect()
            }
            Channel::RoomDetail { ref room_name } => {
                start.chain("room:".chars()).chain(room_name.as_ref().chars()).collect()
            }
            Channel::Other { ref channel } => start.chain(channel.as_ref().chars()).collect(),
        }
    }

    /// Allocates a vec with the byte representation of this channel.
    pub fn to_string(&self) -> String {
        self.chain_and_complete_message("".chars())
    }
}

/// Gets the raw websocket string to send for subscribing to a channel.
///
/// Subscribing to a channel you are already subscribed to may have differing
/// results depending on the server software (official vs. private). Note that if you subscribe
/// multiple times, it may be necessary to unsubscribe at least that many times to fully unsubscribe.
/// Subscribing multiple times may or may not result in duplicated messages, and may or may not
/// result in extra initial messages.
///
/// It's recommended that you keep track of what channels you are subscribed to separately: this
/// is tracked by the server, but is not tracked by `screeps-api`, and cannot be queried from the
/// server.
pub fn subscribe(channel: Channel) -> String {
    let message = channel.chain_and_complete_message("subscribe ".chars());

    sockjs_send_from_internal(&message)
}

/// Gets the raw websocket string to send for unsubscribing to a channel.
///
/// Unsubscribing from a channel you are not subscribed to appears to have no affect.
///
/// Recommended that you keep track of what channels you are subscribed to separately: this
/// is tracked by the server, but is not tracked by `screeps-api`, and cannot be queried from the
/// server.
pub fn unsubscribe(channel: Channel) -> String {
    let message = channel.chain_and_complete_message("unsubscribe ".chars());

    sockjs_send_from_internal(&message)
}

/// Authenticates with the given token.
///
/// After doing this, you'll be able to subscribe and unsubscribe to messages. A "auth success" message
/// will happen as a response which returns either this token or a new one.
pub fn authenticate(token: Token) -> String {

    let message = "auth "
        .chars()
        .chain(token.chars())
        .collect::<String>();

    sockjs_send_from_internal(&message)
}

fn sockjs_send_from_internal<T: AsRef<str>>(source: &T) -> String {
    serde_json::to_string(&(source.as_ref(),)).expect("serializing a tuple containing a single string can't fail.")
}

/// Creates a URL from the default official screeps server API URL.
pub fn default_websocket_url() -> url::Url {
    use DEFAULT_URL_STR;

    websocket_url(DEFAULT_URL_STR).expect("expected known good default URL to parse successfully.")
}

/// Method for finding a websocket URL given the screeps API URL.
///
/// This method uses the thread-local `rand` crate rng to come up with a unique
/// session id, and the resulting url should not be reused over multiple connections.
///
/// The input URL should be an API url in the format of `https://screeps.com/api/`.
pub fn websocket_url<U: AsRef<str> + ?Sized>(url: &U) -> Result<url::Url, UrlError> {
    use std::fmt;

    let mut url = match url.as_ref().parse::<url::Url>() {
        Ok(v) => v,
        Err(e) => return Err(UrlError::from_err(e, url.as_ref().to_owned())),
    };

    let new_scheme = match url.scheme() {
        "http" => Ok("ws"),
        "https" => Ok("wss"),
        other => Err(other.to_string()),
    };

    let new_scheme = match new_scheme {
        Ok(v) => v,
        Err(other) => return Err(UrlError::wrong_scheme(other, url)),
    };

    url.set_scheme(new_scheme).expect("expected `ws` and `wss` to be valid url schemes.");


    // we could probably just use gen_ascii_chars for the session ID, but to be safe
    // we just use the subset that `sockjs-client` does.
    const VALID_CHARS: &'static [u8] = b"abcdefghijklmnopqrstuvwxyz012345";

    // avoiding allocations!
    struct GenServerAndSessionId;

    impl fmt::Display for GenServerAndSessionId {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let mut rng = rand::thread_rng();
            write!(f, "../socket/{:04}/", rng.gen_range(0, 1000))?;

            for _ in 0..8 {
                write!(f, "{}", *rng.choose(VALID_CHARS).unwrap() as char)?;
            }
            write!(f, "/websocket")?;

            Ok(())
        }
    }

    let result = url.join(&GenServerAndSessionId.to_string())
        .expect("expected generated string known to be correct to parse successfully");

    Ok(result)
}
