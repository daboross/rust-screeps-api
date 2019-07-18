//! Parsing code for each individual API endpoint.
//!
//! Each sub-module contains code for interpreting the result of calling a specific API endpoint.
mod leaderboard;
mod login;
mod map_stats;
mod memory_segment;
mod my_info;
mod recent_pvp;
mod register;
mod room_overview;
mod room_status;
mod room_terrain;
mod set_memory_segment;
mod shards;
mod world_start_room;

// don't compile this endpoint template file with regular output, but still compile w/ tests to test for correctness.
#[cfg(test)]
pub mod template;

pub use self::{
    leaderboard::*, login::*, map_stats::*, my_info::*, recent_pvp::*, register::*,
    room_overview::*, room_status::*, room_terrain::*, set_memory_segment::*, shards::*,
    world_start_room::*,
};

pub(crate) use self::memory_segment::*;
