//! Rust wrapper for the Screeps public API.
//!
//! This project aims to allow any rust project to freely call the https://screeps.com API, or any private Screeps
//! server API, and receive typed results that can be easily and safely used within rust.
//!
//! # Usage
//!
//! The `API` struct is the main client api provided by rust-screeps-api. To use it, first create an instance with
//! `API::new(client)` or `API::with_url(client, url)` to connect to a private server.
//!
//! ```
//! # extern crate hyper;
//! extern crate screeps_api;
//! # fn main() {
//! # let hyper_client = hyper::Client::new();
//!
//! let api = screeps_api::API::new(&hyper_client);
//!
//! let mut api = screeps_api::API::with_url(&hyper_client, "https://screeps.com/").expect("expected valid URL");
//! # }
//! ```
//!
//! As you can tell, you'll need to pre-create a hyper client in order to use rust-screeps-api. While it strictly could
//! create a client itself, this way you can provide any backend connection implementation you want, as well as use
//! any of the available https implementations.
//!
//! Here's an example using hyper and hyper-rustls to create a client. hyper-rustls provides a pure-rust https backend,
//! but it, as of yet, is not as well vetted as other projects, such as hyper-openssl.
//!
//! ```
//! extern crate screeps_api;
//! extern crate hyper;
//! extern crate hyper_rustls;
//!
//! use hyper::net::HttpsConnector;
//!
//! # fn main() {
//! let client = hyper::Client::with_connector(HttpsConnector::new(hyper_rustls::TlsClient::new()));
//!
//! let mut api = screeps_api::API::new(&client);
//! # }
//! ```
//!
//! One last thing to note: The screeps API runs on a rotating authentication  token, so the token stored within the
//! API instance is only valid for one call. For this reason, all API requests require mutable access and store the
//! token resulting from the call internally.
//!
//! When making multiple concurrent calls to the API, please make a new API instance for each thread, and provide each
//! with the login details via `login()` separately to obtain multiple tokens.
#![deny(missing_docs)]
#![recursion_limit="512"]
#[macro_use]
extern crate serde_derive;
extern crate hyper;
extern crate serde;
#[cfg_attr(test, macro_use)]
extern crate serde_json;
extern crate time;

pub mod error;
pub mod endpoints;
mod data;

use endpoints::{login, my_info, room_overview, room_terrain, room_status};
pub use endpoints::login::Details as LoginDetails;
pub use error::{Error, Result};
use hyper::header::{Headers, ContentType};
use std::borrow::Cow;

/// A trait for each endpoint
trait EndpointResult: Sized {
    type RequestResult: serde::Deserialize;
    type ErrorResult: serde::Deserialize + Into<Error>;

    fn from_raw(data: Self::RequestResult) -> Result<Self>;
}

/// API Object, stores the current API token and allows access to making requests.
#[derive(Debug)]
pub struct API<'a> {
    /// The base URL for this API instance.
    pub url: hyper::Url,
    /// The current authentication token, in binary UTF8.
    pub token: Option<Vec<u8>>,
    client: &'a hyper::Client,
}

