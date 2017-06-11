//! Rust wrapper for the Screeps public API.
//!
//! Allows rust programs to retrieve game information from https://screeps.com and any private Screeps server.
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
//! use screeps_api::SyncApi;
//!
//! # fn main() {
//! let mut api = SyncApi::new().expect("starting screeps Api failed");
//! # }
//! ```
//!
//! This API object can then be used to make any number of API calls. Each will return a `Result` with a typed response
//! or an error. All calls require mutable access to manage tokens and the underlying tokio instance:
//!
//! ```no_run
//! # extern crate screeps_api;
//! #
//! # use screeps_api::SyncApi;
//! #
//! # fn main() {
//! let mut api = SyncApi::new().unwrap();
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
//! [`Api`]: struct.Api.html
//! [`SyncApi`]: sync/struct.SyncApi.html
#![deny(missing_docs)]
#![recursion_limit="512"]
// Logging
#[macro_use]
extern crate log;
extern crate time;
// Parsing
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[cfg_attr(test, macro_use)]
extern crate serde_json;
extern crate serde_ignored;
extern crate tuple_vec_map;
extern crate arrayvec;
extern crate generic_array;
extern crate typenum;
// HTTP
extern crate futures;
extern crate url;
#[macro_use]
extern crate hyper;
// Websockets
extern crate rand;

pub mod error;
pub mod endpoints;
pub mod data;
pub mod connecting;
pub mod websocket;
#[cfg(feature = "sync")]
pub mod sync;

pub use error::{Error, ErrorKind, NoToken};
pub use data::RoomName;
pub use endpoints::{MyInfo, RecentPvp, RoomOverview, RoomStatus, RoomTerrain, MapStats};
pub use endpoints::leaderboard::LeaderboardType;
pub use endpoints::room_terrain::TerrainGrid;
pub use endpoints::recent_pvp::PvpArgs as RecentPvpDetails;
pub use endpoints::login::LoggedIn;
pub use endpoints::leaderboard::season_list::LeaderboardSeason;
pub use endpoints::leaderboard::find_rank::FoundUserRank;
pub use endpoints::leaderboard::page::LeaderboardPage;
pub use connecting::FutureResponse;
#[cfg(feature = "sync")]
pub use sync::{SyncApi, Config as SyncConfig};

use std::marker::PhantomData;
use std::borrow::Cow;
use std::convert::AsRef;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::rc::Rc;
use std::cell::RefCell;

use url::Url;
use hyper::header::ContentType;

use endpoints::{login, my_info, room_overview, room_terrain, room_status, recent_pvp, map_stats};

use sealed::EndpointResult;

mod sealed {
    use serde;
    use error::Error;

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
pub type Token = String;

/// A generic trait over hyper's Client which allows for references, owned clients, and Arc<hyper::Client>
/// to be used.
pub trait HyperClient<C>
    where C: hyper::client::Connect
{
    /// Get a reference to this client.
    fn client(&self) -> &hyper::Client<C>;
}

/// A generic trait over some storage for auth tokens, possibly for use with sharing tokens between clients.
pub trait TokenStorage: Clone + 'static {
    /// Takes a token from the token storage, if there are any tokens.
    fn take_token(&self) -> Option<Token>;

    /// Gives a new token back to the token storage.
    fn return_token(&self, Token);
}

/// Convenience type representing the regular token storage when sending between threads is required.
pub type ArcTokenStorage = Arc<Mutex<VecDeque<Token>>>;

/// Convenience type representing the regular token storage when sending between threads is not required.
pub type RcTokenStorage = Rc<RefCell<VecDeque<Token>>>;

impl<C> HyperClient<C> for hyper::Client<C>
    where C: hyper::client::Connect
{
    fn client(&self) -> &hyper::Client<C> {
        self
    }
}

impl<'a, C> HyperClient<C> for &'a hyper::Client<C>
    where C: hyper::client::Connect
{
    fn client(&self) -> &hyper::Client<C> {
        self
    }
}

