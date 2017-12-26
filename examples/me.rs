//! Simple command line program to view the information of the user logged in.
//!
//! Logs in using the SCREEPS_API_USERNAME and SCREEPS_API_PASSWORD env variables.
extern crate dotenv;
extern crate fern;
extern crate hyper;
extern crate log;
extern crate screeps_api;

use std::borrow::Cow;

/// Set up dotenv and retrieve a specific variable, informatively panicking if it does not exist.
fn env(var: &str) -> String {
    dotenv::dotenv().ok();
    match ::std::env::var(var) {
        Ok(value) => value,
        Err(e) => panic!("must have `{}` defined (err: {:?})", var, e),
    }
}

fn opt_env(var: &str, default: &'static str) -> Cow<'static, str> {
    dotenv::dotenv().ok();
    match ::std::env::var(var) {
        Ok(value) => value.into(),
        Err(_) => default.into(),
    }
}

fn main() {
    fern::Dispatch::new()
        .level(log::LevelFilter::Warn)
        .chain(std::io::stdout())
        .apply()
        .unwrap();

    let mut client = screeps_api::SyncConfig::new()
        .unwrap()
        .url(&opt_env(
            "SCREEPS_API_URL",
            screeps_api::DEFAULT_OFFICIAL_API_URL,
        ))
        .build()
        .unwrap();

    client
        .login(env("SCREEPS_API_USERNAME"), env("SCREEPS_API_PASSWORD"))
        .unwrap();

    let my_info = client.my_info().unwrap();

    println!(
        "User {}:\
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
        my_info.credits
    );
}
