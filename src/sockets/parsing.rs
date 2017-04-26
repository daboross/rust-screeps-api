use std::borrow::Cow;
use std::convert::AsRef;

use serde_json;

use super::error::ParseError;
use Token;

/// Result of parsing a message
#[derive(Debug, Clone, PartialEq)]
pub enum ParsedResult {
    /// "Open"?
    Open,
    /// Heartbeat
    Heartbeat,
    /// Close
    Close {
        /// Close code
        code: i64,
        /// Close reason
        reason: String,
    },
    /// Single message
    Message(ParsedMessage),
    /// Multiple messages
    Messages(Vec<ParsedMessage>),
}

impl ParsedResult {
    /// Parses an incoming raw websockets messages on a Screeps SockJS socket into some result.
    pub fn parse<'a, T: Into<Cow<'a, str>>>(message: T) -> Result<Self, ParseError> {
        let full_message_cow = message.into();


        let message = full_message_cow.as_ref();

        let first = match message.chars().next() {
            // empty string
            None => return Ok(ParsedResult::Messages(Vec::new())),
            Some(c) => c,
        };

        let parsed = match first {
            // TODO: should we check length for Open and Heartbeat messages?
            'o' => ParsedResult::Open,
            'h' => ParsedResult::Heartbeat,
            'c' => {
                let rest = &message[1..];
                match serde_json::from_str(rest) {
                    Ok((code, reason)) => {
                        ParsedResult::Close {
                            code: code,
                            reason: reason,
                        }
                    }
                    Err(e) => return Err(ParseError::serde("error parsing closed json message", rest.to_owned(), e)),
                }
            }
            'm' => {
                let rest = &message[1..];
                // SockJS _might_ allow providing non-String json values here, but I _think_ the server only ever
                // sends strings.

                // TODO: this shouldn't allocate a new string here.
                match serde_json::from_str::<String>(rest) {
                    Ok(message) => ParsedResult::Message(ParsedMessage::parse(message)?),
                    Err(e) => return Err(ParseError::serde("error parsing single message", rest.to_owned(), e)),
                }
            }
            'a' => {
                let rest = &message[1..];

                // TODO: this shouldn't allocate new strings here.
                match serde_json::from_str::<Vec<String>>(rest) {
                    Ok(messages) => {
                        ParsedResult::Messages(messages.into_iter()
                            .map(ParsedMessage::parse)
                            .collect::<Result<Vec<ParsedMessage>, ParseError>>()?)
                    }
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


/// A parsed message.
#[derive(Debug, Clone, PartialEq)]
pub enum ParsedMessage {
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
        /// The package version? I'm not sure.
        package: u32,
    },
    /// An update on one of the channels.
    ChannelUpdate {
        /// The channel name. TODO: parse this into a Channel.
        channel: String,
        /// The message value. TODO: parse into per-channel types.
        message: serde_json::Value,
    },
    /// Another kind of message.
    Other(String),
}


const AUTH_PREFIX: &'static str = "auth ";
const TIME_PREFIX: &'static str = "time ";
const PROTOCOL_PREFIX: &'static str = "protocol ";
const PACKAGE_PREFIX: &'static str = "package ";
const AUTH_OK: &'static str = "ok ";
const AUTH_FAILED: &'static str = "failed";


impl ParsedMessage {
    /// Parses the internal message from a SockJS message into a meaningful type.
    pub fn parse<'a, T: Into<Cow<'a, str>>>(message: T) -> Result<Self, ParseError> {
        // TODO: deflate with base64 then zlib if the message starts with "gz:".
        let full_message_cow = message.into();

        {
            let full_message = full_message_cow.as_ref();

            if full_message.starts_with(AUTH_PREFIX) {
                let rest = &full_message[AUTH_PREFIX.len()..];

                return Ok({
                    if rest.starts_with(AUTH_OK) {
                        ParsedMessage::AuthOk { new_token: rest[AUTH_OK.len()..].to_owned() }
                    } else if rest == AUTH_FAILED {
                        ParsedMessage::AuthFailed
                    } else {
                        warn!("expected \"auth failed\", found \"{}\" (occurred when parsing authentication failure)",
                              full_message);
                        ParsedMessage::AuthFailed
                    }
                });
            } else if full_message.starts_with(TIME_PREFIX) {
                let rest = &full_message[TIME_PREFIX.len()..];

                match rest.parse::<u64>() {
                    Ok(v) => return Ok(ParsedMessage::ServerTime { time: v }),
                    Err(_) => {
                        warn!("expected \"time <integer>\", found \"{}\". Ignoring inconsistent message!",
                              rest);
                    }
                }
            } else if full_message.starts_with(PROTOCOL_PREFIX) {
                let rest = &full_message[PROTOCOL_PREFIX.len()..];

                match rest.parse::<u32>() {
                    Ok(v) => return Ok(ParsedMessage::ServerProtocol { protocol: v }),
                    Err(_) => {
                        warn!("expected \"protocol <integer>\", found \"{}\". Ignoring inconsistent message!",
                              rest);
                    }
                }
            } else if full_message.starts_with(PACKAGE_PREFIX) {
                let rest = &full_message[PACKAGE_PREFIX.len()..];

                match rest.parse::<u32>() {
                    Ok(v) => return Ok(ParsedMessage::ServerPackage { package: v }),
                    Err(_) => {
                        warn!("expected \"package <integer>\", found \"{}\". Ignoring inconsistent message!",
                              rest);
                    }
                }
            }

            if let Ok((channel, message)) = serde_json::from_str(full_message) {
                return Ok(ParsedMessage::ChannelUpdate {
                    channel: channel,
                    message: message,
                });
            }
        }

        // If it isn't in the exact format we expect, treat it as "other"
        // (TODO: error there instead once we are confident in this)
        Ok(ParsedMessage::Other(full_message_cow.into_owned()))
    }
}
