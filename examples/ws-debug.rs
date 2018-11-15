// .env parsing

extern crate dotenv;
// command line argument parsing

extern crate clap;
// logging macros

#[macro_use]
extern crate log;
// console logging output

extern crate chrono;
extern crate fern;
// sockets

extern crate futures;
extern crate tokio_core;
extern crate websocket;
// Screeps API

extern crate screeps_api;
// json pretty printing

extern crate serde_json;

use std::borrow::Cow;

use futures::{future, stream, Future, Sink, Stream};

use websocket::OwnedMessage;

use screeps_api::websocket::{Channel, ChannelUpdate, ScreepsMessage, SockjsMessage};
use screeps_api::{RoomName, TokenStorage};

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
        0 => log::LevelFilter::Warn,
        1 => log::LevelFilter::Info,
        _ => log::LevelFilter::Debug,
    };
    fern::Dispatch::new()
        .level(log_level)
        .level_for("rustls", log::LevelFilter::Warn)
        .level_for("hyper", log::LevelFilter::Warn)
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
    shard: Option<Cow<'static, str>>,
    rooms: Vec<RoomName>,
    map_view: Vec<RoomName>,
    url: Cow<'static, str>,
}

impl Config {
    fn new<'a>(
        args: &'a clap::ArgMatches,
    ) -> Result<Self, screeps_api::data::room_name::RoomNameParseError<'a>> {
        #[allow(unused_imports)] // becomes unnecessary for rust 1.23+
        use std::ascii::AsciiExt;

        Ok(Config {
            cpu: args.is_present("cpu"),
            messages: args.is_present("messages"),
            credits: args.is_present("credits"),
            console: args.is_present("console"),
            shard: args
                .value_of("shard")
                .map(|v| {
                    if "none".eq_ignore_ascii_case(v) {
                        None
                    } else {
                        Some(v.to_owned().into())
                    }
                })
                .unwrap_or_else(|| Some("shard0".into())),
            rooms: args
                .values_of("room")
                .map(|it| it.map(|v| RoomName::new(v)).collect::<Result<_, _>>())
                .unwrap_or_else(|| Ok(Vec::new()))?,
            map_view: args
                .values_of("map-view")
                .map(|it| it.map(|v| RoomName::new(v)).collect::<Result<_, _>>())
                .unwrap_or_else(|| Ok(Vec::new()))?,
            url: args
                .value_of("url")
                .map(|v| v.to_owned().into())
                .unwrap_or_else(|| screeps_api::DEFAULT_OFFICIAL_API_URL.into()),
        })
    }

    fn subscribe_with(
        &self,
        id: &str,
    ) -> Box<Stream<Item = OwnedMessage, Error = websocket::WebSocketError>> {
        use screeps_api::websocket::subscribe;

        let mut messages = Vec::with_capacity(
            1 + self.cpu as usize
                + self.messages as usize
                + self.credits as usize
                + self.console as usize
                + self.rooms.len()
                + self.map_view.len(),
        );

        messages.push(subscribe(&Channel::ServerMessages));

        if self.cpu {
            messages.push(subscribe(&Channel::user_cpu(id)));
        }

        if self.messages {
            messages.push(subscribe(&Channel::user_messages(id)));
            messages.push(subscribe(&Channel::user_conversation(
                id,
                "57fb16b6e4dd183b746435b0",
            )));
        }

        if self.credits {
            messages.push(subscribe(&Channel::user_credits(id)));
        }

        if self.console {
            messages.push(subscribe(&Channel::user_console(id)));
        }

        for room_name in &self.rooms {
            messages.push(subscribe(&Channel::room_detail(
                *room_name,
                self.shard.as_ref().map(AsRef::as_ref),
            )));
        }

        for room_name in &self.map_view {
            messages.push(subscribe(&Channel::room_map_view(
                *room_name,
                self.shard.as_ref().map(AsRef::as_ref),
            )));
        }

        Box::new(stream::iter_ok(
            messages.into_iter().map(OwnedMessage::Text),
        ))
    }
}

