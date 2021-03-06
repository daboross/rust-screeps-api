rust-screeps-api
================
[![Linux Build Status][travis-image]][travis-builds]
[![Windows Build Status][appveyor-image]][appveyor-builds]

A Rust library for using the [Screeps] HTTP API.

Screeps is a true programming MMO where users uploading JavaScript code to power their online empires.
`rust-screeps-api` can connect to the [official server][screeps], and any [private server][screeps-os] instances run by
users.

`rust-screeps-api` uses [hyper] to run http requests and [serde] to parse json results.

## Usage

```rust
extern crate screeps_api;

use screeps_api::SyncApi;

let mut api = SyncApi::new().unwrap();

api.login("username", "password").unwrap();

let my_info = api.my_info().unwrap();

println!("Logged in with user ID {}!", my_info.user_id);
```

Documentation is located at https://docs.rs/screeps-api/.

Unofficial API documentation for:
- HTTP endpoints at https://github.com/screepers/python-screeps/blob/master/docs/Endpoints.md.
- Websocket endpoints at https://github.com/daboross/rust-screeps-api/blob/master/protocol-docs/websocket.md

## What's implemented

- Logging in
- Getting all leaderboard information
- Getting room terrain
- Checking room status
- Getting room overview info
- Getting logged in user's info
- Getting rooms where PvP recently occurred
- Websocket connections:
  - Getting user CPU and Memory usage each tick
  - Getting a map overview of a room
  - Getting new message notifications
  - Getting console messages
  - Parts of getting room detailed updates
    - Almost all room objects are parsed
    - Flags are parsed
    - RoomVisuals are not parsed

### What isn't implemented

- Market API
- Messaging API
- Detailed user information API
- Game manipulation API
- Room history API
- Parsing room visuals in room socket updates.

## Testing

`rust-screeps-api` has both unit tests for parsing sample results from each endpoint, and integration tests which make calls to the official server.

Environmental variables used when testing:
- SCREEPS_API_USERNAME: the username to log in with for doing authenticated tests
- SCREEPS_API_PASSWORD: the password to login with for doing authenticated tests
- All variables can also be placed into a `.env` file (parsed using the `dotenv` crate)

Use:
- `cargo test` to perform all tests, including calls to https://screeps.com with provided login details.
- `cargo test parse` to only perform parsing unit tests. This can be performed offline.
- `cargo test -- --skip auth` to test both parsing and all unauthenticated calls to the official server.

## Updating / Contributing

To keep working, this library needs to be periodically updated to match the
screeps protocools. Since I might not be doing the updating, I've written up
some barebone notes on how to do that at [CONTRIBUTING.md]

[travis-image]: https://travis-ci.org/daboross/rust-screeps-api.svg?branch=master
[travis-builds]: https://travis-ci.org/daboross/rust-screeps-api
[appveyor-image]: https://ci.appveyor.com/api/projects/status/github/daboross/rust-screeps-api?branch=master&svg=true
[appveyor-builds]: https://ci.appveyor.com/project/daboross/rust-screeps-api
[screeps]: https://screeps.com
[screeps-os]: https://github.com/screeps/screeps/
[hyper]: https://github.com/hyperium/hyper/
[serde]: https://github.com/serde-rs/json/
[`serde_json::Value`]: https://docs.serde.rs/serde_json/value/enum.Value.html
[CONTRIBUTING.md]: https://github.com/daboross/rust-screeps-api/blob/master/CONTRIBUTING.md
