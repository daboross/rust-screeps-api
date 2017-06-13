//! Parsing messages from Screeps websockets.
use std::borrow::Cow;
use std::convert::AsRef;
use std::marker::PhantomData;
use std::{cmp, fmt};

use serde::{Deserializer, Deserialize};
use serde::de::{Visitor, SeqAccess};

use {serde_json, serde_ignored};

use Token;

use websocket::types::ChannelUpdate;

mod error;

pub use self::error::ParseError;

fn from_str_with_warning<'de, T>(input: &'de str, context: &str) -> Result<T, serde_json::Error>
    where T: Deserialize<'de>
{
    let mut deserializer = serde_json::Deserializer::new(serde_json::de::StrRead::new(input));

    let value = serde_ignored::deserialize(&mut deserializer, |path| {
        warn!("unparsed data in {}: {}", context, path);
    })?;

    deserializer.end()?;

    Ok(value)
}

/// Result of parsing a raw message.
#[derive(Clone, Debug)]
pub enum SockjsMessage<'a> {
    /// "Open"?
    Open,
    /// Heartbeat
    Heartbeat,
    /// Close
    Close {
        /// Close code
        code: i64,
        /// Close reason
        reason: Cow<'a, str>,
    },
    /// Single message
    Message(ScreepsMessage<'a>),
    /// Multiple messages
    Messages(Vec<ScreepsMessage<'a>>),
}

impl<'a> SockjsMessage<'a> {
    /// Parses an incoming raw websockets messages on a Screeps SockJS socket into some result.
    pub fn parse<T: AsRef<str> + ?Sized>(message_generic: &'a T) -> Result<Self, ParseError> {
        let message = message_generic.as_ref();

        let first = match message.chars().next() {
            // empty string
            None => return Ok(SockjsMessage::Messages(Vec::new())),
            Some(c) => c,
        };

        let parsed = match first {
            // TODO: should we check length for Open and Heartbeat messages?
            'o' => SockjsMessage::Open,
            'h' => SockjsMessage::Heartbeat,
            'c' => {
                let rest = &message[1..];
                match serde_json::from_str::<(i64, &str)>(rest) {
                    Ok((code, reason)) => {
                        SockjsMessage::Close {
                            code: code,
                            reason: reason.into(),
                        }
                    }
                    Err(e) => return Err(ParseError::serde("error parsing closed json message", rest.to_owned(), e)),
                }
            }
            'm' => {
                let rest = &message[1..];
                // SockJS _might_ allow providing non-String json values here, but the server has only ever sent
                // strings so far.

                // We have to parse into `String` since it contains json escapes.
                match serde_json::from_str::<String>(rest) {
                    Ok(message) => SockjsMessage::Message(ScreepsMessage::parse(&message)),
                    Err(e) => return Err(ParseError::serde("error parsing single message", rest.to_owned(), e)),
                }
            }
            'a' => {
                let rest = &message[1..];

                match from_str_with_warning::<MultipleMessagesIntermediate>(rest, "set of screeps update messages") {
                    Ok(messages) => SockjsMessage::Messages(messages.0),
                    Err(e) => return Err(ParseError::serde("error parsing array of messages", rest.to_owned(), e)),
                }
            }
            other => {
                return Err(ParseError::Other(format!("Error parsing message, unknown start character: {} (full \
                                                      message: {})",
                                                     other,
                                                     message)))
            }
        };

        Ok(parsed)
    }
}

struct MultipleMessagesIntermediate(Vec<ScreepsMessage<'static>>);

struct MultipleMessagesVisitor {
    marker: PhantomData<MultipleMessagesIntermediate>,
}

impl MultipleMessagesVisitor {
    fn new() -> Self {
        MultipleMessagesVisitor { marker: PhantomData }
    }
}

impl<'de> Visitor<'de> for MultipleMessagesVisitor {
    type Value = MultipleMessagesIntermediate;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a sequence")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where A: SeqAccess<'de>
    {

        let mut values = Vec::with_capacity(cmp::min(seq.size_hint().unwrap_or(0), 4069));

        while let Some(string) = seq.next_element::<String>()? {
            values.push(ScreepsMessage::parse(&string));
        }

        Ok(MultipleMessagesIntermediate(values))
    }
}

impl<'de> Deserialize<'de> for MultipleMessagesIntermediate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        deserializer.deserialize_seq(MultipleMessagesVisitor::new())
    }
}