impl<'a> API<'a> {
    /// Creates a new API instance for the official server with the `https://screeps.com/api/` base url.
    ///
    /// The returned stance can be used to make anonymous calls, or `API.login` can be used to allow for
    /// authenticated API calls.
    pub fn new<'b>(client: &'b hyper::Client) -> API<'b> {
        API {
            url: hyper::Url::parse("https://screeps.com/api/").expect("expected pre-set url to parse, parsing failed"),
            client: client,
            token: None,
        }
    }

    /// Creates a new API instance with the given url as the base instead of `https://screeps.com/api/`.
    ///
    /// The returned instance can be used to make anonymous calls, or `API.login` can be used to allow for
    /// authenticated API calls.
    pub fn with_url<'b, T: hyper::client::IntoUrl>(client: &'b hyper::Client, url: T) -> Result<API<'b>> {
        Ok(API {
            url: url.into_url()?,
            client: client,
            token: None,
        })
    }

    /// Makes a POST request to the given endpoint URL, with the given data encoded as JSON in the body of the request.
    fn make_post_request<T: serde::Serialize, R: EndpointResult>(&mut self,
                                                                 endpoint: &str,
                                                                 request_text: T)
                                                                 -> Result<R> {
        let body = serde_json::to_string(&request_text)?;

        let mut headers = Headers::new();
        headers.set(ContentType::json());
        if let Some(ref token) = self.token {
            headers.set_raw("X-Token", vec![token.clone()]);
            headers.set_raw("X-Username", vec![token.clone()]);
        }

        let mut response = self.client
            .post(self.url.join(endpoint)?)
            .body(&body)
            .headers(headers)
            .send()?;

        if !response.status.is_success() {
            return Err(Error::with_url(response.status, Some(response.url.clone())));
        }

        if let Some(token_vec) = response.headers.get_raw("X-Token") {
            if let Some(token_bytes) = token_vec.first() {
                self.token = Some(Vec::from(&**token_bytes));
            }
        }

        let json: serde_json::Value = match serde_json::from_reader(&mut response) {
            Ok(v) => v,
            Err(e) => return Err(Error::with_url(e, Some(response.url.clone()))),
        };

        use serde::Deserialize;

        let result = match R::RequestResult::deserialize(&json) {
            Ok(v) => v,
            Err(e) => {
                match R::ErrorResult::deserialize(&json) {
                    Ok(v) => return Err(Error::with_json(v, Some(response.url.clone()), Some(json))),
                    // Favor the primary parsing error if one occurs parsing the error type as well.
                    Err(_) => return Err(Error::with_json(e, Some(response.url.clone()), Some(json))),
                }
            }
        };

        R::from_raw(result)
    }

    /// Makes a new GET request to the given endpoint URL, with given the query parameters added to the end.
    fn make_get_request<R>(&mut self, endpoint: &str, query_pairs: Option<&[(&str, String)]>) -> Result<R>
        where R: EndpointResult
    {
        let mut headers = Headers::new();
        headers.set(ContentType::json());
        if let Some(ref token) = self.token {
            headers.set_raw("X-Token", vec![token.clone()]);
            headers.set_raw("X-Username", vec![token.clone()]);
        }

        let mut url = self.url.join(endpoint)?;

        if let Some(pairs) = query_pairs {
            url.query_pairs_mut().extend_pairs(pairs).finish();
        }

        let mut response = self.client
            .get(url)
            .headers(headers)
            .send()?;

        if !response.status.is_success() {
            return Err(Error::with_url(response.status, Some(response.url.clone())));
        }

        if let Some(token_vec) = response.headers.get_raw("X-Token") {
            if let Some(token_bytes) = token_vec.first() {
                self.token = Some(Vec::from(&**token_bytes));
            }
        }

        let json: serde_json::Value = match serde_json::from_reader(&mut response) {
            Ok(v) => v,
            Err(e) => return Err(Error::with_url(e, Some(response.url.clone()))),
        };

        use serde::Deserialize;

        let result = match R::RequestResult::deserialize(&json) {
            Ok(v) => v,
            Err(e) => {
                match R::ErrorResult::deserialize(&json) {
                    Ok(v) => return Err(Error::with_json(v, Some(response.url.clone()), Some(json))),
                    // Favor the primary parsing error if one occurs parsing the error type as well.
                    Err(_) => return Err(Error::with_json(e, Some(response.url.clone()), Some(json))),
                }
            }
        };

        R::from_raw(result)
    }

    /// Logs in using a given username and password, and stores the resulting token inside this structure.
    pub fn login(&mut self, login_details: &LoginDetails) -> Result<()> {
        let result: login::LoginResult = self.make_post_request("auth/signin", login_details)?;

        self.token = Some(result.token.into_bytes());
        Ok(())
    }

    /// Gets user information on the user currently logged in, including username and user id.
    pub fn my_info(&mut self) -> Result<my_info::MyInfo> { self.make_get_request("auth/me", None) }

    /// Gets the overview of a room, returning totals for usually 3 intervals, 8, 180 and 1440, representing
    /// data for the past hour, data for the past 24 hours, and data for the past week respectively.
    ///
    /// All Allowed request_intervals are not known, but at least `8`, `180` and `1440` are allowed. The returned data,
    /// at the time of writing, includes 8 data points of each type, representing equal portions of the time period
    /// requested (hour for `8`, day for `180`, week for `1440`).
    pub fn room_overview<'b, T>(&mut self, room_name: T, request_interval: u32) -> Result<room_overview::RoomOverview>
        where T: Into<Cow<'b, str>>
    {
        self.make_get_request("game/room-overview",
                              Some(&[("room", room_name.into().into_owned()),
                                     ("interval", request_interval.to_string())]))
    }

    /// Gets the terrain of a room, returning a 2d array of 50x50 points.
    ///
    /// Does not require authentication.
    pub fn room_terrain<'b, T>(&mut self, room_name: T) -> Result<room_terrain::RoomTerrain>
        where T: Into<Cow<'b, str>>
    {
        self.make_get_request("game/room-terrain",
                              Some(&[("room", room_name.into().into_owned()), ("encoded", true.to_string())]))
    }

    /// Gets the "status" of a room: if it is open, if it is in a novice area, if it exists.
    pub fn room_status<'b, T>(&mut self, room_name: T) -> Result<room_status::RoomStatus>
        where T: Into<Cow<'b, str>>
    {
        self.make_get_request("game/room-status",
                              Some(&[("room", room_name.into().into_owned())]))
    }
}

#[cfg(test)]
mod tests {
    use API;
    extern crate hyper;
    extern crate hyper_rustls;
    use hyper::client::Client;
    use hyper::net::HttpsConnector;

    #[test]
    fn anonymous_creation() {
        let client = Client::with_connector(HttpsConnector::new(hyper_rustls::TlsClient::new()));
        let _unused = API::new(&client);
    }
}
