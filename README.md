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

To run tests, please create a `.env` file containing either both `SCREEPS_API_USERNAME` and `SCREEPS_API_PASSWORD`, or `NO_AUTH_TESTS=1`. These may also be provided as environmental variables.
