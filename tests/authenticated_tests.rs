extern crate screeps_api;
extern crate hyper;
extern crate hyper_rustls;
extern crate dotenv;

use hyper::client::Client;
use hyper::net::HttpsConnector;
use screeps_api::error::{Error, ErrorType, ApiError};

/// Set up dotenv and retrieve a specific variable, informatively panicking if it does not exist.
fn env(var: &str) -> String {
    dotenv::dotenv().ok();
    match ::std::env::var(var) {
        Ok(value) => value,
        Err(e) => panic!("must have `{}` defined (err: {:?})", var, e),
    }
}

fn create_secure_client() -> hyper::Client {
    Client::with_connector(HttpsConnector::new(hyper_rustls::TlsClient::new()))
}

fn logged_in(client: &hyper::Client) -> screeps_api::API {
    let username = env("SCREEPS_API_USERNAME");
    let password = env("SCREEPS_API_PASSWORD");
    let mut api = screeps_api::API::new(client);

    if let Err(err) = api.login(&screeps_api::LoginDetails::new(username, password)) {
        panic!("Error logging in: {:?}\nTo disable login tests, use `cargo test -- --skip auth`",
               err);
    }

    api
}

#[test]
fn test_auth_my_info() {
    let client = create_secure_client();
    let mut api = logged_in(&client);

    let _ = api.my_info().unwrap();
}

#[test]
fn test_auth_token_reretrieval() {
    let client = create_secure_client();
    let mut api = logged_in(&client);

    let _ = api.my_info().unwrap();

    let _ = api.my_info().unwrap();

    let _ = api.my_info().unwrap();
}

#[test]
fn test_auth_room_overview() {
    let client = create_secure_client();
    let mut api = logged_in(&client);

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
    let client = create_secure_client();
    let mut api = logged_in(&client);

    api.leaderboard_season_list().unwrap();
}

#[test]
fn test_auth_retrieve_single_rank() {
    let client = create_secure_client();
    let mut api = logged_in(&client);

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
    let client = create_secure_client();
    let mut api = logged_in(&client);

    let result = api.find_leaderboard_ranks(screeps_api::LeaderboardType::GlobalControl, "daboross")
        .unwrap();
    assert!(result.len() > 0);
}

#[test]
fn test_auth_retrieve_leaderboard() {
    let client = create_secure_client();
    let mut api = logged_in(&client);

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
    let client = create_secure_client();
    let mut api = logged_in(&client);

    match api.leaderboard_page(screeps_api::LeaderboardType::GlobalControl,
                               "2017-02",
                               21,
                               0) {
        Err(Error { err: ErrorType::Api(ApiError::InvalidParameters), .. }) => (),
        Err(other) => panic!("expected InvalidParameters error, found other error {}", other),
        Ok(other) => panic!("expected InvalidParameters error, found success: {:?}", other),
    }

}
