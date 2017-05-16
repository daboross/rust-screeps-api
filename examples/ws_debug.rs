// .env parsing
extern crate dotenv;
// command line argument parsing
extern crate clap;
// logging macros
#[macro_use]
extern crate log;
// console logging output
extern crate fern;
extern crate chrono;
// Screeps API
extern crate screeps_api;
// HTTP connection
extern crate hyper;
// secure HTTPS connection
extern crate hyper_rustls;
// json pretty printing
extern crate serde_json;

use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

use hyper::client::Client;
use hyper::net::HttpsConnector;

use screeps_api::sockets::{ParsedMessage, Channel, ChannelUpdate};
use screeps_api::sockets::ws::Result as WsResult;

/// Set up dotenv and retrieve a specific variable, informatively panicking if it does not exist.
fn env(var: &str) -> String {
    dotenv::dotenv().ok();
    match ::std::env::var(var) {
        Ok(value) => value,
        Err(e) => panic!("must have `{}` defined (err: {:?})", var, e),
    }
}

fn setup_logging(verbose: bool) {
    let log_level = if verbose {
        log::LogLevelFilter::Debug
    } else {
        log::LogLevelFilter::Info
    };
    fern::Dispatch::new()
        .level(log_level)
        .level_for("rustls", log::LogLevelFilter::Warn)
        .level_for("hyper", log::LogLevelFilter::Warn)
        .format(|out, message, record| {
            let now = chrono::Local::now();

            out.finish(format_args!("[{}][{}] {}: {}",
                                    now.format("%H:%M:%S"),
                                    record.level(),
                                    record.target(),
                                    message));
        })
        .chain(std::io::stdout())
        .apply()
        // ignore errors
        .unwrap_or(());
}

struct Handler<T: screeps_api::TokenStorage> {
    sender: screeps_api::sockets::Sender,
    tokens: T,
    info: screeps_api::MyInfo,
}

impl<T: screeps_api::TokenStorage> screeps_api::sockets::Handler for Handler<T> {
    fn on_message(&mut self, message: ParsedMessage) -> WsResult<()> {
        match message {
            ParsedMessage::AuthFailed => panic!("authentication with stored token failed!"),
            ParsedMessage::AuthOk { new_token } => {
                info!("authentication succeeded, now authenticated as {}.",
                      self.info.username);
                // return the token to the token storage in case we need it again - we won't in this example
                // program, but this is a good practice
                //
                // TODO: find an efficient way to do this automatically in the handler.
                self.tokens.return_token(new_token);

                let id = &*self.info.user_id;

                self.sender.subscribe(Channel::ServerMessages)?;
                self.sender.subscribe(Channel::user_cpu(id))?;
                self.sender.subscribe(Channel::user_messages(id))?;
                self.sender.subscribe(Channel::user_credits(id))?;
                self.sender.subscribe(Channel::user_console(id))?;
                self.sender.subscribe(Channel::room_map_updates("E0N0"))?;
                self.sender.subscribe(Channel::room_map_updates("E25S23"))?;
                // self.sender.subscribe(Channel::room_map_updates("E24S23"))?;
                // self.sender.subscribe(Channel::room_map_updates("E23S23"))?;
                // self.sender.subscribe(Channel::room_map_updates("E22S23"))?;
                // self.sender.subscribe(Channel::room_updates("W0S0"))?;
                // self.sender.subscribe(Channel::room_updates("W25S23"))?;

                info!("Successfully subscribed to channels!");
            }
            ParsedMessage::ChannelUpdate { update } => {
                match update {
                    ChannelUpdate::UserCpu { user_id, update } => info!("CPU: [{}] {:#?}", user_id, update),
                    ChannelUpdate::RoomMapView { room_name, update } => {
                        info!("Map View: [{}] {:?}", room_name, update);
                    }
                    ChannelUpdate::UserConsole { user_id, update } => {
                        info!("Console: [{}] {:#?}", user_id, update);
                    }
                    ChannelUpdate::Other { channel, update } => {
                        warn!("ChannelUpdate::Other: {}\n{}",
                              channel,
                              serde_json::to_string_pretty(&update).expect("failed to serialize json string"));
                    }
                }
            }
            ParsedMessage::ServerProtocol { protocol } => {
                info!("server protocol: {}", protocol);
            }
            ParsedMessage::ServerTime { time } => {
                info!("server time: {}", time);
            }
            ParsedMessage::ServerPackage { package } => {
                info!("server package: {}", package);
            }
            ParsedMessage::Other(other) => {
                warn!("ParsedMessage::Other: {}", other);
            }
        }

        Ok(())
    }

    /// Run on any websocket error or message parsing error.
    fn on_error(&mut self, err: screeps_api::sockets::Error) {
        error!("{}", err);
    }
}

fn main() {
    let cmd_arguments = clap::App::new("ws-debug")
        .arg(clap::Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .multiple(true)
            .help("Enables verbose logging"))
        .get_matches();
    setup_logging(cmd_arguments.is_present("verbose"));

    // Create a sharable hyper client
    let hyper = Arc::new(Client::with_connector(HttpsConnector::new(hyper_rustls::TlsClient::new())));
    // Create a sharable token storage.
    let token_storage = Arc::new(Mutex::new(VecDeque::new()));
    // Create the API client for this thread.
    let mut client = screeps_api::API::with_token(hyper, token_storage.clone());

    // Login using the API client - this will storage the auth token in token_storage.
    client.login(env("SCREEPS_API_USERNAME"), env("SCREEPS_API_PASSWORD")).expect("failed to login");

    let my_info = client.my_info().unwrap();

    info!("Logged in as {}, attempting to connect to stream.",
          my_info.username);

    let factory_token = token_storage.clone();
    let factory = move |sender| {
        Handler {
            sender: sender,
            tokens: factory_token.clone(),
            info: my_info.clone(),
        }
    };

    // TODO: somehow create a way to go from API url to websocket url.
    screeps_api::sockets::connect("wss://screeps.com/socket/785/40128567/websocket",
                                  factory,
                                  token_storage)
        .expect("failed to connect to socket");
}
