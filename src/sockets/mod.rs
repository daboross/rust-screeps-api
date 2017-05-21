//! Handling of socket connections to screeps using ws-rs as a backend.

pub extern crate ws;
extern crate rand;
extern crate fnv;
use {serde_json, url};

use std::time::Duration;
use std::borrow::Cow;
use std::str;

use self::fnv::FnvHashMap;
use self::ws::util::Token as WsToken;

pub use self::error::{Error, Result};
pub use self::parsing::{ParsedResult, ParsedMessage, ChannelUpdate};
use error::{Error as HttpError, ErrorType as HttpErrorType};

use TokenStorage;
use Token;

mod error;
mod parsing;

/// Handler trait to implement for socket clients.
pub trait Handler {
    /// Run when a disconnect has occurred.
    fn on_disconnect(&mut self) -> ws::Result<()> {
        Ok(())
    }

    /// Run on any websocket error or message parsing error.
    fn on_error(&mut self, err: Error) {
        warn!("screeps socket error uncaught due to default handler method: {}",
              err);
    }

    /// Run on any messages from the server.
    fn on_message(&mut self, msg: parsing::ParsedMessage) -> ws::Result<()>;

    /// Run on any communication from the server.
    ///
    /// Default behavior is to ignore heartbeats, open and close messages, and just send any actual messages
    /// to on_message.
    fn on_communication(&mut self, result: parsing::ParsedResult) -> ws::Result<()> {
        match result {
            ParsedResult::Message(msg) => {
                debug!("screeps socket connection received single SockJS message");
                self.on_message(msg)
            }
            ParsedResult::Messages(messages) => {
                debug!("screeps socket connection received array of {} SockJS messages",
                       messages.len());
                for msg in messages {
                    self.on_message(msg)?
                }
                Ok(())
            }
            ParsedResult::Heartbeat => {
                debug!("screeps socket connection received SockJS heartbeat");
                Ok(())
            }
            ParsedResult::Open => {
                debug!("screeps socket connection received SockJS open");
                Ok(())
            }
            ParsedResult::Close { code, reason } => {
                // TODO: should we pass this on?
                debug!("screeps socket connection received SockJS close ({}, {})",
                       code,
                       reason);
                Ok(())
            }
        }
    }
}

impl<T> Handler for T
    where T: FnMut(parsing::ParsedMessage) -> ws::Result<()>
{
    fn on_message(&mut self, msg: parsing::ParsedMessage) -> ws::Result<()> {
        (self)(msg)
    }
}

enum FailState {
    Login,
}

struct ApiHandler<H: Handler, T: TokenStorage = Option<Token>> {
    token: T,
    handler: H,
    sender: Sender,
    retrying: FnvHashMap<usize, FailState>,
}

impl<H: Handler, T: TokenStorage> ApiHandler<H, T> {
    fn mark_retry(&mut self, failed: FailState, retry_in: Duration) -> ws::Result<()> {
        let mut num = 0usize;
        while self.retrying.contains_key(&num) {
            num += 1;
        }
        self.retrying.insert(num, failed);

        self.sender.sender().timeout((retry_in.subsec_nanos() as f64 / 1.0e6) as u64 + retry_in.as_secs() * 1000,
                                     WsToken(num))
    }

    fn try_or_retry_auth(&mut self) -> ws::Result<()> {
        let token = match self.token.take_token() {
            Some(t) => t,
            None => {
                self.handler.on_error(HttpError::from(HttpErrorType::Unauthorized).into());
                self.mark_retry(FailState::Login, Duration::from_secs(15))?;
                return Ok(());
            }
        };

        self.sender.authenticate(token)
    }

    fn retry_failstate(&mut self, state: FailState) -> ws::Result<()> {
        match state {
            FailState::Login => self.try_or_retry_auth(),
        }
    }
}

impl<H: Handler, T: TokenStorage> ws::Handler for ApiHandler<H, T> {
    fn on_error(&mut self, err: ws::Error) {
        self.handler.on_error(err.into());
    }

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        match msg {
            ws::Message::Text(s) => {
                match parsing::ParsedResult::parse(&s) {
                    Ok(v) => {
                        match v {
                            ParsedResult::Open => {
                                self.try_or_retry_auth()
                                    .map_err(Into::into)
                                    .unwrap_or_else(|x| self.handler.on_error(x))
                            }
                            ParsedResult::Heartbeat => self.sender.sender().send("[]")?,
                            _ => (),
                        }
                        self.handler.on_communication(v)?;
                    }
                    Err(e) => {
                        self.handler.on_error(e.into());
                    }
                }
            }
            ws::Message::Binary(b) => {
                error!("ignoring binary data received from websocket! {:?}", b);
            }
        }
        Ok(())
    }

    fn on_timeout(&mut self, msg: WsToken) -> ws::Result<()> {
        match self.retrying.remove(&msg.0) {
            Some(state) => self.retry_failstate(state)?,
            None => debug!("timeout for token {:?} ignored: token not known.", msg),
        }

        Ok(())
    }
}

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

