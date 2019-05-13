//! Rust library for using the [Screeps] HTTP API.
//!
//! Screeps is a true programming MMO where users uploading JavaScript code to power their online empires.
//! `rust-screeps-api` can connect to the [official server][screeps], and any [private server][screeps-os] instances
//! run by users.
//!
//! `rust-screeps-api` uses [hyper] to run http requests and [serde] to parse json results.
//!
//! # Usage
//!
//! Screeps API is built on two levels: an underlying asynchronous [`Api`] structure, and an easier-to-use [`SyncApi`]
//! built on top of it.
//!
//! To start using screeps through the blocking synchronous API, simply create a `SyncApi` object:
//!
//! ```
//! extern crate screeps_api;
//!
//! # #[cfg(feature = "sync")]
//! # fn main() {
//! use screeps_api::SyncApi;
//!
//! let mut api = SyncApi::new().expect("expected starting screeps http client to suceed");
//! # }
//! # #[cfg(not(feature = "sync"))] fn main() {}
//! ```
//!
//! This API object can then be used to make any number of API calls. Each will return a `Result` with a typed response
//! or an error. All calls require mutable access to manage tokens and the underlying tokio instance:
//!
//! ```no_run
//! # extern crate screeps_api;
//! #
//! # #[cfg(feature = "sync")]
//! # fn main() {
//! # use screeps_api::SyncApi;
//! #
//! # let mut api = SyncApi::new().unwrap();
//! #
//! api.set_token("auth token").unwrap();
//!
//! let my_info = api.my_info().unwrap();
//!
//! println!("Logged in with user ID {}!", my_info.user_id);
//! # }
//! # #[cfg(not(feature = "sync"))] fn main() {}
//! ```
//!
//! [`Api`]: struct.Api.html
//! [`SyncApi`]: sync/struct.SyncApi.html
//! [screeps]: https://screeps.com
//! [screeps-os]: https://github.com/screeps/screeps/
//! [hyper]: https://github.com/hyperium/hyper/
//! [serde]: https://github.com/serde-rs/json/
#![doc(html_root_url = "https://docs.rs/screeps-api/0.4.1")]
#![deny(missing_docs)]
#![recursion_limit = "512"]
#![cfg_attr(feature = "protocol-docs", feature(external_doc))]

// Logging
#[macro_use]
extern crate log;

// Parsing
extern crate arrayvec;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_ignored;
#[cfg_attr(test, macro_use)]
extern crate serde_json;
extern crate time;
extern crate tuple_vec_map;

// HTTP
extern crate bytes;
extern crate futures;
extern crate hyper;
extern crate url;

// Websockets
extern crate num;
extern crate rand;

pub mod connecting;
pub mod data;
mod decoders;
#[cfg(feature = "protocol-docs")]
pub mod docs;
pub mod endpoints;
pub mod error;
#[cfg(feature = "sync")]
pub mod sync;
pub mod websocket;

pub use data::RoomName;
pub use endpoints::leaderboard::find_rank::FoundUserRank;
pub use endpoints::leaderboard::page::LeaderboardPage;
pub use endpoints::leaderboard::season_list::LeaderboardSeason;
pub use endpoints::leaderboard::LeaderboardType;
pub use endpoints::recent_pvp::PvpArgs as RecentPvpDetails;
pub use endpoints::register::{Details as RegistrationDetails, RegistrationSuccess};
pub use endpoints::room_terrain::TerrainGrid;
pub use endpoints::{
    MapStats, MyInfo, RecentPvp, RoomOverview, RoomStatus, RoomTerrain, ShardInfo, WorldStartRoom,
};
pub use error::{Error, ErrorKind, NoToken};
#[cfg(feature = "sync")]
pub use sync::{Config as SyncConfig, SyncApi};

use std::borrow::Cow;
use std::convert::AsRef;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::Arc;

use bytes::Bytes;
use futures::Future;
use hyper::header::{HeaderValue, CONTENT_TYPE};
use url::Url;

