extern crate screeps_api;
extern crate dotenv;

use screeps_api::error::{ErrorKind, ApiError};
use screeps_api::SyncApi;

#[test]
fn test_login_failure() {
    let mut api = SyncApi::new().unwrap();

    match api.login("username", "password") {
        Err(err) => {
            match *err.kind() {
                ErrorKind::Unauthorized => (),
                _ => panic!("expected unauthorized error, found other error {}", err),
            }
        }
        Ok(()) => panic!("expected unauthorized error, found success"),
    }
}

#[test]
fn test_room_terrain() {
    let mut api = SyncApi::new().unwrap();

    api.room_terrain("shard0", "W0N0").unwrap();
    api.room_terrain("shard0", "W3N9").unwrap();
}

#[test]
fn test_room_terrain_invalid_room() {
    let mut api = SyncApi::new().unwrap();

    match api.room_terrain("shard0", "asdffdsa") {
        Err(err) => {
            match *err.kind() {
                ErrorKind::Api(ApiError::InvalidRoom) => (),
                _ => panic!("expected invalid room api error, found {}", err),
            }
        }
        Ok(_) => panic!("expected invalid room api error, found successful result."),
    };
}

#[test]
fn test_room_terrain_invalid_shard() {
    let mut api = SyncApi::new().unwrap();

    match api.room_terrain("sharasdfd0", "asdffdsa") {
        Err(err) => {
            match *err.kind() {
                ErrorKind::Api(ApiError::InvalidShard) => (),
                _ => panic!("expected invalid shard api error, found {}", err),
            }
        }
        Ok(_) => panic!("expected invalid room api error, found successful result."),
    };
}

#[test]
fn test_recent_pvp() {
    let mut api = SyncApi::new().unwrap();

    let pvp_results_a = api.recent_pvp(screeps_api::RecentPvpDetails::within(15)).unwrap();

    let reported_up_to = pvp_results_a.shards.iter().map(|&(_, ref data)| data.reported_up_to).max().unwrap();

    let _ = api.recent_pvp(screeps_api::RecentPvpDetails::since(reported_up_to - 10)).unwrap();
}
