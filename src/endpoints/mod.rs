//! Parsing code for each individual API endpoint.
//!
//! Each sub-module contains code for interpreting the result of calling a specific API endpoint.
pub mod leaderboard;
pub mod login;
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

pub use self::{
    map_stats::MapStats, my_info::MyInfo, recent_pvp::RecentPvp, room_overview::RoomOverview,
    room_status::RoomStatus, room_terrain::RoomTerrain, shards::ShardInfo,
    world_start_room::WorldStartRoom,
};