impl<C> HyperClient<C> for Arc<hyper::Client<C>>
    where C: hyper::client::Connect
{
    fn client(&self) -> &hyper::Client<C> {
        &self
    }
}

impl<C> HyperClient<C> for Rc<hyper::Client<C>>
    where C: hyper::client::Connect
{
    fn client(&self) -> &hyper::Client<C> {
        &self
    }
}

impl TokenStorage for Rc<RefCell<VecDeque<Token>>> {
    fn take_token(&self) -> Option<Token> {
        self.borrow_mut().pop_front()
    }

    fn return_token(&self, token: Token) {
        self.borrow_mut().push_back(token)
    }
}

impl TokenStorage for Arc<Mutex<VecDeque<Token>>> {
    fn take_token(&self) -> Option<Token> {
        self.lock().unwrap_or_else(|e| e.into_inner()).pop_front()
    }

    fn return_token(&self, token: Token) {
        self.lock().unwrap_or_else(|e| e.into_inner()).push_back(token)
    }
}

/// API Object, stores the current API token and allows access to making requests.
#[derive(Debug)]
pub struct Api<C, H = hyper::Client<C>, T = RcTokenStorage> {
    /// The base URL for this API instance.
    pub url: Url,
    /// The stored authentication tokens.
    pub tokens: T,
    /// The hyper client.
    client: H,
    /// Phantom data required in order to have C bound here.
    _phantom: PhantomData<C>,
}

impl<C, H: Clone, T: Clone> Clone for Api<C, H, T> {
    fn clone(&self) -> Self {
        Api {
            url: self.url.clone(),
            tokens: self.tokens.clone(),
            client: self.client.clone(),
            _phantom: PhantomData,
        }
    }
}

static DEFAULT_URL_STR: &'static str = "https://screeps.com/api/";

fn default_url() -> Url {
    Url::parse(DEFAULT_URL_STR).expect("expected pre-set url to parse, parsing failed")
}

impl<C, H, T: Default> Api<C, H, T> {
    /// Creates a new API instance for the official server with the `https://screeps.com/api/` base url.
    ///
    /// The returned instance can be used to make anonymous calls, or `API.login` can be used to allow for
    /// authenticated API calls.
    #[inline]
    pub fn new(client: H) -> Self {
        Api {
            url: default_url(),
            client: client,
            tokens: T::default(),
            _phantom: PhantomData,
        }
    }

    /// Creates a new API instance with the given url as the base instead of `https://screeps.com/api/`.
    ///
    /// The returned instance can be used to make anonymous calls, or `API.login` can be used to allow for
    /// authenticated API calls.
    #[inline]
    pub fn with_url<U: AsRef<str>>(client: H, url: U) -> Result<Self, url::ParseError> {
        let api = Api {
            url: Url::parse(url.as_ref())?,
            client: client,
            tokens: T::default(),
            _phantom: PhantomData,
        };
        Ok(api)
    }
}

impl<C, H, T> Api<C, H, T> {
    /// Creates a new API instance for the official server with a stored token.
    ///
    /// The returned instance can be used to make both anonymous calls, and authenticated calls, provided
    /// the token is valid.
    #[inline]
    pub fn with_tokens(client: H, tokens: T) -> Self {
        Api {
            url: default_url(),
            client: client,
            tokens: tokens,
            _phantom: PhantomData,
        }
    }

    /// Creates a new API instance with the given url as the base instead of `https://screeps.com/api/`, and
    /// with a stored token for that url.
    ///
    /// The returned instance can be used to make anonymous calls and will be allowed to make authenticated calls
    /// if the token is valid.
    #[inline]
    pub fn with_url_and_tokens<U: AsRef<str>>(client: H, url: U, tokens: T) -> Result<Self, url::ParseError> {
        let api = Api {
            url: Url::parse(url.as_ref())?,
            client: client,
            tokens: tokens,
            _phantom: PhantomData,
        };
        Ok(api)
    }
}

