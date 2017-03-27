//! Rust wrapper for the Screeps public API.
//!
//! Allows rust programs to retrieve game information from https://screeps.com and any private Screeps server.
//!
//! # Usage
//!
//! To start, create a hyper client and an `API` instance. `API` keeps track of the base API url and the current
//! authentication token.
//!
//! `screeps-api` requires a separate hyper client in order to let you choose your own SSL crate. `rustls` is
//! pure rust, but `openssl` is much more vetted.
//!
//! ```
//! extern crate hyper;
//! extern crate screeps_api;
//! extern crate hyper_rustls;
//!
//! use hyper::net::HttpsConnector;
//!
//! # fn main() {
//! let client = hyper::Client::with_connector(
//!         HttpsConnector::new(hyper_rustls::TlsClient::new()));
//!
//! let mut api = screeps_api::API::new(&client);
//! # }
//! ```
//!
//! This API object can then be used to make any number of API calls. Each will return a `Result` with a typed response
//! or an error. All calls require mutable access (more info below).
//!
//! ```no_run
//! # extern crate hyper;
//! # extern crate screeps_api;
//! # extern crate hyper_rustls;
//! #
//! # use hyper::net::HttpsConnector;
//! #
//! # fn main() {
//! #   let client = hyper::Client::with_connector(
//! #           HttpsConnector::new(hyper_rustls::TlsClient::new()));
//!
//! let mut api = screeps_api::API::new(&client);
//!
//! api.login("username", "password").unwrap();
//!
//! let my_info = api.my_info().unwrap();
//!
//! println!("Logged in with user ID {}!", my_info.user_id);
//!
//! # }
//! ```
//!
//! # Multiple clients
//!
//! Unlike hyper, screeps-api clients can not be used in multiple thread simultaneously. The reason for this is Screep's
//! authentication model, a rotating token. Each auth token can only be used for one call, and that call will return
//! the auth new token.
//!
//! For this reason, all API calls made require mutable access to the `API` structure, and if you want to call the API
//! in multiple thread simultaneously, you need to create and log in to multiple `API` structures.
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
pub mod data;

pub use endpoints::{MyInfo, RecentPvp, RoomOverview, RoomStatus, RoomTerrain};
use endpoints::{login, my_info, room_overview, room_terrain, room_status, recent_pvp, leaderboard};
pub use endpoints::leaderboard::LeaderboardType;
pub use endpoints::recent_pvp::PvpArgs as RecentPvpDetails;
pub use error::{Error, Result};

use hyper::header::{Headers, ContentType};
use std::borrow::Cow;

/// A trait for each endpoint
trait EndpointResult: Sized {
    type RequestResult: serde::Deserialize;
    type ErrorResult: serde::Deserialize + Into<Error>;

    fn from_raw(data: Self::RequestResult) -> Result<Self>;
}

/// A generic trait over hyper's Client which allows for references, owned clients, and Arc<hyper::Client>
/// to be used.
pub trait HyperClient {
    /// Get a reference to this client.
    fn client(&self) -> &hyper::Client;
}

impl HyperClient for hyper::Client {
    fn client(&self) -> &hyper::Client { self }
}

impl<'a> HyperClient for &'a hyper::Client {
    fn client(&self) -> &hyper::Client { self }
}

impl HyperClient for ::std::sync::Arc<hyper::Client> {
    fn client(&self) -> &hyper::Client { &self }
}

/// API Object, stores the current API token and allows access to making requests.
#[derive(Debug)]
pub struct API<T>
    where T: HyperClient
{
    /// The base URL for this API instance.
    pub url: hyper::Url,
    /// The current authentication token, in binary UTF8.
    pub token: Option<Vec<u8>>,
    client: T,
}

