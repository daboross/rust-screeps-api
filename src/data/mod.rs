//! Data structures that appear in multiple API endpoint results.
pub mod errors;
pub mod rooms;
pub mod users;

pub use self::errors::ApiError;
pub use self::rooms::{StringNumberTimeSpec, RoomState};
pub use self::users::Badge;