impl<C: hyper::client::Connect, H: HyperClient<C>, T: TokenStorage> Api<C, H, T> {
    /// Starts preparing a POST or GET request to the given endpoint URL
    #[inline]
    fn request<'a, R, S>(&'a self,
                         endpoint: &'a str)
                         -> PartialRequest<'a, C, H, T, R, NoAuthRequired<FutureResponse<R>>, S>
        where R: EndpointResult,
              S: serde::Serialize
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
    fn get<'a, R>(&'a self,
                  endpoint: &'a str)
                  -> PartialRequest<'a, C, H, T, R, NoAuthRequired<FutureResponse<R>>, &'static str>
        where R: EndpointResult
    {
        self.request(endpoint)
    }


    /// Makes a POST request to the given endpoint URL, with the given data encoded as JSON in the body of the request.
    #[inline]
    fn post<'a, U, R>(&'a self,
                      endpoint: &'a str,
                      request_text: U)
                      -> PartialRequest<'a, C, H, T, R, NoAuthRequired<FutureResponse<R>>, U>
        where U: serde::Serialize,
              R: EndpointResult
    {
        self.request(endpoint).post(request_text)
    }

    /// Logs in with the given username and password and returns a result containing the token.
    ///
    /// Use `logged_in.return_to(client.tokens)` to let the client use the token from logging in.
    pub fn login<'b, U, V>(&self, username: U, password: V) -> FutureResponse<LoggedIn>
        where U: Into<Cow<'b, str>>,
              V: Into<Cow<'b, str>>
    {
        self.post("auth/signin", login::Details::new(username, password)).send()
    }

    /// Gets user information on the user currently logged in, including username and user id.
    pub fn my_info(&self) -> Result<FutureResponse<MyInfo>, NoToken> {
        self.get("auth/me").auth().send()
    }

    /// Get information on a number of rooms.
    pub fn map_stats<'a, U, V>(&self, rooms: &'a V) -> Result<FutureResponse<MapStats>, NoToken>
        where U: AsRef<str>,
              &'a V: IntoIterator<Item = U>
    {
        // TODO: interpret for different stats.
        let args = map_stats::MapStatsArgs::new(rooms, map_stats::StatName::RoomOwner);

        self.post("game/map-stats", args)
            .auth()
            .send()
    }

    /// Gets the overview of a room, returning totals for usually 3 intervals, 8, 180 and 1440, representing
    /// data for the past hour, data for the past 24 hours, and data for the past week respectively.
    ///
    /// All Allowed request_intervals are not known, but at least `8`, `180` and `1440` are allowed. The returned data,
    /// at the time of writing, includes 8 data points of each type, representing equal portions of the time period
    /// requested (hour for `8`, day for `180`, week for `1440`).
    pub fn room_overview<'b, U>(&self,
                                room_name: U,
                                request_interval: u32)
                                -> Result<FutureResponse<RoomOverview>, NoToken>
        where U: Into<Cow<'b, str>>
    {
        self.get("game/room-overview")
            .params(&[("room", room_name.into().into_owned()), ("interval", request_interval.to_string())])
            .auth()
            .send()
    }

    /// Gets the terrain of a room, returning a 2d array of 50x50 points.
    ///
    /// Does not require authentication.
    pub fn room_terrain<'b, U>(&self, room_name: U) -> FutureResponse<RoomTerrain>
        where U: Into<Cow<'b, str>>
    {
        self.get("game/room-terrain")
            .params(&[("room", room_name.into().into_owned()), ("encoded", true.to_string())])
            .send()
    }

    /// Gets the "status" of a room: if it is open, if it is in a novice area, if it exists.
    pub fn room_status<'b, U>(&self, room_name: U) -> Result<FutureResponse<RoomStatus>, NoToken>
        where U: Into<Cow<'b, str>>
    {
        self.get("game/room-status").params(&[("room", room_name.into().into_owned())]).auth().send()
    }

    /// Experimental endpoint to get all rooms in which PvP has recently occurred, or where PvP has occurred since a
    /// certain game tick.
    pub fn recent_pvp(&self, details: RecentPvpDetails) -> FutureResponse<RecentPvp> {
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
    pub fn leaderboard_season_list(&self) -> Result<FutureResponse<Vec<LeaderboardSeason>>, NoToken> {
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
    pub fn find_season_leaderboard_rank<'b, U, V>(&self,
                                                  leaderboard_type: LeaderboardType,
                                                  username: U,
                                                  season: V)
                                                  -> Result<FutureResponse<FoundUserRank>, NoToken>
        where U: Into<Cow<'b, str>>,
              V: Into<Cow<'b, str>>
    {
        self.get("leaderboard/find")
            .auth()
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
    pub fn find_leaderboard_ranks<'b, U>(&self,
                                         leaderboard_type: LeaderboardType,
                                         username: U)
                                         -> Result<FutureResponse<Vec<FoundUserRank>>, NoToken>
        where U: Into<Cow<'b, str>>
    {
        self.get("leaderboard/find")
            .auth()
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
    pub fn leaderboard_page<'b, U>(&self,
                                   leaderboard_type: LeaderboardType,
                                   season: U,
                                   limit: u32,
                                   offset: u32)
                                   -> Result<FutureResponse<LeaderboardPage>, NoToken>
        where U: Into<Cow<'b, str>>
    {
        self.get("leaderboard/list")
            .auth()
            .params(&[("mode", leaderboard_type.api_representation().to_string()),
                      ("season", season.into().into_owned()),
                      ("limit", limit.to_string()),
                      ("offset", offset.to_string())])
            .send()
    }
}