fn setup() -> Config {
    let cmd_arguments = clap::App::new("ws-debug")
        .arg(
            clap::Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .multiple(true)
                .help("Enables verbose logging"),
        )
        .arg(
            clap::Arg::with_name("cpu")
                .short("p")
                .long("cpu")
                .help("Subscribe to user cpu and memory updates"),
        )
        .arg(
            clap::Arg::with_name("credits")
                .short("c")
                .long("credits")
                .help("Subscribe to per-tick user credit updates"),
        )
        .arg(
            clap::Arg::with_name("console")
                .short("o")
                .long("console")
                .help("Subscribe to user console messages"),
        )
        .arg(
            clap::Arg::with_name("messages")
                .short("e")
                .long("messages")
                .help("Subscribe to user message alerts"),
        )
        .arg(
            clap::Arg::with_name("shard")
                .short("s")
                .long("shard")
                .value_name("SHARD_NAME")
                .help("Sets the shard (default shard0, use 'None' for no shard)")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("room")
                .short("r")
                .long("room")
                .value_name("ROOM_NAME")
                .help("Subscribes to a room")
                .takes_value(true)
                .multiple(true),
        )
        .arg(
            clap::Arg::with_name("map-view")
                .short("m")
                .long("map-view")
                .value_name("ROOM_NAME")
                .help("Subscribes to a map-view room")
                .takes_value(true)
                .multiple(true),
        )
        .arg(
            clap::Arg::with_name("url")
                .short("u")
                .long("url")
                .value_name("API_URL")
                .help("Server url to connect to")
                .takes_value(true),
        )
        .get_matches();

    setup_logging(cmd_arguments.occurrences_of("verbose"));

    match Config::new(&cmd_arguments) {
        Ok(v) => v,
        Err(e) => clap::Error {
            message: e.to_string(),
            kind: clap::ErrorKind::InvalidValue,
            info: None,
        }
        .exit(),
    }
}

fn main() {
    let config = setup();

    let token_storage = screeps_api::RcTokenStorage::default();

    let mut client = screeps_api::SyncConfig::new()
        .unwrap()
        .url(&config.url)
        .tokens(token_storage.clone())
        .build()
        .unwrap();

    // Login using the API client - this will storage the auth token in token_storage.
    client
        .login(env("SCREEPS_API_USERNAME"), env("SCREEPS_API_PASSWORD"))
        .expect("failed to login");

    let my_info = client.my_info().unwrap();

    info!(
        "Logged in as {}, attempting to connect to stream.",
        my_info.username
    );

    let mut core =
        tokio_core::reactor::Core::new().expect("expected IO core to start up without issue.");

    let handle = core.handle();

    let ws_url = screeps_api::websocket::connecting::transform_url(&config.url)
        .expect("expected server api url to parse into websocket url.");

    let connection = websocket::ClientBuilder::from_url(&ws_url).async_connect(None, &handle);

    core.run(connection.then(|result| {
        let (client, _) = result.expect("connecting to server failed");

        let (sink, stream) = client.split();

        sink.send(OwnedMessage::Text(screeps_api::websocket::authenticate(
            &token_storage.take_token().unwrap(),
        )))
        .and_then(|sink| {
            let handler = Handler::new(token_storage, my_info, config);

            sink.send_all(
                stream
                    .and_then(move |data| future::ok(handler.handle_data(data)))
                    .or_else(|err| {
                        warn!("error occurred: {}", err);

                        future::ok::<_, websocket::WebSocketError>(Box::new(stream::empty())
                            as Box<Stream<Item = OwnedMessage, Error = websocket::WebSocketError>>)
                    })
                    .flatten(),
            )
        })
    }))
    .expect("expected websocket connection to complete successfully, but an error occurred");
}

struct Handler<T>
where
    T: screeps_api::TokenStorage,
{
    tokens: T,
    info: screeps_api::MyInfo,
    config: Config,
}