use connecting::FutureResponse;
use endpoints::{map_stats, recent_pvp};

use sealed::EndpointResult;

mod sealed {
    use error::Error;
    use serde;

    /// A trait for each endpoint
    pub trait EndpointResult: Sized + 'static {
        type RequestResult: for<'de> serde::Deserialize<'de>;
        type ErrorResult: for<'de> serde::Deserialize<'de> + Into<Error>;

        fn from_raw(data: Self::RequestResult) -> Result<Self, Error>;
    }

    pub trait Sealed: ::EndpointResult {}
    impl<T> Sealed for T where T: ::EndpointResult {}
}

/// Sealed trait implemented for each endpoint.
pub trait EndpointType: sealed::Sealed {}

impl<T> EndpointType for T where T: sealed::Sealed {}

/// An API token that allows for one-time authentication. Each use of an API token with the screeps API
/// will cause the API to return a new token which should be stored in its place.
pub type Token = Bytes;

/// A generic trait over hyper's Client which allows for references, owned clients, and `Arc<hyper::Client>`
/// to be used.
pub trait HyperClient<C>
where
    C: hyper::client::connect::Connect,
{
    /// Get a reference to this client.
    fn client(&self) -> &hyper::Client<C>;
}

impl<C> HyperClient<C> for hyper::Client<C>
where
    C: hyper::client::connect::Connect,
{
    fn client(&self) -> &hyper::Client<C> {
        self
    }
}

impl<'a, C> HyperClient<C> for &'a hyper::Client<C>
where
    C: hyper::client::connect::Connect,
{
    fn client(&self) -> &hyper::Client<C> {
        self
    }
}

impl<C> HyperClient<C> for Arc<hyper::Client<C>>
where
    C: hyper::client::connect::Connect,
{
    fn client(&self) -> &hyper::Client<C> {
        self
    }
}

impl<C> HyperClient<C> for Rc<hyper::Client<C>>
where
    C: hyper::client::connect::Connect,
{
    fn client(&self) -> &hyper::Client<C> {
        self
    }
}

/// API Object, stores the current API token and allows access to making requests.
#[derive(Debug)]
pub struct Api<C, H = hyper::Client<C>> {
    /// The base URL for this API instance.
    pub url: Url,
    /// The authentication token.
    auth_token: Option<Token>,
    /// The hyper client.
    client: H,
    /// Phantom data required in order to have C bound here.
    _phantom: PhantomData<C>,
}

impl<C, H: Clone> Clone for Api<C, H> {
    fn clone(&self) -> Self {
        Api {
            url: self.url.clone(),
            auth_token: self.auth_token.clone(),
            client: self.client.clone(),
            _phantom: PhantomData,
        }
    }
}

/// The official server's default api url`
pub static DEFAULT_OFFICIAL_API_URL: &'static str = "https://screeps.com/api/";

fn default_url() -> Url {
    Url::parse(DEFAULT_OFFICIAL_API_URL).expect("expected pre-set url to parse, parsing failed")
}

impl<C, H> Api<C, H> {
    /// Creates a new API instance for the official server with the `https://screeps.com/api/` base url.
    ///
    /// Use [`with_url`] or [`set_url`] to change to a custom server.
    ///
    /// The returned instance can be used to make anonymous calls. Use [`with_token`] or
    /// [`set_token`] to enable authenticated access.
    #[inline]
    pub fn new(client: H) -> Self {
        Api {
            url: default_url(),
            client: client,
            auth_token: None,
            _phantom: PhantomData,
        }
    }

    /// Sets the server url this api client will use.
    ///
    /// See also [`with_url`].
    #[inline]
    pub fn set_url<U: AsRef<str>>(&mut self, url: U) -> Result<(), url::ParseError> {
        self.url = Url::parse(url.as_ref())?;
        Ok(())
    }

    /// Sets the server url this api client will use, and returns the client.
    ///
    /// See also [`set_url`].
    #[inline]
    pub fn with_url<U: AsRef<str>>(mut self, url: U) -> Result<Self, url::ParseError> {
        self.set_url(url)?;
        Ok(self)
    }