/// A parsed message.
#[derive(Clone, Debug)]
pub enum ScreepsMessage<'a> {
    /// Authentication failed.
    AuthFailed,
    /// Authentication successful!
    AuthOk {
        /// The new token to store.
        new_token: Token,
    },
    /// On initial connection, the server reports its own time.
    ServerTime {
        /// The server time.
        time: u64,
    },
    /// On initial connection, the server reports a protocol version.
    ServerProtocol {
        /// The protocol version.
        protocol: u32,
    },
    /// On initial connection, the server reports a "package" number.
    ServerPackage {
        /// I'm not sure what this means at all.
        package: u32,
    },
    /// An update on one of the channels.
    ChannelUpdate {
        /// The update.
        update: ChannelUpdate<'a>,
    },
    /// Another kind of message.
    Other(Cow<'a, str>),
}


const AUTH_PREFIX: &'static str = "auth ";
const TIME_PREFIX: &'static str = "time ";
const PROTOCOL_PREFIX: &'static str = "protocol ";
const PACKAGE_PREFIX: &'static str = "package ";
const AUTH_OK: &'static str = "ok ";
const AUTH_FAILED: &'static str = "failed";


impl ScreepsMessage<'static> {
    /// Parses the internal message from a SockJS message into a meaningful type.
    pub fn parse<T: AsRef<str> + ?Sized>(message: &T) -> Self {
        // TODO: deflate with base64 then zlib if the message starts with "gz:".

        {
            let message = message.as_ref();

            if message.starts_with(AUTH_PREFIX) {
                let rest = &message[AUTH_PREFIX.len()..];

                return {
                    if rest.starts_with(AUTH_OK) {
                        ScreepsMessage::AuthOk { new_token: rest[AUTH_OK.len()..].to_owned() }
                    } else if rest == AUTH_FAILED {
                        ScreepsMessage::AuthFailed
                    } else {
                        warn!("expected \"auth failed\", found \"{}\" (occurred when parsing authentication failure)",
                              message);
                        ScreepsMessage::AuthFailed
                    }
                };
            } else if message.starts_with(TIME_PREFIX) {
                let rest = &message[TIME_PREFIX.len()..];

                match rest.parse::<u64>() {
                    Ok(v) => return ScreepsMessage::ServerTime { time: v },
                    Err(_) => {
                        warn!("expected \"time <integer>\", found \"{}\". Ignoring inconsistent message!",
                              rest);
                    }
                }
            } else if message.starts_with(PROTOCOL_PREFIX) {
                let rest = &message[PROTOCOL_PREFIX.len()..];

                match rest.parse::<u32>() {
                    Ok(v) => return ScreepsMessage::ServerProtocol { protocol: v },
                    Err(_) => {
                        warn!("expected \"protocol <integer>\", found \"{}\". Ignoring inconsistent message!",
                              rest);
                    }
                }
            } else if message.starts_with(PACKAGE_PREFIX) {
                let rest = &message[PACKAGE_PREFIX.len()..];

                match rest.parse::<u32>() {
                    Ok(v) => return ScreepsMessage::ServerPackage { package: v },
                    Err(_) => {
                        warn!("expected \"package <integer>\", found \"{}\". Ignoring inconsistent message!",
                              rest);
                    }
                }
            }

            match from_str_with_warning(message, "screeps typed channel update") {
                Ok(update) => return ScreepsMessage::ChannelUpdate { update: update },
                // let failures just result in an 'other' message.
                Err(e) => warn!("error parsing update message: {}", e),
            }
        }

        // If it isn't in the exact format we expect, treat it as "other"
        // (TODO: error there instead once we are confident in this)
        ScreepsMessage::Other(message.as_ref().to_owned().into())
    }
}
