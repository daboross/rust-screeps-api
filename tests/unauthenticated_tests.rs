#![cfg(feature = "sync")]
use screeps_api::error::{ApiError, ErrorKind};
use screeps_api::SyncApi;

#[test]
fn test_world_size() -> anyhow::Result<()> {
    let mut api = SyncApi::new()?;
    let size = api.world_size("shard0")?;
    assert!(size.width > 0);
    assert!(size.height > 0);
    Ok(())
}

#[test]
fn test_rooms_terrain() -> anyhow::Result<()> {
    let mut api = SyncApi::new()?;
    let shard = "shard3";
    let rooms = ["W3N2", "E4S0"];
    let result = api.rooms_terrain(shard, &rooms)?;
    // Invalid rooms in request will be omitted in response
    assert!(result.rooms.len() == rooms.len());
    Ok(())
}

#[test]
fn test_rooms_terrain_with_invalid() -> anyhow::Result<()> {
    let mut api = SyncApi::new()?;
    let rooms = api.rooms_terrain("shard3", &["W2S1", "whatever"])?;
    // Invalid rooms in request will be omitted in response
    assert!(rooms.rooms.len() == 1);
    Ok(())
}

#[test]
fn test_all_rooms_terrain() -> anyhow::Result<()> {
    let mut api = SyncApi::new()?;
    let shard = "shard3";
    let size = api.world_size(shard)?;
    assert!(size.width > 0);
    assert!(size.height > 0);
    let rooms = api.all_rooms_terrain(shard)?;
    // Invalid rooms in request will be omitted in response
    assert!(rooms.rooms.len() == size.width * size.height);
    Ok(())
}

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
        .recent_pvp(screeps_api::RecentPvpArgs::within(15))
        .unwrap();

    let reported_up_to = pvp_results_a
        .shards
        .iter()
        .map(|&(_, ref data)| data.reported_up_to)
        .max()
        .unwrap();

    let _ = api
        .recent_pvp(screeps_api::RecentPvpArgs::since(reported_up_to - 10))
        .unwrap();
}
