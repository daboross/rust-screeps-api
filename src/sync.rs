//! Small wrapper around the asynchronous Api struct providing synchronous access methods.

extern crate hyper_tls;
extern crate native_tls;
extern crate tokio;

use std::borrow::Cow;
use std::io;
use std::ops::Deref;

use hyper::{self, Client};

use self::hyper_tls::HttpsConnector;
use hyper::client::HttpConnector;

use error::Error;

use {Api, DEFAULT_OFFICIAL_API_URL};

use {
    FoundUserRank, LeaderboardPage, LeaderboardSeason, LeaderboardType, MapStats, MyInfo,
    RecentPvp, RecentPvpDetails, RegistrationDetails, RegistrationSuccess, RoomOverview,
    RoomStatus, RoomTerrain, ShardInfo, WorldStartRoom,
};

type TokioRuntime = self::tokio::runtime::current_thread::Runtime;

mod error {
    use super::native_tls;
    use std::{fmt, io};
    use url;

    /// Error that can occur from building a [`SyncApi`].
    ///
    /// [`SyncApi`]: struct.SyncApi.html
    #[derive(Debug)]
    pub enum SyncError {
        /// The tokio runtime failed to start.
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

/// API structure mirroring [`Api`], but providing utilities for synchronous connection.
///
/// This structure owns both the hyper client and the tokio runtime. If this is not wanted, please
/// use [`Api`] instead.
///
/// [`Api`]: ../struct.Api.html
#[derive(Debug)]
pub struct SyncApi<C: hyper::client::connect::Connect = HttpsConnector<HttpConnector>> {
    runtime: TokioRuntime,
    client: Api<C, Client<C>>,
}

impl SyncApi<HttpsConnector<HttpConnector>> {
    /// Opinionated method to construct a SyncApi with HTTPS support and
    /// connecting to the default server.
    ///
    /// Since this connects to the official server, it won't be useful without HTTPS support.
    ///
    /// Use [`Config`] for more configuration, including choosing an HTTP only backend, or
    /// setting the url to something other than `https://screep.com/api/`.
    ///
    /// [`Config`]: struct.Config.html
    pub fn new() -> Result<Self, SyncError> {
        Ok(Config::<UseHttpsConnector>::new()?.build()?)
    }
}

impl<C: hyper::client::connect::Connect> Deref for SyncApi<C> {
    type Target = Api<C, Client<C>>;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

/// Configuration for construction a `SyncApi`.
pub struct Config<'a, C = UseHttpsConnector> {
    runtime: TokioRuntime,
    connector: C,
    url: &'a str,
}

impl Config<'static, UseHttpsConnector> {
    /// Creates an initial config which will use an HTTPS connector and non-Send tokens.
    pub fn new() -> io::Result<Self> {
        let runtime = TokioRuntime::new()?;
        let config = Config {
            runtime,
            connector: UseHttpsConnector,
            url: DEFAULT_OFFICIAL_API_URL,
        };

        Ok(config)
    }
}

impl<'a, C> Config<'a, C> {
    /// Sets the Hyper connector to connect to to the given connector.
    pub fn connector<CC>(self, connector: CC) -> Config<'a, CC> {
        Config {
            runtime: self.runtime,
            connector,
            url: self.url,
        }
    }

    /// Sets the url to connect to to the given url.
    pub fn url(self, url: &AsRef<str>) -> Config<C> {
        Config {
            runtime: self.runtime,
            connector: self.connector,
            url: url.as_ref(),
        }
    }
}

impl<'a> Config<'a, UseHttpsConnector> {
    /// Builds the config into a SyncApi.
    pub fn build(self) -> Result<SyncApi<HttpsConnector<HttpConnector>>, SyncError> {
        self.connector(HttpsConnector::new(4)?)
            .build()
            .map_err(Into::into)
    }
}

impl<'a, C: hyper::client::connect::Connect + 'static> Config<'a, C> {
    /// Builds the config into a SyncApi.
    pub fn build(self) -> Result<SyncApi<C>, SyncError> {
        let Config {
            runtime,
            url,
            connector,
        } = self;
        let hyper = Client::builder().build(connector);

        let api = SyncApi {
            runtime,
            client: Api::new(hyper).with_url(url)?,
        };

        Ok(api)
    }
}

