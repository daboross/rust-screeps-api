extern crate screeps_api;
extern crate hyper;
extern crate hyper_rustls;
extern crate dotenv;

use hyper::client::Client;
use hyper::net::HttpsConnector;

fn env(var: &str) -> String {
    dotenv::dotenv().ok();
    match ::std::env::var(var) {
        Ok(value) => value,
        Err(_) => panic!("must have `{}` defined", var),
    }
}

fn create_secure_client() -> hyper::Client {
    Client::with_connector(HttpsConnector::new(hyper_rustls::TlsClient::new()))
}

fn logged_in<'a>(client: &'a hyper::Client) -> screeps_api::API<'a> {
    let username = env("SCREEPS_API_USERNAME");
    let password = env("SCREEPS_API_PASSWORD");
    let mut api = screeps_api::API::new(client);

    if let Err(err) = api.login(&screeps_api::LoginDetails::new(username, password)) {
        panic!("Error logging in: {:?}", err);
    }

    api
}

#[test]
fn test_logging_in() {
    let client = create_secure_client();
    let _api = logged_in(&client);
}
