rust-screeps-api
================

rust-screeps-api provides a wrapper for the https://screeps.com API, using hyper and serde_json to make the requests.

Screeps is a true programming MMO where users uploading JavaScript code to power their online empires. The game supports one official server, https://screeps.com, as well connecting to the instances [open sourced server](https://github.com/screeps/screeps/) run by users.

The screeps API is unofficial, and mostly scraped from requests the client makes and the server source code, but it is well documented by several different open source projects. In particular, the documentation for the endpoints rust-screeps-api calls can be found at https://github.com/screepers/python-screeps/blob/master/docs/Endpoints.md.

To successfully run tests, please create a `.env` file containing either both `SCREEPS_API_USERNAME` and `SCREEPS_API_PASSWORD`, or `NO_AUTH_TESTS=1`. These may also be provided as environmental variables.
