//! Endpoint modules for the screeps API.
//!
//! Each sub-module contains code for interpreting the result of calling a specific API endpoint.
pub mod login;
pub mod my_info;
pub mod room_overview;
pub mod room_status;
pub mod room_terrain;

// don't compile this endpoint template file with regular output, but still compile w/ tests to test for correctness.
#[cfg(test)]
pub mod template;
