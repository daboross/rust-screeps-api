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
extern crate log;
extern crate hyper;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate tuple_vec_map;
#[cfg_attr(test, macro_use)]
extern crate serde_json;
extern crate time;

pub mod error;
pub mod endpoints;
pub mod data;

pub use endpoints::{MyInfo, RecentPvp, RoomOverview, RoomStatus, RoomTerrain, MapStats};
pub use endpoints::leaderboard::LeaderboardType;
pub use endpoints::recent_pvp::PvpArgs as RecentPvpDetails;
pub use error::{Error, Result};

use std::borrow::Cow;
use std::convert::AsRef;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::rc::Rc;
use std::cell::RefCell;

use hyper::header::{Headers, ContentType};

use endpoints::{login, my_info, room_overview, room_terrain, room_status, recent_pvp, leaderboard, map_stats};

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

/// A generic trait over some storage for auth tokens, possibly for use with sharing tokens between clients.
pub trait TokenStorage {
    /// Takes a token from the token storage, if there are any tokens.
    fn take_token(&mut self) -> Option<Vec<u8>>;

    /// Gives a new token back to the token storage.
    fn return_token(&mut self, Vec<u8>);
}

impl HyperClient for hyper::Client {
    fn client(&self) -> &hyper::Client {
        self
    }
}

impl<'a> HyperClient for &'a hyper::Client {
    fn client(&self) -> &hyper::Client {
        self
    }
}

impl HyperClient for Arc<hyper::Client> {
    fn client(&self) -> &hyper::Client {
        &self
    }
}

impl HyperClient for Rc<hyper::Client> {
    fn client(&self) -> &hyper::Client {
        &self
    }
}

impl TokenStorage for Option<Vec<u8>> {
    fn take_token(&mut self) -> Option<Vec<u8>> {
        self.take()
    }

    fn return_token(&mut self, token: Vec<u8>) {
        *self = Some(token);
    }
}

impl TokenStorage for VecDeque<Vec<u8>> {
    fn take_token(&mut self) -> Option<Vec<u8>> {
        self.pop_front()
    }

    fn return_token(&mut self, token: Vec<u8>) {
        self.push_back(token)
    }
}

impl<T: TokenStorage> TokenStorage for Arc<Mutex<T>> {
    fn take_token(&mut self) -> Option<Vec<u8>> {
        self.lock().unwrap().take_token()
    }

    fn return_token(&mut self, token: Vec<u8>) {
        self.lock().unwrap().return_token(token)
    }
}

impl<T: TokenStorage> TokenStorage for Rc<RefCell<T>> {
    fn take_token(&mut self) -> Option<Vec<u8>> {
        self.borrow_mut().take_token()
    }

    fn return_token(&mut self, token: Vec<u8>) {
        self.borrow_mut().return_token(token)
    }
}

/// API Object, stores the current API token and allows access to making requests.
#[derive(Debug)]
pub struct API<C: HyperClient = hyper::Client, T: TokenStorage = Option<Vec<u8>>> {
    /// The base URL for this API instance.
    pub url: hyper::Url,
    /// The current authentication token, in binary UTF8.
    pub token: T,
    client: C,
}

fn default_url() -> hyper::Url {
    hyper::Url::parse("https://screeps.com/api/").expect("expected pre-set url to parse, parsing failed")
}

impl<C: HyperClient> API<C, Option<Vec<u8>>> {
    /// Creates a new API instance for the official server with the `https://screeps.com/api/` base url.
    ///
    /// The returned instance can be used to make anonymous calls, or `API.login` can be used to allow for
    /// authenticated API calls.
    pub fn new(client: C) -> Self {
        API {
            url: default_url(),
            client: client,
            token: None,
        }
    }

    /// Creates a new API instance with the given url as the base instead of `https://screeps.com/api/`.
    ///
    /// The returned instance can be used to make anonymous calls, or `API.login` can be used to allow for
    /// authenticated API calls.
    pub fn with_url<U: hyper::client::IntoUrl>(client: C, url: U) -> Result<Self> {
        let api = API {
            url: url.into_url()?,
            client: client,
            token: Default::default(),
        };
        Ok(api)
    }
}

impl<C: HyperClient, T: TokenStorage> API<C, T> {
    /// Creates a new API instance for the official server with a stored token.
    ///
    /// The returned instance can be used to make both anonymous calls, and authenticated calls, provided
    /// the token is valid.
    pub fn with_token(client: C, token: T) -> Self {
        API {
            url: default_url(),
            client: client,
            token: token,
        }
    }