    /// Sets the auth token this api client will use.
    ///
    /// See also [`with_token`].
    #[inline]
    pub fn set_token(&mut self, token: Token) {
        self.auth_token = Some(token);
    }

    /// Sets the auth token this api client will use, and returns the client.
    ///
    /// See also [`set_token`].
    #[inline]
    pub fn with_token(mut self, token: Token) -> Self {
        self.set_token(token);
        self
    }
}

impl<C: hyper::client::connect::Connect + 'static, H: HyperClient<C>> Api<C, H> {
    /// Starts preparing a POST or GET request to the given endpoint URL
    #[inline]
    fn request<'a, R, S>(
        &'a self,
        endpoint: &'a str,
    ) -> PartialRequest<'a, C, H, R, NoAuthRequired, S>
    where
        R: EndpointResult,
        S: serde::Serialize,
    {
        PartialRequest {
            client: self,
            endpoint: endpoint,
            post_body: None,
            query_params: None,
            _phantom: PhantomData,
        }
    }

    /// Makes a new GET request to the given endpoint URL, with given the query parameters added to the end.
    #[inline]
    fn get<'a, R>(
        &'a self,
        endpoint: &'a str,
    ) -> PartialRequest<'a, C, H, R, NoAuthRequired, &'static str>
    where
        R: EndpointResult,
    {
        self.request(endpoint)
    }

    /// Makes a POST request to the given endpoint URL, with the given data encoded as JSON in the body of the request.
    #[inline]
    fn post<'a, U, R>(
        &'a self,
        endpoint: &'a str,
        request_text: U,
    ) -> PartialRequest<'a, C, H, R, NoAuthRequired, U>
    where
        U: serde::Serialize,
        R: EndpointResult,
    {
        self.request(endpoint).post(request_text)
    }

    /// Registers a new account with the given username, password and optional email and returns a
    /// result. Successful results contain no information other than that of success.
    ///
    /// This is primarily for private servers with [screepsmod-auth] installed. Unknown if this
    /// works on the official server.
    ///
    /// [screepsmod-auth]: https://github.com/ScreepsMods/screepsmod-auth
    pub fn register(
        &self,
        details: RegistrationDetails,
    ) -> impl Future<Item = RegistrationSuccess, Error = Error> {
        self.post("register/submit", details).send()
    }

    /// Gets user information on the user currently logged in, including username and user id.
    pub fn my_info(&self) -> Result<impl Future<Item = MyInfo, Error = Error>, NoToken> {
        self.get("auth/me").auth().send()
    }

    /// Gets the world shard and room name the server thinks the client should start with viewing.
    pub fn world_start_room(
        &self,
    ) -> Result<impl Future<Item = WorldStartRoom, Error = Error>, NoToken> {
        self.get("user/world-start-room").auth().send()
    }

    /// Gets the room name the server thinks the client should start with viewing for a particular shard.
    pub fn shard_start_room<'b, U>(
        &self,
        shard: U,
    ) -> Result<impl Future<Item = WorldStartRoom, Error = Error>, NoToken>
    where
        U: Into<Cow<'b, str>>,
    {
        self.get("user/world-start-room")
            .params(&[("shard", shard.into().into_owned())])
            .auth()
            .send()
    }

    /// Get information on a number of rooms.
    pub fn map_stats<'a, U, V>(
        &self,
        shard: &'a str,
        rooms: &'a V,
    ) -> Result<impl Future<Item = MapStats, Error = Error>, NoToken>
    where
        U: AsRef<str>,
        &'a V: IntoIterator<Item = U>,
    {
        // TODO: interpret for different stats.
        let args = map_stats::MapStatsArgs::new(shard, rooms, map_stats::StatName::RoomOwner);

        self.post("game/map-stats", args).auth().send()
    }

    /// Gets the overview of a room, returning totals for usually 3 intervals, 8, 180 and 1440, representing
    /// data for the past hour, data for the past 24 hours, and data for the past week respectively.
    ///
    /// All Allowed request_intervals are not known, but at least `8`, `180` and `1440` are allowed. The returned data,
    /// at the time of writing, includes 8 data points of each type, representing equal portions of the time period
    /// requested (hour for `8`, day for `180`, week for `1440`).
    pub fn room_overview<'b, U, V>(
        &self,
        shard: U,
        room_name: V,
        request_interval: u32,
    ) -> Result<impl Future<Item = RoomOverview, Error = Error>, NoToken>
    where
        U: Into<Cow<'b, str>>,
        V: Into<Cow<'b, str>>,
    {
        self.get("game/room-overview")
            .params(&[
                ("shard", shard.into().into_owned()),
                ("room", room_name.into().into_owned()),
                ("interval", request_interval.to_string()),
            ])
            .auth()
            .send()
    }

    /// Gets the terrain of a room, returning a 2d array of 50x50 points.
    ///
    /// Does not require authentication.
    pub fn room_terrain<'b, U, V>(
        &self,
        shard: Option<U>,
        room_name: V,
    ) -> impl Future<Item = RoomTerrain, Error = Error>
    where
        U: Into<Cow<'b, str>>,
        V: Into<Cow<'b, str>>,
    {
        match shard {
            Some(shard) => self
                .get("game/room-terrain")
                .params(&[
                    ("shard", shard.into().into_owned()),
                    ("room", room_name.into().into_owned()),
                    ("encoded", true.to_string()),
                ])
                .send(),
            None => self
                .get("game/room-terrain")
                .params(&[
                    ("room", room_name.into().into_owned()),
                    ("encoded", true.to_string()),
                ])
                .send(),
        }
    }

    /// Gets a list of shards available on this server. Errors with a `404` error when connected to a
    /// non-sharded server.
    ///
    /// Does not require authentication.
    pub fn shard_list(&self) -> impl Future<Item = Vec<ShardInfo>, Error = Error> {
        self.get("game/shards/info").send()
    }

    /// Gets the "status" of a room: if it is open, if it is in a novice area, if it exists.
    pub fn room_status<'b, U>(
        &self,
        room_name: U,
    ) -> Result<impl Future<Item = RoomStatus, Error = Error>, NoToken>
    where
        U: Into<Cow<'b, str>>,
    {
        self.get("game/room-status")
            .params(&[("room", room_name.into().into_owned())])
            .auth()
            .send()
    }

    /// Experimental endpoint to get all rooms in which PvP has recently occurred, or where PvP has occurred since a
    /// certain game tick.
    pub fn recent_pvp(
        &self,
        details: RecentPvpDetails,
    ) -> impl Future<Item = RecentPvp, Error = Error> {
        let args = match details {
            recent_pvp::PvpArgs::WithinLast { ticks } => [("interval", ticks.to_string())],
            recent_pvp::PvpArgs::Since { time } => [("start", time.to_string())],
        };

        self.get("experimental/pvp").params(&args).send()
    }

    /// Gets a list of all past leaderboard seasons, with end dates, display names, and season ids for each season.
    ///
    /// Seasons are a way of having limited time periods (usually one month) in which all rankings are reset at the
    /// beginning of, and points earned during the time period contribute to a player's rank in that season.
    ///
    /// This method does not return any actual data, but rather just a list of valid past season, any of the ids of
    /// which can then be used to retrieve more information.
    pub fn leaderboard_season_list(
        &self,
    ) -> Result<impl Future<Item = Vec<LeaderboardSeason>, Error = Error>, NoToken> {
        self.get("leaderboard/seasons").auth().send()
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
    pub fn find_season_leaderboard_rank<'b, U, V>(
        &self,
        leaderboard_type: LeaderboardType,
        username: U,
        season: V,
    ) -> Result<impl Future<Item = FoundUserRank, Error = Error>, NoToken>
    where
        U: Into<Cow<'b, str>>,
        V: Into<Cow<'b, str>>,
    {
        self.get("leaderboard/find")
            .auth()
            .params(&[
                ("mode", leaderboard_type.api_representation().to_string()),
                ("season", season.into().into_owned()),
                ("username", username.into().into_owned()),
            ])
            .send()
    }

    /// Finds the rank of a user for all seasons for a specific leaderboard type.
    ///
    /// This will return `ApiError::UserNotFound` if a username does not exist, and may also return an empty `Vec` as
    /// the result if the user does not have any ranks in the given leaderboard type (they have never contributed any
    /// global control points, or processed power, depending on the type).
    pub fn find_leaderboard_ranks<'b, U>(
        &self,
        leaderboard_type: LeaderboardType,
        username: U,
    ) -> Result<impl Future<Item = Vec<FoundUserRank>, Error = Error>, NoToken>
    where
        U: Into<Cow<'b, str>>,
    {
        self.get("leaderboard/find")
            .auth()
            .params(&[
                ("mode", leaderboard_type.api_representation().to_string()),
                ("username", username.into().into_owned()),
            ])
            .send()
    }

    /// Gets a page of the leaderboard for a given season.
    ///
    /// Limit dictates how many users will be returned, maximum is 20. Higher than that will cause an InvalidParameters
    /// error message.
    ///
    /// Offset doesn't have to be a multiple of limit, but it's most likely most useful that it is. Offset 0 will get
    /// you the start/top of the ranked list.
    pub fn leaderboard_page<'b, U>(
        &self,
        leaderboard_type: LeaderboardType,
        season: U,
        limit: u32,
        offset: u32,
    ) -> Result<impl Future<Item = LeaderboardPage, Error = Error>, NoToken>
    where
        U: Into<Cow<'b, str>>,
    {
        self.get("leaderboard/list")
            .auth()
            .params(&[
                ("mode", leaderboard_type.api_representation().to_string()),
                ("season", season.into().into_owned()),
                ("limit", limit.to_string()),
                ("offset", offset.to_string()),
            ])
            .send()
    }
}

