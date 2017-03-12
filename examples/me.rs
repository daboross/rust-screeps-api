//! Simple command line program to view the information of the user logged in.
//!
//! Logs in using the SCREEPS_API_USERNAME and SCREEPS_API_PASSWORD env variables.
extern crate screeps_api;
extern crate hyper;
extern crate hyper_rustls;
extern crate dotenv;

use hyper::client::Client;
use hyper::net::HttpsConnector;

/// Set up dotenv and retrieve a specific variable, informatively panicking if it does not exist.
fn env(var: &str) -> String {
    dotenv::dotenv().ok();
    match ::std::env::var(var) {
        Ok(value) => value,
        Err(e) => panic!("must have `{}` defined (err: {:?})", var, e),
    }
}

fn main() {
    let hyper = Client::with_connector(HttpsConnector::new(hyper_rustls::TlsClient::new()));
    let mut client = screeps_api::API::new(&hyper);

    client.login(&screeps_api::LoginDetails::new(env("SCREEPS_API_USERNAME"), env("SCREEPS_API_PASSWORD"))).unwrap();

    let my_info = client.my_info().unwrap();

    println!("User {}:\
\n\tID: {}\
\n\tPassword: {}\
\n\tCPU: {}\
\n\tGCL points: {}\
\n\tCredits: {}",
             &my_info.username,
             &my_info.user_id,
             if my_info.has_password {
                 "exists"
             } else {
                 "not set"
             },
             my_info.cpu,
             my_info.gcl_points,
             my_info.credits);
}
