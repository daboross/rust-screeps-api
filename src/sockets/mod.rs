//! Handling of socket connections to screeps using ws-rs as a backend.
use std::rc::Rc;
use std::time::Duration;
use std::borrow::{Borrow, Cow};

use ws;
use fnv::FnvHashMap;
use ws::util::Token as WsToken;

pub use self::error::{Error, Result};
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

    /// Run on any communication from the server.
    ///
    /// TODO: deal with non-ParsedMessage ParsedResults internally.
    fn on_message(&mut self, msg: parsing::ParsedResult) -> ws::Result<()>;
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
    fn on_open(&mut self, _handshake: ws::Handshake) -> ws::Result<()> {
        self.try_or_retry_auth()
    }

    fn on_error(&mut self, err: ws::Error) {
        self.handler.on_error(err.into());
    }

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        match msg {
            ws::Message::Text(s) => {
                match parsing::ParsedResult::parse(s) {
                    Ok(v) => {
                        self.handler.on_message(v)?;
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
// TODO: roomMap2
pub enum Channel<'a> {
    /// Server messages (TODO: find message here).
    ServerMessages,
    /// User CPU and memory usage, updates each tick.
    UserCpu {
        /// The user ID of the subscription.
        user_id: Cow<'a, str>,
    },
    /// User message alerts, updates whenever a message is received.
    UserMessages {
        /// The user ID of the subscription.
        user_id: Cow<'a, str>,
    },
    /// User credit count when it changes.
    UserCredits {
        /// The user ID of the subscription.
        user_id: Cow<'a, str>,
    },
    /// Any changes to a specific path in memory.
    UserMemoryPath {
        /// The user ID of the subscription.
        user_id: Cow<'a, str>,
        /// The memory path, separated with '.'.
        path: Cow<'a, str>,
    },
    /// Any console log messages.
    UserConsole {
        /// The user ID of the subscription.
        user_id: Cow<'a, str>,
    },
    /// Small room tile view for map viewing.
    MapRoomUpdates {
        /// The room name of the subscription.
        room_name: Cow<'a, str>,
    },
    /// Updates for all entities in a room.
    RoomUpdates {
        /// The room name of the subscription.
        room_name: Cow<'a, str>,
    },
}

impl<'a> Channel<'a> {
    /// This is a really wonky scheme, but it is probably the best one right now.
    ///
    /// Adds the channel description to the message (does not add preceding space) and collects to a vec.
    fn chain_and_complete_message<T: Iterator<Item = u8>>(&self, start: T) -> Vec<u8> {
        match *self {
            Channel::ServerMessages => start.chain("server-messages".bytes()).collect(),
            Channel::UserCpu { ref user_id } => {
                start.chain("user:".bytes()).chain(user_id.as_ref().bytes()).chain("/cpu".bytes()).collect()
            }
            Channel::UserMessages { ref user_id } => {
                start.chain("user:".bytes()).chain(user_id.as_ref().bytes()).chain("/newMessages".bytes()).collect()
            }
            Channel::UserCredits { ref user_id } => {
                start.chain("user:".bytes()).chain(user_id.as_ref().bytes()).chain("/money".bytes()).collect()
            }
            Channel::UserMemoryPath { ref user_id, ref path } => {
                start.chain("user:".bytes())
                    .chain(user_id.as_ref().bytes())
                    .chain("/memory/".bytes())
                    .chain(path.as_ref().bytes())
                    .collect()
            }
            Channel::UserConsole { ref user_id } => {
                start.chain("user:".bytes()).chain(user_id.as_ref().bytes()).chain("/console".bytes()).collect()
            }
            Channel::MapRoomUpdates { ref room_name } => {
                start.chain("roomMap2:".bytes()).chain(room_name.as_ref().bytes()).collect()
            }
            Channel::RoomUpdates { ref room_name } => {
                start.chain("room:".bytes()).chain(room_name.as_ref().bytes()).collect()
            }
        }
    }

    /// Allocates a vec with the byte representation of this channel.
    pub fn to_vec(&self) -> Vec<u8> {
        self.chain_and_complete_message("".bytes())
    }
}

/// Sender structure wrapping websocket's sender with Screeps API methods.
///
/// This contains an Rc inside, so Clone will just clone the inner Rc to provide another reference.
#[derive(Clone)]
pub struct Sender(Rc<ws::Sender>);

impl Sender {
    fn authenticate(&mut self, token: Token) -> ws::Result<()> {
        let message = "auth "
            .bytes()
            .chain(token.into_iter())
            .collect::<Vec<_>>();

        self.0.send(message)
    }

    /// Subscribes to a channel. Unknown effect if already subscribed, server error?
    ///
    /// Recommended that you keep track of what channels you are subscribed to separately.
    pub fn subscribe(&mut self, channel: Channel) -> ws::Result<()> {
        let message = channel.chain_and_complete_message("subscribe ".bytes());

        self.0.send(message)
    }
    /// Unsubscribes from a channel. Unknown effect if not subscribed, server error?
    ///
    /// Recommended that you keep track of what channels you are subscribed to separately.
    pub fn unsubscribe(&mut self, channel: Channel) -> ws::Result<()> {
        let message = channel.chain_and_complete_message("unsubscribe ".bytes());

        self.0.send(message)
    }

    /// Gets the inner websocket sender.
    #[inline]
    pub fn sender(&self) -> &ws::Sender {
        &self.0
    }
}

// Send: auth <token>
// Recv: auth ok <new token>
// Send: gzip on
// Possibilities:
//  Send: subscribe room:E15N52
//  Send: .

/// Method for connecting to a screeps server, mirroring the ws-rs method of the same name.
///
/// Establishes a connection, using the given token storage to authenticate.
pub fn connect<U, F, H, T>(websocket_address: U, mut factory: F, token: T) -> ws::Result<()>
    where U: Borrow<str>,
          F: FnMut(Sender) -> H,
          H: Handler,
          T: TokenStorage + Clone
{
    ws::connect(websocket_address, |ws_sender| {
        let sender = Sender(Rc::new(ws_sender));
        let handler = factory(sender.clone());

        ApiHandler {
            token: token.clone(),
            handler: handler,
            sender: sender,
            retrying: FnvHashMap::default(),
        }
    })
}