    /// Creates a new API instance with the given url as the base instead of `https://screeps.com/api/`, and
    /// with a stored token for that url.
    ///
    /// The returned instance can be used to make anonymous calls and will be allowed to make authenticated calls
    /// if the token is valid.
    pub fn with_token_and_url<U: hyper::client::IntoUrl>(client: C, token: T, url: U) -> Result<Self> {
        let api = API {
            url: url.into_url()?,
            client: client,
            token: token,
        };
        Ok(api)
    }

    /// Starts preparing a POST or GET request to the given endpoint URL
    #[inline]
    fn request<'a, S: serde::Serialize>(&'a mut self, endpoint: &'a str) -> PartialRequest<'a, C, T, S> {
        PartialRequest {
            client: self,
            endpoint: endpoint,
            post_body: None,
            query_params: None,
            auth_required: false,
        }
    }

    /// Makes a new GET request to the given endpoint URL, with given the query parameters added to the end.
    #[inline]
    fn get<'a>(&'a mut self, endpoint: &'a str) -> PartialRequest<'a, C, T, &'static str> {
        self.request(endpoint)
    }


    /// Makes a POST request to the given endpoint URL, with the given data encoded as JSON in the body of the request.
    #[inline]
    fn post<'a, U: serde::Serialize>(&'a mut self, endpoint: &'a str, request_text: U) -> PartialRequest<'a, C, T, U> {
        self.request(endpoint).post(request_text)
    }

    /// Logs in using a given username and password, and stores the resulting token inside this structure.
    pub fn login<'b, U, V>(&mut self, username: U, password: V) -> Result<()>
        where U: Into<Cow<'b, str>>,
              V: Into<Cow<'b, str>>
    {
        let result: login::LoginResult = self.post("auth/signin", login::Details::new(username, password)).send()?;

        self.token.return_token(result.token.into_bytes());
        Ok(())
    }

    /// Gets user information on the user currently logged in, including username and user id.
    pub fn my_info(&mut self) -> Result<my_info::MyInfo> {
        self.get("auth/me").auth(true).send()
    }

    /// Get information on a number of rooms.
    pub fn map_stats<'a, U, V>(&mut self, rooms: &'a V) -> Result<map_stats::MapStats>
        where U: AsRef<str>,
              &'a V: IntoIterator<Item = U>
    {
        // TODO: interpret for different stats.
        let args = map_stats::MapStatsArgs::new(rooms, map_stats::StatName::RoomOwner);

        self.post("game/map-stats", args)
            .auth(true)
            .send()
    }

    /// Gets the overview of a room, returning totals for usually 3 intervals, 8, 180 and 1440, representing
    /// data for the past hour, data for the past 24 hours, and data for the past week respectively.
    ///
    /// All Allowed request_intervals are not known, but at least `8`, `180` and `1440` are allowed. The returned data,
    /// at the time of writing, includes 8 data points of each type, representing equal portions of the time period
    /// requested (hour for `8`, day for `180`, week for `1440`).
    pub fn room_overview<'b, U>(&mut self, room_name: U, request_interval: u32) -> Result<room_overview::RoomOverview>
        where U: Into<Cow<'b, str>>
    {
        self.get("game/room-overview")
            .params(&[("room", room_name.into().into_owned()), ("interval", request_interval.to_string())])
            .auth(true)
            .send()
    }

    /// Gets the terrain of a room, returning a 2d array of 50x50 points.
    ///
    /// Does not require authentication.
    pub fn room_terrain<'b, U>(&mut self, room_name: U) -> Result<room_terrain::RoomTerrain>
        where U: Into<Cow<'b, str>>
    {
        self.get("game/room-terrain")
            .params(&[("room", room_name.into().into_owned()), ("encoded", true.to_string())])
            .auth(false)
            .send()
    }

    /// Gets the "status" of a room: if it is open, if it is in a novice area, if it exists.
    pub fn room_status<'b, U>(&mut self, room_name: U) -> Result<room_status::RoomStatus>
        where U: Into<Cow<'b, str>>
    {
        self.get("game/room-status").params(&[("room", room_name.into().into_owned())]).auth(true).send()
    }

    /// Experimental endpoint to get all rooms in which PvP has recently occurred, or where PvP has occurred since a
    /// certain game tick.
    pub fn recent_pvp(&mut self, details: RecentPvpDetails) -> Result<recent_pvp::RecentPvp> {
        let args = match details {
            recent_pvp::PvpArgs::WithinLast { ticks } => [("interval", ticks.to_string())],
            recent_pvp::PvpArgs::Since { time } => [("start", time.to_string())],
        };

        self.get("experimental/pvp").params(&args).auth(false).send()
    }

    /// Gets a list of all past leaderboard seasons, with end dates, display names, and season ids for each season.
    ///
    /// Seasons are a way of having limited time periods (usually one month) in which all rankings are reset at the
    /// beginning of, and points earned during the time period contribute to a player's rank in that season.
    ///
    /// This method does not return any actual data, but rather just a list of valid past season, any of the ids of
    /// which can then be used to retrieve more information.
    pub fn leaderboard_season_list(&mut self) -> Result<Vec<leaderboard::season_list::LeaderboardSeason>> {
        self.get("leaderboard/seasons").auth(true).send()
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
        self.get("leaderboard/find")
            .auth(true)
            .params(&[("mode", leaderboard_type.api_representation().to_string()),
                      ("season", season.into().into_owned()),
                      ("username", username.into().into_owned())])
            .send()
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
        self.get("leaderboard/find")
            .auth(true)
            .params(&[("mode", leaderboard_type.api_representation().to_string()),
                      ("username", username.into().into_owned())])
            .send()
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
        self.get("leaderboard/list")
            .auth(true)
            .params(&[("mode", leaderboard_type.api_representation().to_string()),
                      ("season", season.into().into_owned()),
                      ("limit", limit.to_string()),
                      ("offset", offset.to_string())])
            .send()
    }
}

