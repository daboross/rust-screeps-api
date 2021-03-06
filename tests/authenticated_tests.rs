#![cfg(feature = "sync")]
use screeps_api::error::{ApiError, ErrorKind};
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
    let token = env("SCREEPS_API_TOKEN");
    SyncApi::new().unwrap().with_token(token)
}

#[test]
fn test_auth_my_info() {
    let mut api = logged_in();

    let _ = api.my_info().unwrap();
}

#[test]
fn test_auth_map_stats() {
    let mut api = logged_in();

    let result = api
        .map_stats(
            "shard2",
            &[
                "W13S21",
                "W12S20",
                "E32N29",
                "E50N35",
                "E90N90",
                "InvalidRoomName",
            ],
        )
        .unwrap();

    assert_eq!(result.rooms.len(), 4);
}

#[test]
fn test_auth_world_start() {
    let mut api = logged_in();

    let start = api.world_start_room().unwrap();

    let shard = start.shard.as_ref().map(AsRef::as_ref).unwrap_or("");

    let result = api.map_stats(shard, &[start.room_name]).unwrap();

    assert_eq!(result.rooms.len(), 1);
}

#[test]
fn test_auth_token_reretrieval() {
    let mut api = logged_in();

    api.my_info().unwrap();

    api.my_info().unwrap();

    api.my_info().unwrap();
}

#[test]
fn test_auth_room_overview() {
    let mut api = logged_in();

    for &interval in &[8u32, 180u32, 1440u32] {
        // At the time of writing, a room owned by a user who does not have a custom badge.
        api.room_overview("shard0", "W1N1", interval).unwrap();

        // At time of writing, one of dissi's rooms, a user who has a custom badge.
        api.room_overview("shard0", "W3N9", interval).unwrap();

        // A room that can't be owned on the official server.
        api.room_overview("shard0", "W0N0", interval).unwrap();
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

    api.find_season_leaderboard_rank(
        screeps_api::LeaderboardType::GlobalControl,
        "daboross",
        "2017-02",
    )
    .unwrap();

    match api.find_season_leaderboard_rank(
        screeps_api::LeaderboardType::GlobalControl,
        "username_should_not_exist_ever_let's_just_make_it_long",
        "2017-02",
    ) {
        Err(err) => match *err.kind() {
            ErrorKind::Api(ApiError::UserNotFound) => (),
            _ => panic!("expected UserNotFound error, found other error {}", err),
        },
        Ok(other) => panic!("expected UserNotFound error, found success: {:?}", other),
    }
    // "daboross" did not process any power during the 2017-02 season of the official server.
    match api.find_season_leaderboard_rank(
        screeps_api::LeaderboardType::PowerProcessed,
        "daboross",
        "2017-02",
    ) {
        Err(err) => match *err.kind() {
            ErrorKind::Api(ApiError::ResultNotFound) => (),
            _ => panic!("expected ResultNotFound error, found other error {}", err),
        },
        Ok(other) => panic!("expected ResultNotFound error, found success: {:?}", other),
    }
}

#[test]
fn test_auth_retrieve_all_ranks() {
    let mut api = logged_in();

    let result = api
        .find_leaderboard_ranks(screeps_api::LeaderboardType::GlobalControl, "daboross")
        .unwrap();
    assert!(!result.is_empty());
}

#[test]
fn test_auth_retrieve_leaderboard() {
    let mut api = logged_in();

    let result = api
        .leaderboard_page(
            screeps_api::LeaderboardType::GlobalControl,
            "2017-02",
            10,
            0,
        )
        .unwrap();

    for ranked_user in result.ranks {
        if result
            .user_details
            .iter()
            .find(|t| t.0 == ranked_user.user_id)
            .is_none()
        {
            panic!(
                "expected user_details to contain ranked_user user_id, but found {:?} did not contain {:?}",
                result.user_details, ranked_user.user_id
            );
        }
    }
}

/// This is to ensure that the documentation stays up to date if this ever changes.
#[test]
fn test_auth_leaderboard_limit_parameter_error() {
    let mut api = logged_in();

    match api.leaderboard_page(
        screeps_api::LeaderboardType::GlobalControl,
        "2017-02",
        21,
        0,
    ) {
        Err(err) => match *err.kind() {
            ErrorKind::Api(ApiError::InvalidParameters) => (),
            _ => {
                panic!(
                    "expected InvalidParameters error, found other error {}",
                    err
                );
            }
        },
        Ok(other) => {
            panic!(
                "expected InvalidParameters error, found success: {:?}",
                other
            );
        }
    }
}

#[test]
#[cfg(feature = "destructive-tests")]
fn test_memory_segment() {
    let mut api = logged_in();

    let orig = api.memory_segment(Some("shard0"), 1).unwrap();

    api.set_memory_segment(Some("shard0"), 1, "hi, you!")
        .unwrap();

    let retrieved = api.memory_segment(Some("shard0"), 1).unwrap();

    api.set_memory_segment(Some("shard0"), 1, orig).unwrap();

    assert_eq!(&retrieved, "hi, you!");
}
