extern crate screeps_api;
extern crate hyper;
extern crate hyper_rustls;
extern crate hyper_openssl;
extern crate dotenv;

use hyper::client::Client;
use hyper::net::HttpsConnector;
use screeps_api::error::{Error, ErrorType, ApiError};

fn create_secure_client() -> hyper::Client {
    Client::with_connector(HttpsConnector::new(hyper_rustls::TlsClient::new()))
}

#[test]
fn test_login_failure() {
    let client = create_secure_client();
    let mut api = screeps_api::API::new(&client);

    match api.login("username", "password") {
        Err(Error { err: ErrorType::Unauthorized, .. }) => (),
        Err(other) => panic!("expected unauthorized error, found other error {}", other),
        Ok(()) => panic!("expected unauthorized error, found success"),
    }
}

#[test]
fn test_room_terrain() {
    let client = create_secure_client();
    let mut api = screeps_api::API::new(&client);

    api.room_terrain("W0N0").unwrap();
    api.room_terrain("W3N9").unwrap();
}

#[test]
fn test_openssl_room_terrain_connection() {
    let client = Client::with_connector(HttpsConnector::new(hyper_openssl::OpensslClient::new().unwrap()));

    let mut api = screeps_api::API::new(&client);

    api.room_terrain("W20N21").unwrap();
    api.room_terrain("E34S34").unwrap();
}

#[test]
fn test_room_terrain_invalid_room() {
    let client = create_secure_client();
    let mut api = screeps_api::API::new(&client);

    match api.room_terrain("asdffdsa") {
        Err(Error { err: ErrorType::Api(ApiError::InvalidRoom), .. }) => (),
        Err(other) => panic!("expected invalid room api error, found {}", other),
        Ok(_) => panic!("expected invalid room api error, found successful result."),
    };
}

#[test]
fn test_recent_pvp() {
    let client = create_secure_client();
    let mut api = screeps_api::API::new(&client);

    let pvp_results_a = api.recent_pvp(screeps_api::RecentPvpDetails::within(15)).unwrap();

    let _ = api.recent_pvp(screeps_api::RecentPvpDetails::since(pvp_results_a.reported_up_to - 10)).unwrap();
}