/// Sender structure wrapping websocket's sender with Screeps API methods.
#[derive(Clone)]
pub struct Sender(ws::Sender);

impl Sender {
    fn authenticate(&self, token: Token) -> ws::Result<()> {
        let message = "auth "
            .chars()
            .chain(token.chars())
            .collect::<String>();

        self.send_raw(&message)
    }

    /// Subscribes to a channel.
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
    pub fn subscribe(&self, channel: Channel) -> ws::Result<()> {
        let message = channel.chain_and_complete_message("subscribe ".chars());

        self.send_raw(&message)
    }

    /// Unsubscribes from a channel. Unsubscribing from a channel you are not subscribed to appears to
    /// have no affect.
    ///
    /// Recommended that you keep track of what channels you are subscribed to separately: this
    /// is tracked by the server, but is not tracked by `screeps-api`, and cannot be queried from the
    /// server.
    pub fn unsubscribe(&self, channel: Channel) -> ws::Result<()> {
        let message = channel.chain_and_complete_message("unsubscribe ".chars());

        self.send_raw(&message)
    }

    /// Sends an empty SockJS frame.
    pub fn send_empty_frame(&self) -> ws::Result<()> {
        let message = "[]";

        debug!("[SockJS emulation] sending empty frame: {:?}", message);

        self.0.send(message)
    }

    /// Sends a raw SockJS frame.
    pub fn send_raw(&self, message: &str) -> ws::Result<()> {
        let encoded = serde_json::to_string(&(message,))
            .expect("serializing a tuple containing a single string can't fail.");

        debug!("[SockJS emulation] sending frame: {:?}", message);

        self.0.send(encoded)
    }

    /// Gets the inner websocket sender.
    #[inline]
    pub fn sender(&self) -> &ws::Sender {
        &self.0
    }
}


/// Method for connecting to a screeps server, mirroring the ws-rs method of the same name.
///
/// This establishes a connection to the default screeps server, `https://screeps.com/api/`. To
/// use a custom url, [`websocket_url`] and [`connect_with_url`] should be used instead.
///
/// Establishes a connection, using the given token storage to authenticate.
///
/// [`websocket_url`]: fn.websocket_url.html
/// [`connect_with_url`]: fn.connect_with_url.html
pub fn connect<F, H, T>(factory: F, token: T) -> Result<()>
    where F: FnMut(Sender) -> H,
          H: Handler,
          T: TokenStorage + Clone
{
    use default_url;
    connect_with_url(&websocket_url(&default_url())?, factory, token)?;

    Ok(())
}

/// Method for connecting to a screeps server, mirroring the ws-rs method of the same name.
///
/// The url should be generated by running [`websocket_url`].`
///
/// Establishes a connection, using the given token storage to authenticate.
///
/// [`websocket_url`]: fn.websocket_url.html
pub fn connect_with_url<U, F, H, T>(websocket_address: &U, mut factory: F, token: T) -> ws::Result<()>
    where U: AsRef<str> + ?Sized,
          F: FnMut(Sender) -> H,
          H: Handler,
          T: TokenStorage + Clone
{
    ws::connect(websocket_address.as_ref(), |ws_sender| {
        let sender = Sender(ws_sender);
        let handler = factory(sender.clone());

        ApiHandler {
            token: token.clone(),
            handler: handler,
            sender: sender,
            retrying: FnvHashMap::default(),
        }
    })
}

/// Method for finding a websocket URL given the screeps API URL.
///
/// This method uses the thread-local `rand` crate rng to come up with a unique
/// session id, and the resulting url should not be reused over multiple connections.
///
/// The input URL should be an API url in the format of `https://screeps.com/api/`.
pub fn websocket_url<U: AsRef<str> + ?Sized>(url: &U) -> Result<url::Url> {
    use std::fmt;

    let mut url = url.as_ref().parse::<url::Url>()?;

    let new_scheme = match url.scheme() {
        "http" => "ws",
        "https" => "wss",
        other => return Err(Error::UnknownScheme(other.to_owned())),
    };

    url.set_scheme(new_scheme).expect("expected `ws` and `wss` to be valid url schemes.");


    // we could probably just use gen_ascii_chars for the session ID, but to be safe
    // we just use the subset that `sockjs-client` does.
    const VALID_CHARS: &'static [u8] = b"abcdefghijklmnopqrstuvwxyz012345";

    // avoiding allocations!
    struct GenServerAndSessionId;

    impl fmt::Display for GenServerAndSessionId {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            use self::rand::Rng;

            let mut rng = rand::thread_rng();
            write!(f, "../socket/{:04}/", rng.gen_range(0, 1000))?;

            for _ in 0..8 {
                write!(f, "{}", *rng.choose(VALID_CHARS).unwrap() as char)?;
            }
            write!(f, "/websocket")?;

            Ok(())
        }
    }

    let result = url.join(&GenServerAndSessionId.to_string())?;

    Ok(result)
}
