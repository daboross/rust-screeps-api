extern crate screeps_api;
extern crate dotenv;

use screeps_api::error::{Error, ErrorType, ApiError};
use screeps_api::SyncApi;

/// Set up dotenv and retrieve a specific variable, informatively panicking if it does not exist.
fn env(var: &str) -> String {
    dotenv::dotenv().ok();
    match ::std::env::var(var) {
        Ok(value) => value,
        Err(e) => panic!("must have `{}` defined (err: {:?})", var, e),
    }
}

fn logged_in() -> SyncApi {
    let username = env("SCREEPS_API_USERNAME");
    let password = env("SCREEPS_API_PASSWORD");
    let mut api = SyncApi::new().unwrap();

    if let Err(err) = api.login(username, password) {
        panic!("Error logging in: {:?}\nTo disable login tests, use `cargo test -- --skip auth`",
               err);
    }

    api
}

#[test]
fn test_auth_my_info() {
    let mut api = logged_in();

    let _ = api.my_info().unwrap();
}

#[test]
fn test_auth_map_stats() {
    let mut api = logged_in();

    let result = api.map_stats(&["E15N52", "E19S79", "E14S78", "E19S81", "W6S67", "InvalidRoomName"]).unwrap();

    assert_eq!(result.rooms.len(), 5);
}

#[test]
fn test_auth_token_reretrieval() {
    let mut api = logged_in();

    api.my_info().unwrap();

    api.my_info().unwrap();

    api.my_info().unwrap();
}

#[test]
fn test_auth_shared_token_storage() {
    // let shared = std::rc::Rc::new(std::cell::RefCell::new(None::<screeps_api::Token>));

    // let username = env("SCREEPS_API_USERNAME");
    // let password = env("SCREEPS_API_PASSWORD");

    // TODO: this is disabled waiting for screeps_api::sync::Config to be implemented.

    // screeps_api::API::with_token(shared_client.clone(), shared.clone()).login(username, password).unwrap();

    // screeps_api::API::with_token(shared_client.clone(), shared.clone()).my_info().unwrap();

    // screeps_api::API::with_token(shared_client.clone(), shared.clone()).my_info().unwrap();

    // screeps_api::API::with_token(shared_client.clone(), shared.clone()).my_info().unwrap();

    // screeps_api::API::with_token(shared_client.clone(), shared.clone()).my_info().unwrap();
}

#[test]
fn test_auth_room_overview() {
    let mut api = logged_in();

    for &interval in &[8u32, 180u32, 1440u32] {
        // At the time of writing, a room owned by a user who does not have a custom badge.
        api.room_overview("W1N1", interval).unwrap();

        // At time of writing, one of dissi's rooms, a user who has a custom badge.
        api.room_overview("W3N9", interval).unwrap();

        // A room that can't be owned on the official server.
        api.room_overview("W0N0", interval).unwrap();
    }
}

#[test]
fn test_auth_leaderboard_seasons() {
    let mut api = logged_in();

    api.leaderboard_season_list().unwrap();
}

#[test]
fn test_auth_retrieve_single_rank() {
    let mut api = logged_in();

    api.find_season_leaderboard_rank(screeps_api::LeaderboardType::GlobalControl,
                                      "daboross",
                                      "2017-02")
        .unwrap();

    match api.find_season_leaderboard_rank(screeps_api::LeaderboardType::GlobalControl,
                                           "username_should_not_exist_ever_let's_just_make_it_long",
                                           "2017-02") {
        Err(Error { err: ErrorType::Api(ApiError::UserNotFound), .. }) => (),
        Err(other) => panic!("expected UserNotFound error, found other error {}", other),
        Ok(other) => panic!("expected UserNotFound error, found success: {:?}", other),
    }
    // "daboross" did not process any power during the 2017-02 season of the official server.
    match api.find_season_leaderboard_rank(screeps_api::LeaderboardType::PowerProcessed,
                                           "daboross",
                                           "2017-02") {
        Err(Error { err: ErrorType::Api(ApiError::ResultNotFound), .. }) => (),
        Err(other) => panic!("expected ResultNotFound error, found other error {}", other),
        Ok(other) => panic!("expected ResultNotFound error, found success: {:?}", other),
    }
}

#[test]
fn test_auth_retrieve_all_ranks() {
    let mut api = logged_in();

    let result = api.find_leaderboard_ranks(screeps_api::LeaderboardType::GlobalControl, "daboross")
        .unwrap();
    assert!(!result.is_empty());
}

#[test]
fn test_auth_retrieve_leaderboard() {
    let mut api = logged_in();

    let result = api.leaderboard_page(screeps_api::LeaderboardType::GlobalControl,
                          "2017-02",
                          10,
                          0)
        .unwrap();

    for ranked_user in result.ranks {
        if !result.user_details.contains_key(&ranked_user.user_id) {
            panic!("expected user_details to contain ranked_user user_id, but found {:?} did not contain {:?}",
                   result.user_details,
                   ranked_user.user_id);
        }
    }
}

/// This is to ensure that the documentation stays up to date if this ever changes.
#[test]
fn test_auth_leaderboard_limit_parameter_error() {
    let mut api = logged_in();

    match api.leaderboard_page(screeps_api::LeaderboardType::GlobalControl,
                               "2017-02",
                               21,
                               0) {
        Err(Error { err: ErrorType::Api(ApiError::InvalidParameters), .. }) => (),
        Err(other) => {
            panic!("expected InvalidParameters error, found other error {}",
                   other)
        }
        Ok(other) => {
            panic!("expected InvalidParameters error, found success: {:?}",
                   other)
        }
    }

}