struct PartialRequest<'a, C: HyperClient + 'a, T: TokenStorage + 'a, S: serde::Serialize + 'a = &'static str> {
    client: &'a mut API<C, T>,
    endpoint: &'a str,
    query_params: Option<&'a [(&'static str, String)]>,
    post_body: Option<S>,
    auth_required: bool,
}

impl<'a, C: HyperClient, T: TokenStorage, S: serde::Serialize> PartialRequest<'a, C, T, S> {
    #[inline]
    fn params(mut self, params: &'a [(&'static str, String)]) -> Self {
        self.query_params = Some(params);
        self
    }

    #[inline]
    fn auth(mut self, auth: bool) -> Self {
        self.auth_required = auth;
        self
    }

    #[inline]
    fn post(mut self, body: S) -> Self {
        self.post_body = Some(body);
        self
    }

    fn send<R: EndpointResult>(self) -> Result<R> {
        let PartialRequest { client, endpoint, query_params, post_body, auth_required } = self;
        let mut headers = Headers::new();
        headers.set(ContentType::json());

        let temp_token = match (auth_required, client.token.take_token()) {
            (_, Some(token)) => {
                headers.set_raw("X-Token", vec![token.clone()]);
                headers.set_raw("X-Username", vec![token.clone()]);
                Some(token)
            }
            (true, None) => {
                return Err(Error::with_url(error::ErrorType::Unauthorized, None));
            }
            (false, None) => None,
        };

        let url = {
            let mut temp = client.url.join(endpoint)?;

            if let Some(pairs) = query_params {
                temp.query_pairs_mut().extend_pairs(pairs).finish();
            }
            temp
        };


        // Option<Result<A, B>> -> Result<Option<A>, B>
        let post_body = post_body.map_or(Ok(None), |body| serde_json::to_string(&body).map(Some))?;

        let mut response = {
            let client = client.client.client();
            let request = match post_body.as_ref() {
                Some(body) => client.post(url).body(body),
                None => client.get(url),
            };

            request.headers(headers).send()?
        };

        if !response.status.is_success() {
            if log_enabled!(log::LogLevel::Debug) {
                if response.status == hyper::status::StatusCode::Unauthorized {
                    if auth_required {
                        debug!("Token was passed, but still received unauthorized error for {}.",
                               response.url);
                    }
                    if !auth_required {
                        debug!("authentication not required for endpoint {}, but unauthorized error returned anyways.",
                               endpoint);
                    }
                }
            }
            return Err(Error::with_url(response.status, Some(response.url.clone())));
        }

        if let Some(token_vec) = response.headers.get_raw("X-Token") {
            if let Some(token_bytes) = token_vec.first() {
                client.token.return_token(Vec::from(&**token_bytes));
            }
        } else if let Some(token) = temp_token {
            client.token.return_token(token)
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
