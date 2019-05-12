//! Parsing code for each individual API endpoint.
//!
//! Each sub-module contains code for interpreting the result of calling a specific API endpoint.
pub mod leaderboard;
pub mod map_stats;
pub mod my_info;
pub mod recent_pvp;
pub mod register;
pub mod room_overview;
pub mod room_status;
pub mod room_terrain;
pub mod shards;
pub mod world_start_room;

// don't compile this endpoint template file with regular output, but still compile w/ tests to test for correctness.
#[cfg(test)]
pub mod template;

pub use self::map_stats::MapStats;
pub use self::my_info::MyInfo;
pub use self::recent_pvp::RecentPvp;
pub use self::room_overview::RoomOverview;
pub use self::room_status::RoomStatus;
pub use self::room_terrain::RoomTerrain;
pub use self::shards::ShardInfo;
pub use self::world_start_room::WorldStartRoom;
