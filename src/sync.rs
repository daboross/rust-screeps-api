//! Small wrapper around the asynchronous Api struct providing synchronous access methods.
extern crate tokio_core;
extern crate hyper_tls;
extern crate native_tls;

use std::borrow::Cow;
use std::ops::Deref;
use std::io;

use hyper::{self, Client};

use hyper::client::HttpConnector;
use self::hyper_tls::HttpsConnector;

use error::Error;

use {TokenStorage, RcTokenStorage, Api, DEFAULT_URL_STR};

use {MyInfo, RecentPvp, RoomOverview, RoomStatus, RoomTerrain, MapStats, LeaderboardPage, LeaderboardType,
     FoundUserRank, RecentPvpDetails, LeaderboardSeason};

mod error {
    use std::{io, fmt};
    use url;
    use super::native_tls;

    /// Error that can occur from building a SyncApi.
    #[derive(Debug)]
    pub enum SyncError {
        /// The tokio core failed to start.
        Io(io::Error),
        /// The URL failed to parse.
        Url(url::ParseError),
        /// The TLS connector failed.
        Tls(native_tls::Error),
    }

    impl From<io::Error> for SyncError {
        fn from(e: io::Error) -> Self {
            SyncError::Io(e)
        }
    }

    impl From<url::ParseError> for SyncError {
        fn from(e: url::ParseError) -> Self {
            SyncError::Url(e)
        }
    }

    impl From<native_tls::Error> for SyncError {
        fn from(e: native_tls::Error) -> Self {
            SyncError::Tls(e)
        }
    }

    impl fmt::Display for SyncError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                SyncError::Io(ref e) => e.fmt(f),
                SyncError::Url(ref e) => e.fmt(f),
                SyncError::Tls(ref e) => e.fmt(f),
            }
        }
    }

    impl ::std::error::Error for SyncError {
        fn description(&self) -> &str {
            match *self {
                SyncError::Io(ref e) => e.description(),
                SyncError::Url(ref e) => e.description(),
                SyncError::Tls(ref e) => e.description(),
            }
        }

        fn cause(&self) -> Option<&::std::error::Error> {
            match *self {
                SyncError::Io(ref e) => Some(e),
                SyncError::Url(ref e) => Some(e),
                SyncError::Tls(ref e) => Some(e),
            }
        }
    }
}

pub use self::error::SyncError;

/// Represents the configuration which will create a reasonable default HTTPS connector.
#[derive(Copy, Clone, Debug, Default)]
pub struct UseHttpsConnector;

/// Represents the configuration which will create an non-Send token storage.
#[derive(Copy, Clone, Debug, Default)]
pub struct UseRcTokens;

/// API structure mirroring [`Api`], but providing utilities for synchronous connection.
///
/// This structure owns both the hyper client and the tokio core. If this is not wanted, please
/// use [`Api`] instead.
///
/// [`Api`]: ../struct.Api.html
#[derive(Debug)]
pub struct SyncApi<C: hyper::client::Connect = HttpsConnector<HttpConnector>, T: TokenStorage = RcTokenStorage> {
    core: tokio_core::reactor::Core,
    client: Api<C, Client<C>, T>,
}

impl SyncApi<HttpsConnector<HttpConnector>, RcTokenStorage> {
    /// Opinionated method to construct a SyncApi with non-Send token storage, with HTTPS support and
    /// connecting to the default server.
    ///
    /// Since this connects to the official server, it won't be useful without HTTPS support.
    ///
    /// Use [`Config`] for more configuration, including choosing an HTTP only backend, or
    /// setting the url to something other than `https://screep.com/api/`.
    ///
    /// [`Config`]: struct.Config.html
    pub fn new() -> Result<Self, SyncError> {
        Ok(Config::<UseHttpsConnector, UseRcTokens>::new()?.build()?)
    }
}

impl<C: hyper::client::Connect, T: TokenStorage> Deref for SyncApi<C, T> {
    type Target = Api<C, Client<C>, T>;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

/// Configuration for construction a `SyncApi`.
pub struct Config<'a, C = UseHttpsConnector, T = UseRcTokens> {
    core: tokio_core::reactor::Core,
    hyper: hyper::client::Config<C, hyper::Body>,
    tokens: T,
    url: &'a str,
}

