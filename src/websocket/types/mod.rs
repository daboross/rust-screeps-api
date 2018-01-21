//! Parsing inner Screeps websocket update messages.
use std::borrow::Cow;
use std::convert::AsRef;
use std::marker::PhantomData;
use std::fmt;

use serde::{Deserialize, Deserializer};
use serde::de::{self, SeqAccess, Unexpected, Visitor};

use serde_json;
use websocket::Channel;

use RoomName;

use self::room_map_view::RoomMapViewUpdate;
use self::user_cpu::UserCpuUpdate;
use self::user_console::UserConsoleUpdate;
use self::room::RoomUpdate;
use self::messages::{ConversationUpdate, MessageUpdate};

pub mod room;
pub mod messages;
pub mod room_map_view;
pub mod user_cpu;
pub mod user_console;

/// An update to a Screeps server 'channel' that has been subscribed to.
#[derive(Clone, Debug)]
pub enum ChannelUpdate<'a> {
    /// A 'map view' update of a room. Sent once per tick.
    RoomMapView {
        /// The name of the room this is an update for.
        room_name: RoomName,
        /// The shard the room is in, if any.
        shard_name: Option<String>,
        /// The data: the positions and nondescript types of entities in this room.
        update: RoomMapViewUpdate,
    },
    /// An update of objects in a room. Sent once per tick for up to 2 rooms per account.
    /// Other subscribed rooms receive `ChannelUpdate::NoRoomDetail` instead.
    RoomDetail {
        /// The name of the room this is an update for.
        room_name: RoomName,
        /// The shard the room is in, if any.
        shard_name: Option<String>,
        /// The data: all properties of all objects in this room that have changed since the last tick.
        update: RoomUpdate,
    },
    /// An update/error received for room detail subscriptions which will not receive an update this tick.
    ///
    /// This is due to screeps having a 2-room subscription limit. If more are subscribed globally (per-account,
    /// not per-connection) then only 2 receive actual updates.
    ///
    /// TODO: This should have a better name, and possibly be incorporated into `ChannelUpdate::RoomDetail`.
    NoRoomDetail {
        /// The name of the room this is a notification for.
        room_name: RoomName,
        /// The shard the room is in, if any.
        shard_name: Option<String>,
    },
    /// An update on the last tick's CPU and memory usage. Sent once per tick.
    UserCpu {
        /// The user ID this is a cpu/memory update for.
        user_id: Cow<'a, str>,
        /// The update.
        update: UserCpuUpdate,
    },
    /// An update on all user script log messages last tick or a specific error message.
    /// Sent once and exactly once per tick unless:
    ///
    /// - Multiple script errors occurred.
    /// - Normal log messages were sent and a script error also occurred.
    ///
    /// In either of these cases, two or more of these updates will occur in short succession.
    UserConsole {
        /// The user ID this console update is for.
        user_id: Cow<'a, str>,
        /// The update.
        update: UserConsoleUpdate,
    },
    /// An update on the user's credit total at the end of the last tick. Sent once per tick.
    UserCredits {
        /// The user ID this credit update is for.
        user_id: Cow<'a, str>,
        /// The number of credits.
        update: f64,
    },
    /// An update on a new message received by a user. Sent each time a user receives a message.
    UserMessage {
        /// The user ID this message update is for.
        user_id: Cow<'a, str>,
        /// The message update.
        update: MessageUpdate,
    },
    /// An update on a change in a conversation the user is participating in. Sent each time either
    /// this user or the user or the respondent either sends a message, or reads a previously unread
    /// message.
    UserConversation {
        /// The user ID of the subscribed user this update is for.
        user_id: Cow<'a, str>,
        /// The user ID of the other user in the conversation.
        target_user_id: Cow<'a, str>,
        /// The message update.
        update: ConversationUpdate,
    },
    /// Another update that was not accounted for.
    ///
    /// TODO: when we're sure of everything, remove this variant.
    Other {
        /// The string describing what channel this is.
        channel: Cow<'a, str>,
        /// The update.
        update: serde_json::Value,
    },
}

impl<'a> ChannelUpdate<'a> {
    /// If this update is directly associated with a room, gets the room name.
    pub fn shard_name(&self) -> Option<&str> {
        match *self {
            ChannelUpdate::RoomMapView { ref shard_name, .. }
            | ChannelUpdate::RoomDetail { ref shard_name, .. }
            | ChannelUpdate::NoRoomDetail { ref shard_name, .. } => shard_name.as_ref().map(AsRef::as_ref),
            _ => None,
        }
    }
    /// If this update is directly associated with a room, gets the room name.
    pub fn room_name(&self) -> Option<&RoomName> {
        match *self {
            ChannelUpdate::RoomMapView { ref room_name, .. }
            | ChannelUpdate::RoomDetail { ref room_name, .. }
            | ChannelUpdate::NoRoomDetail { ref room_name, .. } => Some(room_name),
            _ => None,
        }
    }