/// Really hacky way of having compile-time assurance there's no
/// auth errors for non-auth requiring types.
trait PartialRequestAuth<T> {
    type Result;

    fn token_or_result<U: TokenStorage>(token_storage: &U) -> Result<Option<Token>, Self::Result>;

    fn successful_result(success: T) -> Self::Result;
}

struct NoAuthRequired<T>(PhantomData<T>);

impl<T> PartialRequestAuth<T> for NoAuthRequired<T> {
    type Result = T;

    fn token_or_result<U: TokenStorage>(_: &U) -> Result<Option<Token>, T> {
        Ok(None)
    }

    fn successful_result(success: T) -> T {
        success
    }
}

struct AuthRequired<T>(PhantomData<T>);

impl<T> PartialRequestAuth<T> for AuthRequired<T> {
    type Result = Result<T, NoToken>;

    fn token_or_result<U: TokenStorage>(token_storage: &U) -> Result<Option<Token>, Result<T, NoToken>> {
        match token_storage.take_token() {
            Some(v) => Ok(Some(v)),
            None => Err(Err(NoToken)),
        }
    }

    fn successful_result(success: T) -> Result<T, NoToken> {
        Ok(success)
    }
}

struct PartialRequest<'a, C, H, T, R, A = NoAuthRequired<FutureResponse<R>>, S = &'static str>
    where C: hyper::client::Connect,
          H: HyperClient<C> + 'a,
          T: TokenStorage + 'a,
          R: EndpointResult,
          A: PartialRequestAuth<FutureResponse<R>>,
          S: serde::Serialize + 'a
{
    client: &'a Api<C, H, T>,
    endpoint: &'a str,
    query_params: Option<&'a [(&'static str, String)]>,
    post_body: Option<S>,
    _phantom: PhantomData<(R, A)>,
}

impl<'a, C, H, T, R, S> PartialRequest<'a, C, H, T, R, NoAuthRequired<FutureResponse<R>>, S>
    where C: hyper::client::Connect,
          H: HyperClient<C> + 'a,
          T: TokenStorage + 'a,
          R: EndpointResult,
          S: serde::Serialize
{
    #[inline]
    fn auth(self) -> PartialRequest<'a, C, H, T, R, AuthRequired<FutureResponse<R>>, S> {
        PartialRequest {
            client: self.client,
            endpoint: self.endpoint,
            query_params: self.query_params,
            post_body: self.post_body,
            _phantom: PhantomData,
        }
    }
}

