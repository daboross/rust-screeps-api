//! Data structures that appear in multiple API endpoint results.
mod errors;
mod room_name;
mod rooms;
mod users;

pub use self::errors::*;
pub use self::room_name::*;
pub use self::rooms::*;
pub use self::users::*;
