//! Interpreting leaderboard season list results.

use EndpointResult;
use data;
use error::{Result, ApiError};
use std::marker::PhantomData;

/// Call raw result.
#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
#[doc(hidden)]
pub struct Response {
    ok: i32,
    seasons: Vec<Season>,
}

#[derive(Deserialize, Debug)]
struct Season {
    _id: String,
    name: String,
    date: String,
}

/// Single leaderboard season description, part of the result to a call to get all leaderboard seasons.
///
/// A leaderboard season is a completed/past saved "season", which in the server marks a past ranking of all
/// players based off of their earned points during that season.
#[derive(Debug, Clone)]
pub struct LeaderboardSeason {
    /// The display name of the season.
    pub name: String,
    /// The season "id", usable by passing to a leaderboard list call.
    pub season_id: String,
    /// The date when the leaderboard season ended, in the format like 2017-03-04T05:38:04.012Z.
    pub end_date: String,
    /// Phantom data in order to allow adding any additional fields in the future.
    _phantom: PhantomData<()>,
}

impl EndpointResult for Vec<LeaderboardSeason> {
    type RequestResult = Response;
    type ErrorResult = data::ApiError;

    fn from_raw(raw: Response) -> Result<Vec<LeaderboardSeason>> {
        let Response { ok, seasons } = raw;

        if ok != 1 {
            return Err(ApiError::NotOk(ok).into());
        }

        Ok(seasons.into_iter()
            .map(|s| {
                LeaderboardSeason {
                    name: s.name,
                    season_id: s._id,
                    end_date: s.date,
                    _phantom: PhantomData,
                }
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::LeaderboardSeason;
    use EndpointResult;
    use serde_json;

    fn test_parse(json: serde_json::Value) {
        let response = serde_json::from_value(json).unwrap();

        let _ = Vec::<LeaderboardSeason>::from_raw(response).unwrap();
    }

    #[test]
    fn parse_sample_leaderboard_season_list() {
        // This is not an actual call, but rather just a truncated call containing the top 5 results.
        test_parse(json! ({
            "ok": 1,
            "seasons": [
                {
                    "_id": "2017-03",
                    "date": "2017-03-01T00:00:05.605Z",
                    "name": "March 2017"
                },
                {
                    "_id": "2017-02",
                    "date": "2017-02-01T00:00:05.410Z",
                    "name": "February 2017"
                },
                {
                    "_id": "2017-01",
                    "date": "2017-01-01T00:00:03.681Z",
                    "name": "January 2017"
                },
                {
                    "_id": "2016-12",
                    "date": "2016-12-01T00:00:05.105Z",
                    "name": "December 2016"
                },
                {
                    "_id": "2016-11",
                    "date": "2016-11-01T00:00:04.489Z",
                    "name": "November 2016"
                },
                {
                    "_id": "2016-10",
                    "date": "2016-10-01T00:00:03.348Z",
                    "name": "October 2016"
                }
            ]
        }));
    }
}
