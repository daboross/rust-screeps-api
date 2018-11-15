#![cfg(feature = "sync")]
extern crate dotenv;
extern crate screeps_api;

use screeps_api::error::{ApiError, ErrorKind};
use screeps_api::SyncApi;

#[test]
fn test_room_terrain() {
    let mut api = SyncApi::new().unwrap();

    api.room_terrain(Some("shard0"), "W0N0").unwrap();
    api.room_terrain(Some("shard0"), "W3N9").unwrap();
}

#[test]
fn test_shard_list() {
    let mut api = SyncApi::new().unwrap();

    let list = api.shard_list().unwrap();
    assert!(list.len() >= 2);
}

#[test]
fn test_room_terrain_invalid_room() {
    let mut api = SyncApi::new().unwrap();

    match api.room_terrain(Some("shard0"), "asdffdsa") {
        Err(err) => match *err.kind() {
            ErrorKind::Api(ApiError::InvalidRoom) => (),
            _ => panic!("expected invalid room api error, found {}", err),
        },
        Ok(_) => panic!("expected invalid room api error, found successful result."),
    };
}

#[test]
fn test_room_terrain_invalid_shard() {
    let mut api = SyncApi::new().unwrap();

    match api.room_terrain(Some("sharasdfd0"), "asdffdsa") {
        Err(err) => match *err.kind() {
            ErrorKind::Api(ApiError::InvalidShard) => (),
            _ => panic!("expected invalid shard api error, found {}", err),
        },
        Ok(_) => panic!("expected invalid room api error, found successful result."),
    };
}

#[test]
fn test_recent_pvp() {
    let mut api = SyncApi::new().unwrap();

    let pvp_results_a = api
        .recent_pvp(screeps_api::RecentPvpDetails::within(15))
        .unwrap();

    let reported_up_to = pvp_results_a
        .shards
        .iter()
        .map(|&(_, ref data)| data.reported_up_to)
        .max()
        .unwrap();

    let _ = api
        .recent_pvp(screeps_api::RecentPvpDetails::since(reported_up_to - 10))
        .unwrap();
}
