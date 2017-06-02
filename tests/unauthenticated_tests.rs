extern crate screeps_api;
extern crate dotenv;

use screeps_api::error::{Error, ErrorType, ApiError};
use screeps_api::SyncApi;

#[test]
fn test_login_failure() {
    let mut api = SyncApi::new().unwrap();

    match api.login("username", "password") {
        Err(Error { err: ErrorType::Unauthorized, .. }) => (),
        Err(other) => panic!("expected unauthorized error, found other error {}", other),
        Ok(()) => panic!("expected unauthorized error, found success"),
    }
}

#[test]
fn test_room_terrain() {
    let mut api = SyncApi::new().unwrap();

    api.room_terrain("W0N0").unwrap();
    api.room_terrain("W3N9").unwrap();
}

#[test]
fn test_room_terrain_invalid_room() {
    let mut api = SyncApi::new().unwrap();

    match api.room_terrain("asdffdsa") {
        Err(Error { err: ErrorType::Api(ApiError::InvalidRoom), .. }) => (),
        Err(other) => panic!("expected invalid room api error, found {}", other),
        Ok(_) => panic!("expected invalid room api error, found successful result."),
    };
}

#[test]
fn test_recent_pvp() {
    let mut api = SyncApi::new().unwrap();

    let pvp_results_a = api.recent_pvp(screeps_api::RecentPvpDetails::within(15)).unwrap();

    let _ = api.recent_pvp(screeps_api::RecentPvpDetails::since(pvp_results_a.reported_up_to - 10)).unwrap();
}
