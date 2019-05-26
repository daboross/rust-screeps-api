//! Endpoints relating to game leaderboards.
pub mod find_rank;
pub mod page;
pub mod season_list;

pub use self::{find_rank::*, page::*, season_list::*};

/// Type of leaderboards that are available for each season.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum LeaderboardType {
    /// Global control point leaderboard.
    GlobalControl,
    /// Power processed leaderboard
    PowerProcessed,
    /// A marker variant that tells the compiler that users of this enum cannot match it exhaustively.
    #[doc(hidden)]
    __Nonexhaustive,
}

impl LeaderboardType {
    /// Gets the representation of this leaderbaord type in the raw API calls.
    ///
    /// Mostly for internal use, but if you can find a reason to call this, feel free.
    pub fn api_representation(&self) -> &'static str {
        match *self {
            LeaderboardType::GlobalControl | LeaderboardType::__Nonexhaustive => "world",
            LeaderboardType::PowerProcessed => "power",
        }
    }
}