/// Really hacky way of having compile-time assurance there's no
/// auth errors for non-auth requiring types.
trait PartialRequestAuth<T> {
    type Result;

    fn token_or_result(token: Option<&Token>) -> Result<Option<&Token>, Self::Result>;

    fn successful_result(success: T) -> Self::Result;
}

struct NoAuthRequired;

impl<T> PartialRequestAuth<T> for NoAuthRequired {
    type Result = T;

    fn token_or_result(_token: Option<&Token>) -> Result<Option<&Token>, T> {
        Ok(None)
    }

    fn successful_result(success: T) -> T {
        success
    }
}

struct AuthRequired;

impl<T> PartialRequestAuth<T> for AuthRequired {
    type Result = Result<T, NoToken>;

    fn token_or_result(token: Option<&Token>) -> Result<Option<&Token>, Result<T, NoToken>> {
        match token {
            Some(v) => Ok(Some(v)),
            None => Err(Err(NoToken)),
        }
    }

    fn successful_result(success: T) -> Result<T, NoToken> {
        Ok(success)
    }
}

struct PartialRequest<'a, C, H, R, A = NoAuthRequired, S = &'static str>
where
    C: hyper::client::connect::Connect,
    H: HyperClient<C> + 'a,
    R: EndpointResult,
    A: PartialRequestAuth<FutureResponse<R>>,
    S: serde::Serialize + 'a,
{
    client: &'a Api<C, H>,
    endpoint: &'a str,
    query_params: Option<&'a [(&'static str, String)]>,
    post_body: Option<S>,
    _phantom: PhantomData<(R, A)>,
}

