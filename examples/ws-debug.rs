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

fn setup_logging(verbosity: u64) {
    let log_level = match verbosity {
        0 => log::LogLevelFilter::Warn,
        1 => log::LogLevelFilter::Info,
        _ => log::LogLevelFilter::Debug,
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

#[derive(Clone, Debug)]
struct Config {
    cpu: bool,
    messages: bool,
    credits: bool,
    console: bool,
    rooms: Vec<String>,
    map_view: Vec<String>,
}

impl Config {
    fn new(args: &clap::ArgMatches) -> Self {
        Config {
            cpu: args.is_present("cpu"),
            messages: args.is_present("messages"),
            credits: args.is_present("credits"),
            console: args.is_present("console"),
            rooms: args.values_of("room")
                .map(|it| it.map(|v| v.to_uppercase()).collect())
                .unwrap_or_else(|| Vec::new()),
            map_view: args.values_of("map-view")
                .map(|it| it.map(|v| v.to_uppercase()).collect())
                .unwrap_or_else(|| Vec::new()),
        }
    }

    fn subscribe_with(&self, id: &str, sender: &mut screeps_api::sockets::Sender) -> WsResult<()> {
        sender.subscribe(Channel::ServerMessages)?;

        if self.cpu {
            sender.subscribe(Channel::user_cpu(id))?;
        }

        if self.messages {
            sender.subscribe(Channel::user_messages(id))?;
            sender.subscribe(Channel::user_conversation(id, "57fb16b6e4dd183b746435b0"))?;
        }

        if self.credits {
            sender.subscribe(Channel::user_credits(id))?;
        }

        if self.console {
            sender.subscribe(Channel::user_console(id))?;
        }

        for room_name in &self.rooms {
            sender.subscribe(Channel::room_detail(&**room_name))?;
        }

        for room_name in &self.map_view {
            sender.subscribe(Channel::room_map_view(&**room_name))?;
        }

        Ok(())
    }
}

struct Handler<T: screeps_api::TokenStorage> {
    sender: screeps_api::sockets::Sender,
    tokens: T,
    info: screeps_api::MyInfo,
    config: Config,
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

                self.config.subscribe_with(&self.info.user_id, &mut self.sender)?;

                warn!("Subscribed.");
            }
            ParsedMessage::ChannelUpdate { update } => {
                match update {
                    ChannelUpdate::UserCpu { user_id, update } => info!("CPU: [{}] {:#?}", user_id, update),
                    ChannelUpdate::RoomMapView { room_name, update } => {
                        info!("Map View: [{}] {:?}", room_name, update);
                    }
                    ChannelUpdate::RoomDetail { room_name, update } => {
                        info!("Room Detail: [{}] {:?}", room_name, update);
                    }
                    ChannelUpdate::NoRoomDetail { room_name } => {
                        info!("Room Skipped: {}", room_name);
                    }
                    ChannelUpdate::UserConsole { user_id, update } => {
                        info!("Console: [{}] {:#?}", user_id, update);
                    }
                    ChannelUpdate::UserCredits { user_id, update } => {
                        info!("Credits: [{}] {}", user_id, update);
                    }
                    ChannelUpdate::UserMessage { user_id, update } => {
                        info!("New message: [{}] {:#?}", user_id, update);
                    }
                    ChannelUpdate::UserConversation { user_id, target_user_id, update } => {
                        info!("Conversation update: [{}->{}] {:#?}",
                              user_id,
                              target_user_id,
                              update);
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
        .arg(clap::Arg::with_name("cpu")
            .short("p")
            .long("cpu")
            .help("Subscribe to user cpu and memory updates"))
        .arg(clap::Arg::with_name("credits")
            .short("c")
            .long("credits")
            .help("Subscribe to per-tick user credit updates"))
        .arg(clap::Arg::with_name("console")
            .short("o")
            .long("console")
            .help("Subscribe to user console messages"))
        .arg(clap::Arg::with_name("messages")
            .short("e")
            .long("messages")
            .help("Subscribe to user message alerts"))
        .arg(clap::Arg::with_name("room")
            .short("r")
            .long("room")
            .value_name("ROOM_NAME")
            .help("Subscribes to a room")
            .takes_value(true)
            .multiple(true))
        .arg(clap::Arg::with_name("map-view")
            .short("m")
            .long("map-view")
            .value_name("ROOM_NAME")
            .help("Subscribes to a map-view room")
            .takes_value(true)
            .multiple(true))
        .get_matches();
    setup_logging(cmd_arguments.occurrences_of("verbose"));

    let config = Config::new(&cmd_arguments);

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
            config: config.clone(),
        }
    };

    // TODO: somehow create a way to go from API url to websocket url.
    screeps_api::sockets::connect("wss://screeps.com/socket/785/40128567/websocket",
                                  factory,
                                  token_storage)
        .expect("failed to connect to socket");
}