impl<C: hyper::client::connect::Connect + 'static> SyncApi<C> {
    /// Registers a new account with the given username, password and optional email and returns a
    /// result. Successful results contain no information other than that of success.
    ///
    /// This is primarily for private servers with [screepsmod-auth] installed. Unknown if this
    /// works on the official server.
    ///
    /// [screepsmod-auth]: https://github.com/ScreepsMods/screepsmod-auth
    pub fn register(&mut self, details: RegistrationDetails) -> Result<RegistrationSuccess, Error> {
        self.runtime.block_on(self.client.register(details))
    }

    /// Gets user information on the user currently logged in, including username and user id.
    ///
    /// See [`Api::my_info`](../struct.Api.html#method.my_info) for more information.
    pub fn my_info(&mut self) -> Result<MyInfo, Error> {
        self.runtime.block_on(self.client.my_info()?)
    }

    /// Gets the world shard and room name the server thinks the client should start with viewing.
    ///
    /// See [`Api::world_start_room`](../struct.Api.html#method.world_start_room) for more information.
    pub fn world_start_room(&mut self) -> Result<WorldStartRoom, Error> {
        self.runtime.block_on(self.client.world_start_room()?)
    }

    /// Gets the room name the server thinks the client should start with viewing for a particular shard.
    ///
    /// See [`Api::world_start_room`](../struct.Api.html#method.world_start_room) for more information.
    pub fn shard_start_room<'b, U>(&mut self, shard: U) -> Result<WorldStartRoom, Error>
    where
        U: Into<Cow<'b, str>>,
    {
        self.runtime.block_on(self.client.shard_start_room(shard)?)
    }

    /// Get information on a number of rooms.
    ///
    /// See [`Api::map_stats`](../struct.Api.html#method.map_stats) for more information.
    pub fn map_stats<'a, U, V>(&mut self, shard: &'a str, rooms: &'a V) -> Result<MapStats, Error>
    where
        U: AsRef<str>,
        &'a V: IntoIterator<Item = U>,
    {
        self.runtime.block_on(self.client.map_stats(shard, rooms)?)
    }

    /// Gets the overview of a room, returning totals for usually 3 intervals, 8, 180 and 1440, representing
    /// data for the past hour, data for the past 24 hours, and data for the past week respectively.
    ///
    /// See [`Api::room_overview`](../struct.Api.html#method.room_overview) for more information.
    pub fn room_overview<'b, U, V>(
        &mut self,
        shard: U,
        room_name: V,
        request_interval: u32,
    ) -> Result<RoomOverview, Error>
    where
        U: Into<Cow<'b, str>>,
        V: Into<Cow<'b, str>>,
    {
        self.runtime.block_on(
            self.client
                .room_overview(shard, room_name, request_interval)?,
        )
    }

    /// Gets the terrain of a room, returning a 2d array of 50x50 points.
    ///
    /// See [`Api::room_terrain`](../struct.Api.html#method.room_terrain) for more information.
    pub fn room_terrain<'b, U, V>(
        &mut self,
        shard: Option<U>,
        room_name: V,
    ) -> Result<RoomTerrain, Error>
    where
        U: Into<Cow<'b, str>>,
        V: Into<Cow<'b, str>>,
    {
        self.runtime
            .block_on(self.client.room_terrain(shard, room_name))
    }

    /// Gets a list of shards available on this server. Errors with a `404` error when connected to a
    /// non-sharded server.
    ///
    /// See [`Api::shard_list`](../struct.Api.html#method.shard_list) for more information.
    pub fn shard_list(&mut self) -> Result<Vec<ShardInfo>, Error> {
        self.runtime.block_on(self.client.shard_list())
    }

    /// Gets the "status" of a room: if it is open, if it is in a novice area, if it exists.
    ///
    /// See [`Api::room_status`](../struct.Api.html#method.room_status) for more information.
    pub fn room_status<'b, U>(&mut self, room_name: U) -> Result<RoomStatus, Error>
    where
        U: Into<Cow<'b, str>>,
    {
        self.runtime.block_on(self.client.room_status(room_name)?)
    }

    /// Experimental endpoint to get all rooms in which PvP has recently occurred.
    ///
    /// See [`Api::recent_pvp`](../struct.Api.html#method.recent_pvp) for more information.
    pub fn recent_pvp(&mut self, details: RecentPvpDetails) -> Result<RecentPvp, Error> {
        self.runtime.block_on(self.client.recent_pvp(details))
    }

    /// Gets a list of all past leaderboard seasons, with end dates, display names, and season ids for each season.
    ///
    /// See [`Api::leaderboard_season_list`](../struct.Api.html#method.leaderboard_season_list) for more information.
    pub fn leaderboard_season_list(&mut self) -> Result<Vec<LeaderboardSeason>, Error> {
        self.runtime
            .block_on(self.client.leaderboard_season_list()?)
    }

    /// Finds the rank of a user in a specific season for a specific leaderboard type.
    ///
    /// See [`Api::find_season_leaderboard_rank`] for more information.
    ///
    /// [`Api::find_season_leaderboard_rank`]: ../struct.Api.html#method.find_season_leaderboard_rank
    pub fn find_season_leaderboard_rank<'b, U, V>(
        &mut self,
        leaderboard_type: LeaderboardType,
        username: U,
        season: V,
    ) -> Result<FoundUserRank, Error>
    where
        U: Into<Cow<'b, str>>,
        V: Into<Cow<'b, str>>,
    {
        self.runtime
            .block_on(self.client.find_season_leaderboard_rank(
                leaderboard_type,
                username,
                season,
            )?)
    }

    /// Finds the rank of a user for all seasons for a specific leaderboard type.
    ///
    /// See [`Api::leaderboard_page`](../struct.Api.html#method.find_leaderboard_ranks) for more information.
    pub fn find_leaderboard_ranks<'b, U>(
        &mut self,
        leaderboard_type: LeaderboardType,
        username: U,
    ) -> Result<Vec<FoundUserRank>, Error>
    where
        U: Into<Cow<'b, str>>,
    {
        self.runtime.block_on(
            self.client
                .find_leaderboard_ranks(leaderboard_type, username)?,
        )
    }

    /// Gets a page of the leaderboard for a given season.
    ///
    /// See [`Api::leaderboard_page`](../struct.Api.html#method.leaderboard_page) for more information.
    pub fn leaderboard_page<'b, U>(
        &mut self,
        leaderboard_type: LeaderboardType,
        season: U,
        limit: u32,
        offset: u32,
    ) -> Result<LeaderboardPage, Error>
    where
        U: Into<Cow<'b, str>>,
    {
        self.runtime.block_on(self.client.leaderboard_page(
            leaderboard_type,
            season,
            limit,
            offset,
        )?)
    }
}
