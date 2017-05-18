use std::borrow::Cow;
use std::convert::AsRef;
use std::marker::PhantomData;
use std::fmt;

use serde::{Deserializer, Deserialize};
use serde::de::{self, Visitor, SeqAccess};

use serde_json;

pub use self::room_map_view::RoomMapViewUpdate;
pub use self::user_cpu::UserCpuUpdate;
pub use self::user_console::UserConsoleUpdate;

use sockets::Channel;

mod room_map_view;
mod user_cpu;
mod user_console;

/// An update to a Screeps server 'channel' that has been subscribed to.
#[derive(Debug, Clone, PartialEq)]
pub enum ChannelUpdate<'a> {
    /// A 'map view' update of a room. Sent once per tick.
    RoomMapView {
        /// The name of the room this is an update for.
        room_name: Cow<'a, str>,
        /// The data: the positions and nondescript types of entities in this room.
        update: RoomMapViewUpdate,
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
    pub fn room_name(&self) -> Option<&str> {
        match *self {
            ChannelUpdate::RoomMapView { ref room_name, .. } => Some(room_name.as_ref()),
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
            ChannelUpdate::UserCpu { ref user_id, .. } |
            ChannelUpdate::UserConsole { ref user_id, .. } |
            ChannelUpdate::UserCredits { ref user_id, .. } => Some(user_id.as_ref()),
            _ => None,
        }
    }

    /// Gets the channel which this update is from.
    ///
    /// This channel specification can be used to subscribe or unsubscribe from this channel if needed.
    pub fn channel(&self) -> Channel {
        match *self {
            ChannelUpdate::RoomMapView { ref room_name, .. } => Channel::room_map_view(room_name.as_ref()),
            ChannelUpdate::UserCpu { ref user_id, .. } => Channel::user_cpu(user_id.as_ref()),
            ChannelUpdate::UserConsole { ref user_id, .. } => Channel::user_console(user_id.as_ref()),
            ChannelUpdate::UserCredits { ref user_id, .. } => Channel::user_credits(user_id.as_ref()),
            ChannelUpdate::Other { ref channel, .. } => Channel::other(channel.as_ref()),
        }
    }
}

struct ChannelUpdateVisitor<'a> {
    marker: PhantomData<ChannelUpdate<'a>>,
}

impl<'a> ChannelUpdateVisitor<'a> {
    fn new() -> Self {
        ChannelUpdateVisitor { marker: PhantomData }
    }
}

impl<'de> Visitor<'de> for ChannelUpdateVisitor<'de> {
    type Value = ChannelUpdate<'static>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a sequence")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where A: SeqAccess<'de>
    {
        const ROOM_MAP_VIEW_PREFIX: &'static str = "roomMap2:";
        const USER_PREFIX: &'static str = "user:";
        const USER_CPU: &'static str = "cpu";
        const USER_CONSOLE: &'static str = "console";
        const USER_CREDITS: &'static str = "money";

        let channel: &str = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(2, &self))?;

        macro_rules! finish_other {
            () => ({
                return Ok(ChannelUpdate::Other {
                    channel: channel.to_owned().into(),
                    update: seq.next_element()?.ok_or_else(|| de::Error::invalid_length(2, &self))?,
                });
            })
        }

        if channel.starts_with(ROOM_MAP_VIEW_PREFIX) {
            let room_name = &channel[ROOM_MAP_VIEW_PREFIX.len()..];

            return Ok(ChannelUpdate::RoomMapView {
                room_name: room_name.to_owned().into(),
                update: seq.next_element()?.ok_or_else(|| de::Error::invalid_length(2, &self))?,
            });
        } else if channel.starts_with(USER_PREFIX) {
            let user_id_and_part = &channel[USER_PREFIX.len()..];

            let (user_id, sub_channel) = {
                let mut split = user_id_and_part.splitn(2, "/");
                match (split.next(), split.next()) {
                    (Some(v1), Some(v2)) => (v1, v2),
                    _ => finish_other!(),
                }
            };

            match sub_channel {
                USER_CPU => {
                    return Ok(ChannelUpdate::UserCpu {
                        user_id: user_id.to_owned().into(),
                        update: seq.next_element()?.ok_or_else(|| de::Error::invalid_length(2, &self))?,
                    });
                }
                USER_CONSOLE => {
                    return Ok(ChannelUpdate::UserConsole {
                        user_id: user_id.to_owned().into(),
                        update: seq.next_element()?.ok_or_else(|| de::Error::invalid_length(2, &self))?,
                    });
                }
                USER_CREDITS => {
                    return Ok(ChannelUpdate::UserCredits {
                        user_id: user_id.to_owned().into(),
                        update: seq.next_element()?.ok_or_else(|| de::Error::invalid_length(2, &self))?,
                    })
                }
                _ => finish_other!(),
            }
        }

        finish_other!();
    }
}

impl<'de> Deserialize<'de> for ChannelUpdate<'static> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        deserializer.deserialize_seq(ChannelUpdateVisitor::new())
    }
}