impl<'a, C, H, R, S> PartialRequest<'a, C, H, R, NoAuthRequired, S>
where
    C: hyper::client::connect::Connect + 'static,
    H: HyperClient<C> + 'a,
    R: EndpointResult,
    S: serde::Serialize,
{
    #[inline]
    fn auth(self) -> PartialRequest<'a, C, H, R, AuthRequired, S> {
        PartialRequest {
            client: self.client,
            endpoint: self.endpoint,
            query_params: self.query_params,
            post_body: self.post_body,
            _phantom: PhantomData,
        }
    }
}

impl<'a, C, H, R, S> PartialRequest<'a, C, H, R, AuthRequired, S>
where
    C: hyper::client::connect::Connect + 'static,
    H: HyperClient<C> + 'a,
    R: EndpointResult,
    S: serde::Serialize,
{
    // This particular method should be a useful one to have around, even if just for completeness.
    #[allow(dead_code)]
    #[inline]
    fn no_auth(self) -> PartialRequest<'a, C, H, R, NoAuthRequired, S> {
        PartialRequest {
            client: self.client,
            endpoint: self.endpoint,
            query_params: self.query_params,
            post_body: self.post_body,
            _phantom: PhantomData,
        }
    }
}

impl<'a, C, H, R, A, S> PartialRequest<'a, C, H, R, A, S>
where
    C: hyper::client::connect::Connect + 'static,
    H: HyperClient<C> + 'a,
    R: EndpointResult,
    A: PartialRequestAuth<FutureResponse<R>>,
    S: serde::Serialize,
{
    #[inline]
    fn params(mut self, params: &'a [(&'static str, String)]) -> Self {
        self.query_params = Some(params);
        self
    }

    #[inline]
    fn post(mut self, body: S) -> Self {
        self.post_body = Some(body);
        self
    }

    /// Result type here _so hacky!_ Glad this is an internal API.
    ///
    /// Returns either `connecting::impl Future<Item=R, Error=Error>` or `Result<connecting::FutureResponse<R>, NoToken>`
    /// depending on if auth() has been called.
    fn send(self) -> A::Result {
        let PartialRequest {
            client,
            endpoint,
            query_params,
            post_body,
            _phantom: _,
        } = self;

        // this checks if authentication is required.
        //
        // like:
        // ```
        // let auth_token = if auth_required {
        //     match client.tokens.take_token() {
        //         Some(token) => Some(token),
        //         None => return Err(NoToken),
        //     }
        // } else {
        //     None
        // };
        // ```
        //
        // but this way we can return without a Result if authentication isn't required.
        let auth_token = match A::token_or_result(client.auth_token.as_ref()) {
            Ok(token_option) => token_option,
            Err(return_value) => return return_value,
        };

        let method = match post_body {
            Some(_) => hyper::Method::POST,
            None => hyper::Method::GET,
        };

        let url = {
            let mut temp = client
                .url
                .join(endpoint)
                .expect("expected pre-set endpoint url text to succeed, but it failed.");

            if let Some(pairs) = query_params {
                temp.query_pairs_mut().extend_pairs(pairs).finish();
            }

            temp
        };

        let mut request = hyper::Request::builder();

        request.method(method).uri(url.as_str());

        // headers
        request.header(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        if let Some(token) = auth_token {
            request.header("X-Token", token.clone());
        }

        let request = if let Some(ref serializable) = post_body {
            request.body(hyper::Body::from(
                serde_json::to_string(serializable).expect(
                    "expected serde_json::to_string to unfailingly succeed, but it failed.",
                ),
            ))
        } else {
            request.body(hyper::Body::empty())
        };
        let request = request.expect("building http request should never fail");

        let hyper_future = client.client.client().request(request);
        let finished = connecting::interpret(url, hyper_future);

        // turns into either `Result<FutureResponse<..>>` or `FutureResponse<..>` depending on
        // if we required auth.
        A::successful_result(finished)
    }
}

/// Calculates GCL, given GCL points.
#[inline]
pub fn gcl_calc(gcl_points: u64) -> u64 {
    const GCL_INV_MULTIPLY: f64 = 1.0 / 1_000_000f64;
    const GCL_INV_POW: f64 = 1.0 / 2.4f64;

    ((gcl_points as f64) * GCL_INV_MULTIPLY)
        .powf(GCL_INV_POW)
        .floor() as u64
        + 1
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
