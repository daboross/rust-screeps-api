//! Small wrapper around the asynchronous Api struct providing synchronous access methods.
use std::{
    borrow::Cow,
    io,
    ops::{Deref, DerefMut},
};

use hyper::client::HttpConnector;
use hyper::{self, Client};
use hyper_tls::HttpsConnector;

use crate::{
    error::Error, Api, FoundUserRank, LeaderboardPage, LeaderboardSeason, LeaderboardType,
    MapStats, MyInfo, RecentPvp, RecentPvpArgs, RegistrationArgs, RegistrationSuccess,
    RoomOverview, RoomStatus, RoomTerrain, ShardInfo, Token, WorldStartRoom,
};

type TokioRuntime = tokio::runtime::current_thread::Runtime;

mod error {
    use std::{fmt, io};

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
        Tls(hyper_tls::Error),
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

    impl From<hyper_tls::Error> for SyncError {
        fn from(e: hyper_tls::Error) -> Self {
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

/// API structure mirroring [`Api`], but providing utilities for synchronous connection.
///
/// This structure owns both the hyper client and the tokio runtime. If this is not wanted, please
/// use [`Api`] instead.
///
/// [`Api`]: ../struct.Api.html
#[derive(Debug)]
pub struct SyncApi<C = HttpsConnector<HttpConnector>> {
    runtime: TokioRuntime,
    client: Api<C>,
}

impl SyncApi<HttpsConnector<HttpConnector>> {
    /// Creates a sync API client using an Https connector.
    ///
    /// Use [`SyncApi::new_with_connector`] to set another backend, such as an HTTP only backend.
    pub fn new() -> Result<Self, SyncError> {
        Ok(Self::new_with_connector(HttpsConnector::new(4)?)?)
    }
}

impl<C: hyper::client::connect::Connect + 'static> SyncApi<C> {
    /// Creates a sync API client using a custom connector.
    pub fn new_with_connector(connector: C) -> Result<Self, io::Error> {
        let runtime = TokioRuntime::new()?;
        let hyper = Client::builder().build(connector);
        Ok(SyncApi {
            runtime,
            client: Api::new(hyper),
        })
    }
}

impl<C> Deref for SyncApi<C> {
    type Target = Api<C>;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

impl<C> DerefMut for SyncApi<C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.client
    }
}

impl<C: hyper::client::connect::Connect + 'static> SyncApi<C> {
    /// Sets the server url this api client will use, and returns the client.
    ///
    /// See also [`Api::set_url`].
    #[inline]
    pub fn with_url<U: AsRef<str>>(mut self, url: U) -> Result<Self, url::ParseError> {
        self.set_url(url)?;
        Ok(self)
    }

    /// Sets the auth token this api client will use, and returns the client.
    ///
    /// See also [`Api::set_token`].
    #[inline]
    pub fn with_token<T: Into<Token>>(mut self, token: T) -> Self {
        self.set_token(token.into());
        self
    }

    /// Logs in with the given username and password and gets an authentication token as the
    /// result.
    ///
    /// The authentication token will then be stored in this client.
    pub fn login<'b, U, V>(&mut self, username: U, password: V) -> Result<(), Error>
    where
        U: Into<Cow<'b, str>>,
        V: Into<Cow<'b, str>>,
    {
        let result = self
            .runtime
            .block_on(self.client.login(username, password))?;

        result.return_to(&self.client.auth_token);

        Ok(())
    }

    /// Registers a new account with the given username, password and optional email and returns a
    /// result. Successful results contain no information other than that of success.
    ///
    /// This is primarily for private servers with [screepsmod-auth] installed. Unknown if this
    /// works on the official server.
    ///
    /// [screepsmod-auth]: https://github.com/ScreepsMods/screepsmod-auth
    pub fn register(&mut self, details: RegistrationArgs) -> Result<RegistrationSuccess, Error> {
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
    pub fn recent_pvp(&mut self, details: RecentPvpArgs) -> Result<RecentPvp, Error> {
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
