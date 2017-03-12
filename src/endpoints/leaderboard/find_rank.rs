//! Find the rank of a users on the leaderboard.

use EndpointResult;
use data;
use error::{Result, ApiError};
use std::marker::PhantomData;

/// Raw result for when the API endpoint is called with a specific season id.
#[derive(Deserialize, Debug)]
pub struct SingleResponse {
    // I have no idea what this is for, so not including in the documented and expected response.
    // id: String,
    ok: i32,
    rank: u32,
    score: u64,
    season: String,
    user: String,
}

/// Raw result for when the API endpoint is called without a specific season id.
#[derive(Deserialize, Debug)]
pub struct AllSeasonRanksResponse {
    ok: i32,
    list: Vec<InnerAllSeasonsResponse>,
}

#[derive(Deserialize, Debug)]
struct InnerAllSeasonsResponse {
    // Again, no idea what this is for, so I'm not including it in the documented response.
    // _id: String,
    rank: u32,
    score: u64,
    season: String,
    user: String,
}

/// Result from a lookup for a user's rank on the leaderboard
#[derive(Debug, Clone)]
pub struct FoundUserRank {
    /// The season ID which this rank is for
    pub season_id: String,
    /// The user's ID
    pub user_id: String,
    /// The user's rank in this season for the requested leaderboard type
    ///
    /// The top user's rank is 0, so add one to this digit if displaying to a user.
    pub rank: u32,
    /// The user's raw score for this season for the requested leaderboard type.
    pub raw_score: u64,
    /// Phantom data in order to allow adding any additional fields in the future.
    #[doc(hidden)]
    pub _phantom: PhantomData<()>,
}

// This is the result when specifying one season.
impl EndpointResult for FoundUserRank {
    type RequestResult = SingleResponse;
    type ErrorResult = data::ApiError;

    fn from_raw(raw: SingleResponse) -> Result<FoundUserRank> {
        let SingleResponse { ok, rank, score, season, user } = raw;

        if ok != 1 {
            return Err(ApiError::NotOk(ok).into());
        }

        Ok(FoundUserRank {
            season_id: season,
            user_id: user,
            rank: rank,
            raw_score: score,
            _phantom: PhantomData,
        })
    }
}

// This is the result when requesting without specifying one season.
impl EndpointResult for Vec<FoundUserRank> {
    type RequestResult = AllSeasonRanksResponse;
    type ErrorResult = data::ApiError;

    fn from_raw(raw: AllSeasonRanksResponse) -> Result<Vec<FoundUserRank>> {
        let AllSeasonRanksResponse { ok, list: season_ranks } = raw;

        if ok != 1 {
            return Err(ApiError::NotOk(ok).into());
        }

        Ok(season_ranks.into_iter()
            .map(|raw_rank| {
                let InnerAllSeasonsResponse { rank, score, season, user } = raw_rank;
                FoundUserRank {
                    season_id: season,
                    user_id: user,
                    rank: rank,
                    raw_score: score,
                    _phantom: PhantomData,
                }
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::FoundUserRank;
    use EndpointResult;
    use serde_json;

    fn test_parse_single(json: serde_json::Value) {
        let response = serde_json::from_value(json).unwrap();

        let _ = FoundUserRank::from_raw(response).unwrap();
    }
    fn test_parse_multi(json: serde_json::Value) {
        let response = serde_json::from_value(json).unwrap();

        let _ = Vec::<FoundUserRank>::from_raw(response).unwrap();
    }

    #[test]
    fn parse_sample_single_season() {
        test_parse_single(json! ({
            "_id": "58b60f2f75a8e42a5c0923f9",
            "ok": 1,
            "rank": 68,
            "score": 43146791,
            "season": "2017-03",
            "user": "57874d42d0ae911e3bd15bbc"
        }));
    }

    #[test]
    fn parse_sample_multiple_seasons() {
        test_parse_multi(json! ({
            "list": [
                {
                    "_id": "578bec7c276fc5fcb7dfe0c1",
                    "rank": 1016,
                    "score": 1195020,
                    "season": "2016-07",
                    "user": "57874d42d0ae911e3bd15bbc"
                },
                {
                    "_id": "579e911a6cb50b7bd8f9d346",
                    "rank": 213,
                    "score": 17250219,
                    "season": "2016-08",
                    "user": "57874d42d0ae911e3bd15bbc"
                },
                {
                    "_id": "57c76f95dab8e790758fccb0",
                    "rank": 129,
                    "score": 38156711,
                    "season": "2016-09",
                    "user": "57874d42d0ae911e3bd15bbc"
                },
                {
                    "_id": "57eefc9577a8d271d10d9b54",
                    "rank": 110,
                    "score": 53901405,
                    "season": "2016-10",
                    "user": "57874d42d0ae911e3bd15bbc"
                },
                {
                    "_id": "5817db0fd22ddaa587ebfae0",
                    "rank": 47,
                    "score": 111031282,
                    "season": "2016-11",
                    "user": "57874d42d0ae911e3bd15bbc"
                },
                {
                    "_id": "583f682780bb5a3d2b36f1e8",
                    "rank": 60,
                    "score": 112480526,
                    "season": "2016-12",
                    "user": "57874d42d0ae911e3bd15bbc"
                },
                {
                    "_id": "586846a0ec0e632daa9077fe",
                    "rank": 76,
                    "score": 99291185,
                    "season": "2017-01",
                    "user": "57874d42d0ae911e3bd15bbc"
                },
                {
                    "_id": "589125339c76d517f2424990",
                    "rank": 59,
                    "score": 119478210,
                    "season": "2017-02",
                    "user": "57874d42d0ae911e3bd15bbc"
                },
                {
                    "_id": "58b60f2f75a8e42a5c0923f9",
                    "rank": 68,
                    "score": 43159803,
                    "season": "2017-03",
                    "user": "57874d42d0ae911e3bd15bbc"
                }
            ],
            "ok": 1
        }));
    }
}
