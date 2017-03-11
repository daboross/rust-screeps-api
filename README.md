rust-screeps-api
================
[![Build Status](https://travis-ci.org/daboross/rust-screeps-api.svg?branch=master)](https://travis-ci.org/daboross/rust-screeps-api)

A wrapper for the Screeps public API.

Screeps is a true programming MMO where users uploading JavaScript code to power their online empires. The game supports one official
server, https://screeps.com, as well connecting to the instances [open sourced server](https://github.com/screeps/screeps/) run by users.

rust-screeps-api provides a rust wrapper for the unofficial Screeps API, making requests using [hyper](https://github.com/hyperium/hyper)
and [serde_json](https://github.com/serde-rs/json) to create queries and parse responses.

While the API endpoints are not officially recognized, they are not obfuscated and can be used freely on both the official server and on private servers.
Documentation for all known endpoints can be found at https://github.com/screepers/python-screeps/blob/master/docs/Endpoints.md.

Documentation for this crate can be found at https://dabo.guru/rust/screeps-api/screeps_api/

Environmental variables used when testing:
- SCREEPS_API_USERNAME: the username to log in with for doing authenticated tests
- SCREEPS_API_PASSWORD: the password to login with for doing authenticated tests

To control testing, use:
- `cargo test parse`: only test sample result parsing, no HTTP calls
- `cargo test -- --skip auth`: only test parsing and unauthenticated calls, nothing requiring login in.