impl<T> Handler<T>
where
    T: screeps_api::TokenStorage,
{
    fn new(tokens: T, info: screeps_api::MyInfo, config: Config) -> Self {
        Handler {
            tokens: tokens,
            info: info,
            config: config,
        }
    }

    fn handle_data(
        &self,
        data: OwnedMessage,
    ) -> Box<Stream<Item = OwnedMessage, Error = websocket::WebSocketError>> {
        match data {
            OwnedMessage::Text(string) => {
                let data = SockjsMessage::parse(&string)
                    .expect("expected a correct SockJS message, found a parse error.");

                match data {
                    SockjsMessage::Open => debug!("SockJS connection opened"),
                    SockjsMessage::Heartbeat => debug!("SockJS heartbeat."),
                    SockjsMessage::Close { .. } => debug!("SockJS close"),
                    SockjsMessage::Message(message) => {
                        return Box::new(self.handle_parsed_message(message));
                    }
                    SockjsMessage::Messages(messages) => {
                        let results = messages
                            .into_iter()
                            .map(|message| Ok(self.handle_parsed_message(message)))
                            .collect::<Vec<_>>();

                        return Box::new(
                            stream::iter_result::<_, _, websocket::WebSocketError>(results)
                                .flatten(),
                        );
                    }
                }
            }
            OwnedMessage::Binary(data) => warn!("ignoring binary data from websocket: {:?}", data),
            OwnedMessage::Close(data) => warn!("connection closing: {:?}", data),
            OwnedMessage::Ping(data) => return Box::new(stream::once(Ok(OwnedMessage::Pong(data)))),
            OwnedMessage::Pong(_) => (),
        }

        Box::new(stream::empty())
    }

    fn handle_parsed_message(
        &self,
        message: screeps_api::websocket::parsing::ScreepsMessage,
    ) -> Box<Stream<Item = OwnedMessage, Error = websocket::WebSocketError>> {
        match message {
            ScreepsMessage::AuthFailed => panic!("authentication with stored token failed!"),
            ScreepsMessage::AuthOk { new_token } => {
                info!(
                    "authentication succeeded, now authenticated as {}.",
                    self.info.username
                );
                // return the token to the token storage in case we need it again - we won't in this example
                // program, but this is a good practice
                //
                // TODO: find an efficient way to do this automatically in the handler.
                self.tokens.return_token(new_token);

                return Box::new(
                    self.config.subscribe_with(&self.info.user_id).chain(
                        stream::futures_unordered(vec![future::lazy(|| {
                            warn!("subscribed!");
                            future::ok::<_, websocket::WebSocketError>(stream::empty())
                        })])
                        .flatten(),
                    ),
                )
                    as Box<Stream<Item = OwnedMessage, Error = websocket::WebSocketError>>;
            }
            ScreepsMessage::ChannelUpdate { update } => {
                self.handle_update(update);
            }
            ScreepsMessage::ServerProtocol { protocol } => {
                info!("server protocol: {}", protocol);
            }
            ScreepsMessage::ServerTime { time } => {
                info!("server time: {}", time);
            }
            ScreepsMessage::ServerPackage { package } => {
                info!("server package: {}", package);
            }
            ScreepsMessage::Other(other) => {
                warn!("ScreepsMessage::Other: {}", other);
            }
        }

        Box::new(stream::empty())
    }

    fn handle_update(&self, update: ChannelUpdate) {
        match update {
            ChannelUpdate::UserCpu { user_id, update } => info!("CPU: [{}] {:#?}", user_id, update),
            ChannelUpdate::RoomMapView {
                room_name,
                shard_name,
                update,
            } => {
                info!(
                    "Map View: [{}/{}] {:?}",
                    shard_name.as_ref().map(AsRef::as_ref).unwrap_or("<None>"),
                    room_name,
                    update
                );
            }
            ChannelUpdate::RoomDetail {
                room_name,
                shard_name,
                update,
            } => {
                debug!(
                    "Room Detail: [{}/{}] {:?}",
                    shard_name.as_ref().map(AsRef::as_ref).unwrap_or("<None>"),
                    room_name,
                    update
                );
                info!(
                    "Room {}/{}: {}",
                    shard_name.as_ref().map(AsRef::as_ref).unwrap_or("<None>"),
                    room_name,
                    serde_json::to_string_pretty(&serde_json::Value::Object(
                        update.objects.iter().cloned().collect()
                    ))
                    .expect("expected to_string to succeed on plain map.")
                );
            }
            ChannelUpdate::NoRoomDetail {
                room_name,
                shard_name,
            } => {
                info!(
                    "Room Skipped: {}/{}",
                    shard_name.as_ref().map(AsRef::as_ref).unwrap_or("<None>"),
                    room_name
                );
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
            ChannelUpdate::UserConversation {
                user_id,
                target_user_id,
                update,
            } => {
                info!(
                    "Conversation update: [{}->{}] {:#?}",
                    user_id, target_user_id, update
                );
            }
            ChannelUpdate::Other { channel, update } => {
                warn!(
                    "ChannelUpdate::Other: {}\n{}",
                    channel,
                    serde_json::to_string_pretty(&update).expect("failed to serialize json string")
                );
            }
        }
    }
}
