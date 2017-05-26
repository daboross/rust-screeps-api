//! Data structures that appear in multiple API endpoint results.
pub mod errors;
pub mod rooms;
pub mod users;
pub mod room_name;

pub use self::errors::ApiError;
pub use self::users::Badge;
pub use self::rooms::*;
pub use self::room_name::RoomName;
