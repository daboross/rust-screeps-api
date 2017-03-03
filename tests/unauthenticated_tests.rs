extern crate screeps_api;
extern crate hyper;
extern crate hyper_rustls;
extern crate dotenv;

use hyper::client::Client;
use hyper::net::HttpsConnector;
use screeps_api::error::{Error, ErrorType, ApiError};

fn create_secure_client() -> hyper::Client {
    Client::with_connector(HttpsConnector::new(hyper_rustls::TlsClient::new()))
}

#[test]
fn login_creation_auth_failure() {
    let client = create_secure_client();
    let login = screeps_api::LoginDetails::new("username", "password");
    let mut api = screeps_api::API::new(&client);

    match api.login(&login) {
        Err(Error { err: ErrorType::Unauthorized, .. }) => (),
        Err(other) => panic!("Expected unauthorized error, found other error {}", other),
        Ok(()) => panic!("Expected unauthorized error, found success"),
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
fn test_room_terrain_invalid_room() {
    let client = create_secure_client();
    let mut api = screeps_api::API::new(&client);

    match api.room_terrain("asdffdsa") {
        Err(Error { err: ErrorType::Api(ApiError::InvalidRoom), .. }) => (),
        Err(other) => panic!("Expected invalid room api error, found {}", other),
        Ok(_) => panic!("Expected invalid room api error, found successful result."),
    };
}