    /// If this update is directly associated with a subscribed user id, gets the user id.
    ///
    /// The user_id is *always* the user id of the subscribed user, never another associated id.
    ///
    /// For example, with 'message' updates, this is the user ID of the user receiving the
    /// notification, *not* the user id of the sender.
    pub fn user_id(&self) -> Option<&str> {
        match *self {
            ChannelUpdate::UserCpu { ref user_id, .. }
            | ChannelUpdate::UserConsole { ref user_id, .. }
            | ChannelUpdate::UserMessage { ref user_id, .. }
            | ChannelUpdate::UserConversation { ref user_id, .. }
            | ChannelUpdate::UserCredits { ref user_id, .. } => Some(user_id.as_ref()),
            _ => None,
        }
    }

    /// Gets the channel which this update is from.
    ///
    /// This channel specification can be used to subscribe or unsubscribe from this channel if needed.
    pub fn channel(&self) -> Channel {
        match *self {
            ChannelUpdate::RoomMapView {
                room_name,
                ref shard_name,
                ..
            } => Channel::room_map_view(room_name, shard_name.as_ref().map(AsRef::as_ref)),
            ChannelUpdate::RoomDetail {
                room_name,
                ref shard_name,
                ..
            }
            | ChannelUpdate::NoRoomDetail {
                room_name,
                ref shard_name,
                ..
            } => Channel::room_detail(room_name, shard_name.as_ref().map(AsRef::as_ref)),
            ChannelUpdate::UserCpu { ref user_id, .. } => Channel::user_cpu(user_id.as_ref()),
            ChannelUpdate::UserConsole { ref user_id, .. } => Channel::user_console(user_id.as_ref()),
            ChannelUpdate::UserCredits { ref user_id, .. } => Channel::user_credits(user_id.as_ref()),
            ChannelUpdate::UserMessage { ref user_id, .. } => Channel::user_messages(user_id.as_ref()),
            ChannelUpdate::UserConversation {
                ref user_id,
                ref target_user_id,
                ..
            } => Channel::user_conversation(user_id.as_ref(), target_user_id.as_ref()),
            ChannelUpdate::Other { ref channel, .. } => Channel::other(channel.as_ref()),
        }
    }
}

struct ChannelUpdateVisitor<'a> {
    marker: PhantomData<ChannelUpdate<'a>>,
}

impl<'a> ChannelUpdateVisitor<'a> {
    fn new() -> Self {
        ChannelUpdateVisitor {
            marker: PhantomData,
        }
    }
}

