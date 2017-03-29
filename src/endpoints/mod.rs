//! Parsing code for each individual API endpoint.
//!
//! Each sub-module contains code for interpreting the result of calling a specific API endpoint.
pub mod login;
pub mod map_stats;
pub mod my_info;
pub mod room_overview;
pub mod room_status;
pub mod room_terrain;
pub mod recent_pvp;
pub mod leaderboard;

// don't compile this endpoint template file with regular output, but still compile w/ tests to test for correctness.
#[cfg(test)]
pub mod template;

pub use my_info::MyInfo;
pub use recent_pvp::RecentPvp;
pub use room_overview::RoomOverview;
pub use room_status::RoomStatus;
pub use room_terrain::RoomTerrain;
pub use map_stats::MapStats;
