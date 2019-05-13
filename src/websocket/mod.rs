//! Handling of socket connections to screeps using ws-rs as a backend.
pub mod channel;
pub mod commands;
pub mod connecting;
pub mod parsing;
pub mod types;

pub use self::{
    channel::Channel,
    commands::{authenticate, subscribe, unsubscribe},
    connecting::{default_url, transform_url},
    parsing::{ScreepsMessage, SockjsMessage},
    types::{
        messages::{ConversationUpdate, Message, MessageUnreadUpdate, MessageUpdate},
        room::{RoomUpdate, RoomUpdateInfo, RoomUpdateUserInfo},
        room_map_view::RoomMapViewUpdate,
        user_console::UserConsoleUpdate,
        user_cpu::UserCpuUpdate,
        ChannelUpdate,
    },
};
