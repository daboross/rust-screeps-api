//! Handling of socket connections to screeps using ws-rs as a backend.
pub mod parsing;
pub mod channel;
pub mod commands;
pub mod connecting;
pub mod types;

pub use self::channel::Channel;
pub use self::commands::{authenticate, subscribe, unsubscribe};
pub use self::connecting::{default_url, transform_url};

pub use self::parsing::{ScreepsMessage, SockjsMessage};

pub use self::types::ChannelUpdate;
pub use self::types::messages::{ConversationUpdate, Message, MessageUnreadUpdate, MessageUpdate};
pub use self::types::room::{RoomUpdate, RoomUpdateInfo, RoomUpdateUserInfo};
pub use self::types::room_map_view::RoomMapViewUpdate;
pub use self::types::user_console::UserConsoleUpdate;
pub use self::types::user_cpu::UserCpuUpdate;