impl<'de> Visitor<'de> for ChannelUpdateVisitor<'de> {
    type Value = ChannelUpdate<'static>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a sequence")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        const ROOM_MAP_VIEW_PREFIX: &str = "roomMap2:";
        const ROOM_PREFIX: &str = "room:";
        const ROOM_ERR_PREFIX: &str = "err@room:"; // TODO: generic error handling with this `err@` format.
        const USER_PREFIX: &str = "user:";
        const USER_CPU: &str = "cpu";
        const USER_CONSOLE: &str = "console";
        const USER_CREDITS: &str = "money";
        const USER_MESSAGE: &str = "newMessage";
        const USER_CONVERSATION_PREFIX: &str = "message:";

        let channel: &str = seq.next_element()?
            .ok_or_else(|| de::Error::invalid_length(2, &self))?;

        macro_rules! finish_other {
            () => ({
                return Ok(ChannelUpdate::Other {
                    channel: channel.to_owned().into(),
                    update: seq.next_element()?.ok_or_else(|| de::Error::invalid_length(2, &self))?,
                });
            })
        }

        if channel.starts_with(ROOM_MAP_VIEW_PREFIX) {
            let room_name_and_shard = &channel[ROOM_MAP_VIEW_PREFIX.len()..];

            let (shard_name, room_name) = {
                let mut split = room_name_and_shard.splitn(2, "/");
                match (split.next(), split.next()) {
                    (Some(shard), Some(room)) => (Some(shard), room),
                    (Some(room), None) => (None, room),
                    _ => finish_other!(),
                }
            };
            let room_name = RoomName::new(room_name).map_err(|_| {
                de::Error::invalid_value(
                    Unexpected::Str(room_name),
                    &"room name formatted `(E|W)[0-9]+(N|S)[0-9]+`",
                )
            })?;

            return Ok(ChannelUpdate::RoomMapView {
                room_name: room_name,
                shard_name: shard_name.map(ToOwned::to_owned),
                update: seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?,
            });
        } else if channel.starts_with(ROOM_PREFIX) {
            let room_name_and_shard = &channel[ROOM_PREFIX.len()..];

            let (shard_name, room_name) = {
                let mut split = room_name_and_shard.splitn(2, '/');
                match (split.next(), split.next()) {
                    (Some(shard), Some(room)) => (Some(shard), room),
                    (Some(room), None) => (None, room),
                    _ => finish_other!(),
                }
            };

            let room_name = RoomName::new(room_name).map_err(|_| {
                de::Error::invalid_value(
                    Unexpected::Str(room_name),
                    &"room name formatted `(E|W)[0-9]+(N|S)[0-9]+`",
                )
            })?;

            return Ok(ChannelUpdate::RoomDetail {
                room_name: room_name,
                shard_name: shard_name.map(ToOwned::to_owned),
                update: seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?,
            });
        } else if channel.starts_with(ROOM_ERR_PREFIX) {
            let room_name_and_shard = &channel[ROOM_ERR_PREFIX.len()..];

            let (shard_name, room_name) = {
                let mut split = room_name_and_shard.splitn(2, '/');
                match (split.next(), split.next()) {
                    (Some(shard), Some(room)) => (Some(shard), room),
                    (Some(room), None) => (None, room),
                    _ => finish_other!(),
                }
            };

            let room_name = RoomName::new(room_name).map_err(|_| {
                de::Error::invalid_value(
                    Unexpected::Str(room_name),
                    &"room name formatted `(E|W)[0-9]+(N|S)[0-9]+`",
                )
            })?;

            let err_message = seq.next_element::<&str>()?
                .ok_or_else(|| de::Error::invalid_length(2, &self))?;

            // TODO: This is currently just a patch in for a common error message, but we don't handle any other
            // errors that are reported as `err@<rest of channel name>`.
            //
            // We should:
            // A. find out if the server actually ever sends other error messages, or if this is hardcoded on the
            //    other side too.
            // B. add handling of all err@ messages into a variant which can then just parse the channel name into
            //    a `Channel`.
            if err_message == "subscribe limit reached" {
                return Ok(ChannelUpdate::NoRoomDetail {
                    room_name: room_name,
                    shard_name: shard_name.map(ToOwned::to_owned),
                });
            }
        } else if channel.starts_with(USER_PREFIX) {
            let user_id_and_part = &channel[USER_PREFIX.len()..];

            let (user_id, sub_channel) = {
                let mut split = user_id_and_part.splitn(2, '/');
                match (split.next(), split.next()) {
                    (Some(v1), Some(v2)) => (v1, v2),
                    _ => finish_other!(),
                }
            };

            match sub_channel {
                USER_CPU => {
                    return Ok(ChannelUpdate::UserCpu {
                        user_id: user_id.to_owned().into(),
                        update: seq.next_element()?
                            .ok_or_else(|| de::Error::invalid_length(2, &self))?,
                    });
                }
                USER_CONSOLE => {
                    return Ok(ChannelUpdate::UserConsole {
                        user_id: user_id.to_owned().into(),
                        update: seq.next_element()?
                            .ok_or_else(|| de::Error::invalid_length(2, &self))?,
                    });
                }
                USER_CREDITS => {
                    return Ok(ChannelUpdate::UserCredits {
                        user_id: user_id.to_owned().into(),
                        update: seq.next_element()?
                            .ok_or_else(|| de::Error::invalid_length(2, &self))?,
                    })
                }
                USER_MESSAGE => {
                    return Ok(ChannelUpdate::UserMessage {
                        user_id: user_id.to_owned().into(),
                        update: seq.next_element()?
                            .ok_or_else(|| de::Error::invalid_length(2, &self))?,
                    })
                }
                sub_channel => if sub_channel.starts_with(USER_CONVERSATION_PREFIX) {
                    let target_user_id = &sub_channel[USER_CONVERSATION_PREFIX.len()..];

                    return Ok(ChannelUpdate::UserConversation {
                        user_id: user_id.to_owned().into(),
                        target_user_id: target_user_id.to_owned().into(),
                        update: seq.next_element()?
                            .ok_or_else(|| de::Error::invalid_length(2, &self))?,
                    });
                } else {
                    finish_other!()
                },
            }
        }

        finish_other!();
    }
}

impl<'de> Deserialize<'de> for ChannelUpdate<'static> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(ChannelUpdateVisitor::new())
    }
}