impl Config<'static, UseHttpsConnector, UseRcTokens> {
    /// Creates an initial config which will use an HTTPS connector and non-Send tokens.
    pub fn new() -> io::Result<Self> {
        let core = tokio_core::reactor::Core::new()?;
        let hyper = hyper::Client::configure().connector(UseHttpsConnector);
        let config = Config {
            core: core,
            hyper: hyper,
            tokens: UseRcTokens,
            url: DEFAULT_URL_STR,
        };

        Ok(config)
    }
}

impl<'a, C, T> Config<'a, C, T> {
    /// Sets the Hyper connector to connect to to the given connector.
    pub fn connector<F, CC>(self, connector: F) -> Config<'a, CC, T>
        where F: FnOnce(&tokio_core::reactor::Handle) -> CC
    {
        let handle = self.core.handle();
        Config {
            core: self.core,
            hyper: self.hyper.connector(connector(&handle)),
            tokens: self.tokens,
            url: self.url,
        }
    }

    /// Sets the Hyper connector to connect to to the given connector.
    pub fn try_connector<F, E, CC>(self, connector: F) -> Result<Config<'a, CC, T>, E>
        where F: FnOnce(&tokio_core::reactor::Handle) -> Result<CC, E>
    {
        let handle = self.core.handle();
        Ok(Config {
            core: self.core,
            hyper: self.hyper.connector(connector(&handle)?),
            tokens: self.tokens,
            url: self.url,
        })
    }

    /// Sets the url to connect to to the given url.
    pub fn url<'b>(self, url: &'b AsRef<str>) -> Config<'b, C, T> {
        Config {
            core: self.core,
            hyper: self.hyper,
            tokens: self.tokens,
            url: url.as_ref(),
        }
    }

    /// Sets the token storage for the config to the given token storage.
    pub fn tokens<TT>(self, tokens: TT) -> Config<'a, C, TT> {
        Config {
            core: self.core,
            hyper: self.hyper,
            url: self.url,
            tokens: tokens,
        }
    }
}

impl<'a, T: TokenStorage> Config<'a, UseHttpsConnector, T> {
    /// Builds the config into a SyncApi.
    pub fn build(self) -> Result<SyncApi<HttpsConnector<HttpConnector>, T>, SyncError> {
        self.try_connector(|handle| HttpsConnector::new(4, &handle))?
            .build()
            .map_err(Into::into)
    }
}

impl<'a, C: hyper::client::Connect> Config<'a, C, UseRcTokens> {
    /// Builds the config into a SyncApi.
    pub fn build(self) -> Result<SyncApi<C, RcTokenStorage>, SyncError> {
        self.tokens(RcTokenStorage::default()).build()
    }
}

impl<'a> Config<'a, UseHttpsConnector, UseRcTokens> {
    /// Builds the config into a SyncApi.
    pub fn build(self) -> Result<SyncApi<HttpsConnector<HttpConnector>, RcTokenStorage>, SyncError> {
        self.try_connector(|handle| HttpsConnector::new(4, &handle))?
            .tokens(RcTokenStorage::default())
            .build()
            .map_err(Into::into)
    }
}

impl<'a, C: hyper::client::Connect, T: TokenStorage> Config<'a, C, T> {
    /// Builds the config into a SyncApi.
    pub fn build(self) -> Result<SyncApi<C, T>, SyncError> {
        let Config { core, hyper, tokens, url } = self;
        let handle = core.handle();
        let hyper = hyper.build(&handle);

        let api = SyncApi {
            core: core,
            client: Api::with_url_and_tokens(hyper, url, tokens)?,
        };

        Ok(api)
    }
}

impl<C: hyper::client::Connect, T: TokenStorage> SyncApi<C, T> {
    /// Logs in with the given username and password and gets an authentication token as the result.
    ///
    /// The authentication token will then be stored in this client.
    pub fn login<'b, U, V>(&mut self, username: U, password: V) -> Result<(), Error>
        where U: Into<Cow<'b, str>>,
              V: Into<Cow<'b, str>>
    {
        let result = self.core.run(self.client.login(username, password))?;

        result.return_to(&self.client.tokens);

        Ok(())
    }

    /// Gets user information on the user currently logged in, including username and user id.
    ///
    /// See [`Api::my_info`](../struct.Api.html#method.my_info) for more information.
    pub fn my_info(&mut self) -> Result<MyInfo, Error> {
        self.core.run(self.client.my_info()?)
    }

