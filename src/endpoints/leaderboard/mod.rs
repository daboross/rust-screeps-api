//! Endpoints relating to game leaderboards.
pub mod find_rank;
pub mod season_list;
pub mod page;

/// Type of leaderboards that are available for each season.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LeaderboardType {
    /// Global control point leaderboard.
    GlobalControl,
    /// Power processed leaderboard
    PowerProcessed,
}

impl LeaderboardType {
    /// Gets the representation of this leaderbaord type in the raw API calls.
    ///
    /// Mostly for internal use, but if you can find a reason to call this, feel free.
    pub fn api_representation(&self) -> &'static str {
        match self {
            &LeaderboardType::GlobalControl => "world",
            &LeaderboardType::PowerProcessed => "power",
        }
    }
}
