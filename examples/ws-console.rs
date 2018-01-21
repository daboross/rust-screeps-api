// .env parsing

extern crate dotenv;
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

use screeps_api::TokenStorage;
use screeps_api::websocket::{Channel, ChannelUpdate, ScreepsMessage, SockjsMessage, UserConsoleUpdate};

static CONSOLE_LOG_TARGET: &'static str = "log:console";
static CONSOLE_RAW_OUTPUT_TARGET: &'static str = "log:console-raw";

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
        Ok(value) => if !value.is_empty() {
            value.into()
        } else {
            default.into()
        },
        Err(_) => default.into(),
    }
}

fn setup_logging() {
    fern::Dispatch::new()
        .level(log::LevelFilter::Info)
        .level_for("rustls", log::LevelFilter::Warn)
        .level_for("hyper", log::LevelFilter::Warn)
        .level_for("screeps_api::connecting", log::LevelFilter::Error)
        .format(|out, message, record| {
            let now = chrono::Local::now();

            if record.level() == log::Level::Info && record.target() == CONSOLE_LOG_TARGET {
                out.finish(format_args!("[{}]{}", now.format("%H:%M:%S"), message));
            } else if record.level() == log::Level::Info && record.target() == CONSOLE_RAW_OUTPUT_TARGET {
                out.finish(format_args!("{}", message));
            } else {
                out.finish(
                    format_args!("[{}][{}] {}: {}", now.format("%H:%M:%S"), record.level(), record.target(), message),
                );
            }
        })
        .chain(std::io::stdout())
        .apply()
        // if we've already set a logger, it's OK.
        .unwrap_or(());
}

fn subscribe_with(id: &str) -> Box<Stream<Item = OwnedMessage, Error = websocket::WebSocketError>> {
    use screeps_api::websocket::subscribe;

    let messages = vec![
        subscribe(&Channel::ServerMessages),
        subscribe(&Channel::user_console(id)),
    ];

    Box::new(stream::iter_ok(
        messages.into_iter().map(OwnedMessage::Text),
    ))
}

fn server_url() -> Cow<'static, str> {
    opt_env("SCREEPS_API_URL", screeps_api::DEFAULT_OFFICIAL_API_URL)
}

fn main() {
    setup_logging();

    let http_url = server_url();
    let token_storage = screeps_api::RcTokenStorage::default();

    let mut client = screeps_api::SyncConfig::new()
        .unwrap()
        .url(&http_url)
        .tokens(token_storage.clone())
        .build()
        .unwrap();

    // Login using the API client - this will storage the auth token in token_storage.
    client
        .login(env("SCREEPS_API_USERNAME"), env("SCREEPS_API_PASSWORD"))
        .expect("login failed");

    let my_info = client.my_info().expect("my_info call failed");

    info!("connecting - {}", my_info.username);

    let mut core = tokio_core::reactor::Core::new().expect("expected IO core to start up without issue.");

    let handle = core.handle();

    let ws_url = screeps_api::websocket::connecting::transform_url(&http_url)
        .expect("expected server api url to parse into websocket url");

    let connection = websocket::ClientBuilder::from_url(&ws_url).async_connect(None, &handle);

    core.run(connection.then(|result| {
        let (client, _) = result.expect("connecting to server failed");

        let (sink, stream) = client.split();

        sink.send(OwnedMessage::Text(screeps_api::websocket::authenticate(
            &token_storage.take_token().unwrap(),
        ))).and_then(|sink| {
            let handler = Handler::new(token_storage, my_info);

            sink.send_all(
                stream
                    .and_then(move |data| future::ok(handler.handle_data(data)))
                    .or_else(|err| {
                        warn!("IO error: {}", err);

                        future::ok::<_, websocket::WebSocketError>(Box::new(stream::empty())
                            as Box<Stream<Item = OwnedMessage, Error = websocket::WebSocketError>>)
                    })
                    .flatten(),
            )
        })
    })).expect("websocket connection exited with failure");
}

struct Handler<T>
where
    T: screeps_api::TokenStorage,
{
    tokens: T,
    info: screeps_api::MyInfo,
}

impl<T> Handler<T>
where
    T: screeps_api::TokenStorage,
{
    fn new(tokens: T, info: screeps_api::MyInfo) -> Self {
        Handler {
            tokens: tokens,
            info: info,
        }
    }

    fn handle_data(&self, data: OwnedMessage) -> Box<Stream<Item = OwnedMessage, Error = websocket::WebSocketError>> {
        match data {
            OwnedMessage::Text(string) => {
                let data = SockjsMessage::parse(&string).expect("expected a SockJS message");

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
                            .map(|message| self.handle_parsed_message(message))
                            .collect::<Vec<_>>();

                        return Box::new(stream::iter_ok::<_, websocket::WebSocketError>(results).flatten());
                    }
                }
            }
            OwnedMessage::Binary(data) => warn!("ignoring binary data from websocket: {:?}", data),
            OwnedMessage::Close(data) => debug!("connection closing: {:?}", data),
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
                info!("connected - {}", self.info.username);

                self.tokens.return_token(new_token);

                return Box::new(
                    subscribe_with(&self.info.user_id).chain(
                        stream::futures_unordered(vec![
                            future::lazy(|| {
                                info!("subscribed");
                                future::ok::<_, websocket::WebSocketError>(stream::empty())
                            }),
                        ]).flatten(),
                    ),
                ) as Box<Stream<Item = OwnedMessage, Error = websocket::WebSocketError>>;
            }
            ScreepsMessage::ChannelUpdate { update } => {
                self.handle_update(update);
            }
            ScreepsMessage::ServerProtocol { protocol } => {
                debug!("server protocol: {}", protocol);
            }
            ScreepsMessage::ServerTime { time } => {
                debug!("server time: {}", time);
            }
            ScreepsMessage::ServerPackage { package } => {
                debug!("server package: {}", package);
            }
            ScreepsMessage::Other(other) => {
                warn!("ScreepsMessage::Other: {}", other);
            }
        }

        Box::new(stream::empty())
    }

    fn handle_update(&self, update: ChannelUpdate) {
        match update {
            ChannelUpdate::UserConsole { user_id, update } => {
                assert_eq!(user_id, self.info.user_id);
                match update {
                    UserConsoleUpdate::Messages {
                        log_messages,
                        result_messages,
                        shard: None,
                    } => {
                        for message in &log_messages {
                            info!(target: CONSOLE_LOG_TARGET, " {}", message);
                        }
                        for message in &result_messages {
                            info!(target: CONSOLE_RAW_OUTPUT_TARGET, "{}", message);
                        }
                    }
                    UserConsoleUpdate::Messages {
                        log_messages,
                        result_messages,
                        shard: Some(shard),
                    } => {
                        for message in &log_messages {
                            info!(target: CONSOLE_LOG_TARGET, "[{}] {}", shard, message);
                        }
                        for message in &result_messages {
                            info!(target: CONSOLE_RAW_OUTPUT_TARGET, "{}: {}", shard, message);
                        }
                    }
                    UserConsoleUpdate::Error {
                        message,
                        shard: None,
                    } => {
                        info!(target: CONSOLE_LOG_TARGET, " {}", message);
                    }
                    UserConsoleUpdate::Error {
                        message,
                        shard: Some(shard),
                    } => {
                        info!(target: CONSOLE_LOG_TARGET, "[{}:ERROR] {}", shard, message);
                    }
                }
            }
            other => {
                warn!("Unexpected update: {:?}", other);
            }
        }
    }
}