impl<T> API<T>
    where T: HyperClient
{
    /// Creates a new API instance for the official server with the `https://screeps.com/api/` base url.
    ///
    /// The returned stance can be used to make anonymous calls, or `API.login` can be used to allow for
    /// authenticated API calls.
    pub fn new(client: T) -> Self {
        API {
            url: hyper::Url::parse("https://screeps.com/api/").expect("expected pre-set url to parse, parsing failed"),
            client: client.into(),
            token: None,
        }
    }

    /// Creates a new API instance with the given url as the base instead of `https://screeps.com/api/`.
    ///
    /// The returned instance can be used to make anonymous calls, or `API.login` can be used to allow for
    /// authenticated API calls.
    pub fn with_url<U: hyper::client::IntoUrl>(client: T, url: U) -> Result<Self> {
        Ok(API {
            url: url.into_url()?,
            client: client,
            token: None,
        })
    }

    /// Makes a POST request to the given endpoint URL, with the given data encoded as JSON in the body of the request.
    fn make_post_request<U: serde::Serialize, R: EndpointResult>(&mut self,
                                                                 endpoint: &str,
                                                                 request_text: U)
                                                                 -> Result<R> {
        let body = serde_json::to_string(&request_text)?;

        let mut headers = Headers::new();
        headers.set(ContentType::json());
        if let Some(ref token) = self.token {
            headers.set_raw("X-Token", vec![token.clone()]);
            headers.set_raw("X-Username", vec![token.clone()]);
        }

        let mut response = self.client
            .client()
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
            .client()
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
    pub fn login<'b, U, V>(&mut self, username: U, password: V) -> Result<()>
        where U: Into<Cow<'b, str>>,
              V: Into<Cow<'b, str>>
    {
        let result: login::LoginResult =
            self.make_post_request("auth/signin", login::Details::new(username, password))?;

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
    pub fn room_overview<'b, U>(&mut self, room_name: U, request_interval: u32) -> Result<room_overview::RoomOverview>
        where U: Into<Cow<'b, str>>
    {
        self.make_get_request("game/room-overview",
                              Some(&[("room", room_name.into().into_owned()),
                                     ("interval", request_interval.to_string())]))
    }

    /// Gets the terrain of a room, returning a 2d array of 50x50 points.
    ///
    /// Does not require authentication.
    pub fn room_terrain<'b, U>(&mut self, room_name: U) -> Result<room_terrain::RoomTerrain>
        where U: Into<Cow<'b, str>>
    {
        self.make_get_request("game/room-terrain",
                              Some(&[("room", room_name.into().into_owned()), ("encoded", true.to_string())]))
    }

    /// Gets the "status" of a room: if it is open, if it is in a novice area, if it exists.
    pub fn room_status<'b, U>(&mut self, room_name: U) -> Result<room_status::RoomStatus>
        where U: Into<Cow<'b, str>>
    {
        self.make_get_request("game/room-status",
                              Some(&[("room", room_name.into().into_owned())]))
    }

    /// Experimental endpoint to get all rooms in which PvP has recently occurred, or where PvP has occurred since a
    /// certain game tick.
    pub fn recent_pvp(&mut self, details: RecentPvpDetails) -> Result<recent_pvp::RecentPvp> {
        let args = match details {
            recent_pvp::PvpArgs::WithinLast { ticks } => [("interval", ticks.to_string())],
            recent_pvp::PvpArgs::Since { time } => [("start", time.to_string())],
        };

        self.make_get_request("experimental/pvp", Some(&args))
    }

    /// Gets a list of all past leaderboard seasons, with end dates, display names, and season ids for each season.
    ///
    /// Seasons are a way of having limited time periods (usually one month) in which all rankings are reset at the
    /// beginning of, and points earned during the time period contribute to a player's rank in that season.
    ///
    /// This method does not return any actual data, but rather just a list of valid past season, any of the ids of
    /// which can then be used to retrieve more information.
    pub fn leaderboard_season_list(&mut self) -> Result<Vec<leaderboard::season_list::LeaderboardSeason>> {
        self.make_get_request("leaderboard/seasons", None)
    }

    /// Finds the rank of a user in a specific season for a specific leaderboard type.
    ///
    /// Will return `ApiError::UserNotFound` when the username does not exist, and `ApiError::ResultNotFound`
    /// when the user exists but does not have a rank for the given season. The user will not have a rank when either
    /// the account did not exist when the season ended, or the user either processed no power or upgraded no
    /// controllers, during the specific leaderboard season.
    ///
    /// This is technically the same API endpoint as find_leaderboard_rank, but the result format differs when
    /// requesting a specific season from when requesting all season ranks.
    pub fn find_season_leaderboard_rank<'b, U, V>(&mut self,
                                                  leaderboard_type: LeaderboardType,
                                                  username: U,
                                                  season: V)
                                                  -> Result<leaderboard::find_rank::FoundUserRank>
        where U: Into<Cow<'b, str>>,
              V: Into<Cow<'b, str>>
    {
        self.make_get_request("leaderboard/find",
                              Some(&[("mode", leaderboard_type.api_representation().to_string()),
                                     ("season", season.into().into_owned()),
                                     ("username", username.into().into_owned())]))
    }

    /// Finds the rank of a user for all seasons for a specific leaderboard type.
    ///
    /// This will return `ApiError::UserNotFound` if a username does not exist, and may also return an empty `Vec` as
    /// the result if the user does not have any ranks in the given leaderboard type (they have never contributed any
    /// global control points, or processed power, depending on the type).
    pub fn find_leaderboard_ranks<'b, U>(&mut self,
                                         leaderboard_type: LeaderboardType,
                                         username: U)
                                         -> Result<Vec<leaderboard::find_rank::FoundUserRank>>
        where U: Into<Cow<'b, str>>
    {
        self.make_get_request("leaderboard/find",
                              Some(&[("mode", leaderboard_type.api_representation().to_string()),
                                     ("username", username.into().into_owned())]))
    }

    /// Gets a page of the leaderboard for a given season.
    ///
    /// Limit dictates how many users will be returned, maximum is 20. Higher than that will cause an InvalidParameters
    /// error message.
    ///
    /// Offset doesn't have to be a multiple of limit, but it's most likely most useful that it is. Offset 0 will get
    /// you the start/top of the ranked list.
    pub fn leaderboard_page<'b, U>(&mut self,
                                   leaderboard_type: LeaderboardType,
                                   season: U,
                                   limit: u32,
                                   offset: u32)
                                   -> Result<leaderboard::page::LeaderboardPage>
        where U: Into<Cow<'b, str>>
    {
        self.make_get_request("leaderboard/list",
                              Some(&[("mode", leaderboard_type.api_representation().to_string()),
                                     ("season", season.into().into_owned()),
                                     ("limit", limit.to_string()),
                                     ("offset", offset.to_string())]))
    }
}


/// Calculates GCL, given GCL points.
#[inline]
pub fn gcl_calc(gcl_points: u64) -> u64 {
    const GCL_INV_MULTIPLY: f64 = 1.0 / 1_000_000f64;
    const GCL_INV_POW: f64 = 1.0 / 2.4f64;

    ((gcl_points as f64) * GCL_INV_MULTIPLY)
        .powf(GCL_INV_POW)
        .floor() as u64 + 1
}


#[cfg(test)]
mod tests {
    use super::gcl_calc;

    #[test]
    fn parse_gcl_1() {
        assert_eq!(gcl_calc(0), 1);
        assert_eq!(gcl_calc(900_000), 1);
    }

    #[test]
    fn parse_gcl_2() {
        assert_eq!(gcl_calc(1_000_000), 2);
        assert_eq!(gcl_calc(5_278_000), 2);
    }

    #[test]
    fn parse_gcl_3() {
        assert_eq!(gcl_calc(5_278_032), 3);
    }

    #[test]
    fn parse_gcl_late_15() {
        assert_eq!(gcl_calc(657_254_041), 15);
    }
}
