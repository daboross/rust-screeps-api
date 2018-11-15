// .env parsing
extern crate dotenv;
// logging macros
#[macro_use]
extern crate log;
// console logging output
extern crate chrono;
extern crate fern;
// Screeps API
extern crate screeps_api;
// json pretty printing
extern crate serde_json;

use std::borrow::Cow;
use std::io::{self, Write};

fn opt_env(var: &str, default: &'static str) -> Cow<'static, str> {
    dotenv::dotenv().ok();
    match ::std::env::var(var) {
        Ok(value) => {
            if !value.is_empty() {
                value.into()
            } else {
                default.into()
            }
        }
        Err(_) => default.into(),
    }
}

fn setup_logging() {
    fern::Dispatch::new()
        .level(log::LevelFilter::Info)
        .level_for("rustls", log::LevelFilter::Warn)
        .level_for("hyper", log::LevelFilter::Warn)
        // .level_for("screeps_api::connecting", log::LevelFilter::Error)
        .format(|out, message, record| {
            let now = chrono::Local::now();

            out.finish(
                format_args!("[{}][{}] {}: {}", now.format("%H:%M:%S"), record.level(), record.target(), message),
            );
        })
        .chain(std::io::stdout())
        .apply()
        // if we've already set a logger, it's OK.
        .unwrap_or(());
}

fn server_url() -> Cow<'static, str> {
    opt_env("SCREEPS_API_URL", "http://127.0.0.1:21025/api/")
}

fn main() {
    setup_logging();

    match perform_registration() {
        Ok(()) => (),
        Err(e) => {
            eprintln!("error: {}", e);
            std::process::exit(1);
        }
    }
}

fn perform_registration() -> Result<(), Box<std::error::Error>> {
    let http_url = server_url();

    let mut client = screeps_api::SyncConfig::new()
        .unwrap()
        .url(&http_url)
        .build()?;

    println!("New user registration! Connecting to {}", http_url);

    print!("username to register > ");
    io::stdout().flush()?;
    let mut username = String::new();
    io::stdin().read_line(&mut username)?;

    let username = username.trim();

    print!("password to register > ");
    io::stdout().flush()?;
    let mut password = String::new();
    io::stdin().read_line(&mut password)?;

    let password = password.trim();

    println!(
        "attempting to register username {:?} onto '{}'",
        username, http_url
    );

    client.register(screeps_api::RegistrationDetails::new(username, password))?;

    println!("registration succeeded! attempting to verify via login.");

    client.login(username, password)?;

    let my_info = client.my_info()?;

    info!("success! created user {:#?}", my_info);

    Ok(())
}
