//! Data structures that appear in multiple API endpoint results.
pub mod errors;
pub mod room_name;
pub mod rooms;
pub mod users;

pub use self::errors::ApiError;
pub use self::room_name::RoomName;
pub use self::rooms::*;
pub use self::users::Badge;
