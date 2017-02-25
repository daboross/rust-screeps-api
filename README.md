rust-screeps-api
================

rust-screeps-api provides a wrapper for the https://screeps.com API, using hyper and serde_json to make the requests.

Screeps is a true programming MMO where users uploading JavaScript code to power their online empires. The game supports one official server, https://screeps.com, as well connecting to the instances [open sourced server](https://github.com/screeps/screeps/) run by users.

The screeps API is unofficial, and mostly scraped from requests the client makes and the server source code, but it is well documented by several different open source projects. In particular, the documentation for the endpoints rust-screeps-api calls can be found at https://github.com/screepers/python-screeps/blob/master/docs/Endpoints.md.

To successfully run the tests, please provide SCREEPS_API_USERNAME and SCREEPS_API_PASSWORD in the `.env` file or as environmental variables. These variables are used only in integration testing code, in order to test the library's ability to connect to the official server. A screeps account with an active subscription is not needed to run these tests, any account at all will do.