    /// Get information on a number of rooms.
    ///
    /// See [`Api::map_stats`](../struct.Api.html#method.map_stats) for more information.
    pub fn map_stats<'a, U, V>(&mut self, rooms: &'a V) -> Result<MapStats, Error>
        where U: AsRef<str>,
              &'a V: IntoIterator<Item = U>
    {
        self.core.run(self.client.map_stats(rooms)?)
    }

    /// Gets the overview of a room, returning totals for usually 3 intervals, 8, 180 and 1440, representing
    /// data for the past hour, data for the past 24 hours, and data for the past week respectively.
    ///
    /// See [`Api::room_overview`](../struct.Api.html#method.room_overview) for more information.
    pub fn room_overview<'b, U>(&mut self, room_name: U, request_interval: u32) -> Result<RoomOverview, Error>
        where U: Into<Cow<'b, str>>
    {
        self.core.run(self.client.room_overview(room_name, request_interval)?)
    }

    /// Gets the terrain of a room, returning a 2d array of 50x50 points.
    ///
    /// See [`Api::room_terrain`](../struct.Api.html#method.room_terrain) for more information.
    pub fn room_terrain<'b, U>(&mut self, room_name: U) -> Result<RoomTerrain, Error>
        where U: Into<Cow<'b, str>>
    {
        self.core.run(self.client.room_terrain(room_name))
    }

    /// Gets the "status" of a room: if it is open, if it is in a novice area, if it exists.
    ///
    /// See [`Api::room_status`](../struct.Api.html#method.room_status) for more information.
    pub fn room_status<'b, U>(&mut self, room_name: U) -> Result<RoomStatus, Error>
        where U: Into<Cow<'b, str>>
    {
        self.core.run(self.client.room_status(room_name)?)
    }

    /// Experimental endpoint to get all rooms in which PvP has recently occurred.
    ///
    /// See [`Api::recent_pvp`](../struct.Api.html#method.recent_pvp) for more information.
    pub fn recent_pvp(&mut self, details: RecentPvpDetails) -> Result<RecentPvp, Error> {
        self.core.run(self.client.recent_pvp(details))
    }

    /// Gets a list of all past leaderboard seasons, with end dates, display names, and season ids for each season.
    ///
    /// See [`Api::leaderboard_season_list`](../struct.Api.html#method.leaderboard_season_list) for more information.
    pub fn leaderboard_season_list(&mut self) -> Result<Vec<LeaderboardSeason>, Error> {
        self.core.run(self.client.leaderboard_season_list()?)
    }

    /// Finds the rank of a user in a specific season for a specific leaderboard type.
    ///
    /// See [`Api::find_season_leaderboard_rank`] for more information.
    ///
    /// [`Api::find_season_leaderboard_rank`]: ../struct.Api.html#method.find_season_leaderboard_rank
    pub fn find_season_leaderboard_rank<'b, U, V>(&mut self,
                                                  leaderboard_type: LeaderboardType,
                                                  username: U,
                                                  season: V)
                                                  -> Result<FoundUserRank, Error>
        where U: Into<Cow<'b, str>>,
              V: Into<Cow<'b, str>>
    {
        self.core.run(self.client.find_season_leaderboard_rank(leaderboard_type, username, season)?)
    }

    /// Finds the rank of a user for all seasons for a specific leaderboard type.
    ///
    /// See [`Api::leaderboard_page`](../struct.Api.html#method.find_leaderboard_ranks) for more information.
    pub fn find_leaderboard_ranks<'b, U>(&mut self,
                                         leaderboard_type: LeaderboardType,
                                         username: U)
                                         -> Result<Vec<FoundUserRank>, Error>
        where U: Into<Cow<'b, str>>
    {
        self.core.run(self.client.find_leaderboard_ranks(leaderboard_type, username)?)
    }

    /// Gets a page of the leaderboard for a given season.
    ///
    /// See [`Api::leaderboard_page`](../struct.Api.html#method.leaderboard_page) for more information.
    pub fn leaderboard_page<'b, U>(&mut self,
                                   leaderboard_type: LeaderboardType,
                                   season: U,
                                   limit: u32,
                                   offset: u32)
                                   -> Result<LeaderboardPage, Error>
        where U: Into<Cow<'b, str>>
    {
        self.core.run(self.client.leaderboard_page(leaderboard_type, season, limit, offset)?)
    }
}