impl<'a, C, H, T, R, S> PartialRequest<'a, C, H, T, R, AuthRequired<FutureResponse<R>>, S>
    where C: hyper::client::Connect,
          H: HyperClient<C> + 'a,
          T: TokenStorage + 'a,
          R: EndpointResult,
          S: serde::Serialize
{
    // This particular method should be a useful one to have around, even if just for completeness.
    #[allow(dead_code)]
    #[inline]
    fn no_auth(self) -> PartialRequest<'a, C, H, T, R, NoAuthRequired<FutureResponse<R>>, S> {
        PartialRequest {
            client: self.client,
            endpoint: self.endpoint,
            query_params: self.query_params,
            post_body: self.post_body,
            _phantom: PhantomData,
        }
    }
}

impl<'a, C, H, T, R, A, S> PartialRequest<'a, C, H, T, R, A, S>
    where C: hyper::client::Connect,
          H: HyperClient<C> + 'a,
          T: TokenStorage + 'a,
          R: EndpointResult,
          A: PartialRequestAuth<FutureResponse<R>>,
          S: serde::Serialize
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
    /// Returns either `connecting::FutureResponse<R>` or `Result<connecting::FutureResponse<R>, NoToken>`
    /// depending on if auth() has been called.
    fn send(self) -> A::Result {
        let PartialRequest { client, endpoint, query_params, post_body, _phantom: _ } = self;

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
        let auth_token = match A::token_or_result(&client.tokens) {
            Ok(token_option) => token_option,
            Err(return_value) => return return_value,
        };

        let method = match post_body {
            Some(_) => hyper::Method::Post,
            None => hyper::Method::Get,
        };

        let url = {
            let mut temp =
                client.url.join(endpoint).expect("expected pre-set endpoint url text to succeed, but it failed.");

            if let Some(pairs) = query_params {
                temp.query_pairs_mut().extend_pairs(pairs).finish();
            }

            temp
        };

        let mut request = hyper::client::Request::new(method,
                                                      url.as_str()
                                                          .parse()
                                                          .expect("expected parsed url to successfully \
                                                               parse as uri, but it failed."));

        // headers
        {
            let mut headers = request.headers_mut();

            headers.set(ContentType::json());

            if let Some(token) = auth_token.as_ref() {
                let arc = Arc::new(token.to_owned());
                headers.set(headers::XTokenHeader(arc.clone()));
                headers.set(headers::XUsernameHeader(arc));
            }
        }

        if let Some(ref serializable) = post_body {
            request.set_body(serde_json::to_string(serializable)
                .expect("expected serde_json::to_string to unfailingly succeed, but it failed."));
        }

        let hyper_future = client.client.client().request(request);
        let finished = connecting::interpret(client.tokens.clone(), auth_token, url, hyper_future);

        // turns into either `Result<FutureResponse<..>>` or `FutureResponse<..>` depending on
        // if we required auth.
        A::successful_result(finished)
    }
}

mod headers {
    use std::fmt;
    use std::sync::Arc;

    use hyper::{self, header};

    #[derive(Clone, Debug)]
    pub struct XTokenHeader(pub Arc<String>);

    impl header::Header for XTokenHeader {
        fn header_name() -> &'static str {
            "X-Token"
        }

        fn parse_header(raw: &header::Raw) -> hyper::Result<Self> {
            header::parsing::from_one_raw_str(raw).map(Arc::new).map(XTokenHeader)
        }

        fn fmt_header(&self, f: &mut header::Formatter) -> fmt::Result {
            f.fmt_line(&&**self.0)
        }
    }

    #[derive(Clone, Debug)]
    pub struct XUsernameHeader(pub Arc<String>);

    impl header::Header for XUsernameHeader {
        fn header_name() -> &'static str {
            "X-Username"
        }

        fn parse_header(raw: &header::Raw) -> hyper::Result<Self> {
            header::parsing::from_one_raw_str(raw).map(Arc::new).map(XUsernameHeader)
        }

        fn fmt_header(&self, f: &mut header::Formatter) -> fmt::Result {
            f.fmt_line(&&**self.0)
        }
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
