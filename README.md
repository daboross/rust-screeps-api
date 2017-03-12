rust-screeps-api
================
[![Build Status](https://travis-ci.org/daboross/rust-screeps-api.svg?branch=master)][1]

A Rust library for using the [Screeps][2] HTTP API.

Screeps is a true programming MMO where users uploading JavaScript code to power their online empires.
`rust-screeps-api` can connect to the [official server][2], and any [private server][3] instances run by users.

Rust uses [hyper][4] to run http requests, and [serde][5] to parse json results.

## Usage

```rust
let client = hyper::Client::with_connector(
                HttpsConnector::new(hyper_rustls::TlsClient::new()));

let mut api = screeps_api::API::new(&client);

api.login("username", "password").unwrap();

let my_info = api.my_info().unwrap();

println!("Logged in with user ID {}!", my_info.user_id);
```

More comprehensive examples and documentation at https://dabo.guru/rust/screeps-api/.

Unofficial documentation for HTTP endpoints can be found at https://github.com/screepers/python-screeps/blob/master/docs/Endpoints.md.

## What's implemented

- Logging in
- Getting all leaderboard information
- Getting room terrain
- Checking room status
- Getting room overview info
- Getting logged in user's info
- Getting rooms where PvP recently occurred

### What isn't implemented

- Market API
- Messaging API
- Detailed user information API
- Game manipulation API
- Room history API
- Room update / user console websocket API

## Testing

`rust-screeps-api` has both unit tests for parsing sample results from each endpoint, and integration tests which make calls to the official server.

Environmental variables used when testing:
- SCREEPS_API_USERNAME: the username to log in with for doing authenticated tests
- SCREEPS_API_PASSWORD: the password to login with for doing authenticated tests

Use:
- `cargo test` to perform all tests, including calls to https://screeps.com with provided login details.
- `cargo test parse` to only perform parsing unit tests. This can be performed offline.
- `cargo test -- --skip auth` to test both parsing and all unauthenticated calls to the official server.

[1]: https://travis-ci.org/daboross/rust-screeps-api
[2]: https://screeps.com
[3]: https://github.com/screeps/screeps/
[4]: https://github.com/hyperium/hyper/
[5]: https://github.com/serde-rs/json/
