//! Interpreting user leaderboard page results.

use super::find_rank;
use EndpointResult;
use data;
use error::{ApiError, Result};
use std::collections::HashMap;
use std::marker::PhantomData;

/// Raw list results.
#[derive(Deserialize, Clone, Debug)]
#[doc(hidden)]
pub struct Response {
    ok: i32,
    count: u64,
    list: Vec<ResponseRankedUser>,
    users: HashMap<String, ExtendedUserInfo>,
}

#[derive(Deserialize, Clone, Hash, Debug)]
struct ResponseRankedUser {
    //_id: String, // exists, but I don't know what it's for.
    rank: u32,
    score: u64,
    season: String,
    user: String,
}

#[derive(Deserialize, Clone, Hash, Debug)]
struct ExtendedUserInfo {
    _id: String,
    username: String,
    gcl: u64,
    badge: data::Badge,
}

/// Single leaderboard page of users.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LeaderboardPage {
    /// The total number of users present in this leaderboard.
    pub total_count: u64,
    /// List of users with user IDs, rank, score and season. I'm... not sure why season is in here and not a top-level
    /// thing, but that seems to be how it is.
    pub ranks: Vec<RankedUser>,
    /// Details about all users listed. This is a map from user_id to info struct containing username, gcl, and badge.
    #[serde(with = "::tuple_vec_map")]
    pub user_details: Vec<(String, UserDetails)>,
    /// Phantom data in order to allow adding any additional fields in the future.
    _phantom: PhantomData<()>,
}

/// Alias since the format is the same for the inner user ranks and found user ranks.
pub type RankedUser = find_rank::FoundUserRank;

/// Details on any user in a given leaderboard page result.
#[derive(Serialize, Deserialize, Clone, Hash, Debug)]
pub struct UserDetails {
    /// The user's id.
    pub user_id: String,
    /// The user's badge.
    pub badge: data::Badge,
    /// The user's GCL points (calculate to get GCL)
    pub gcl_points: u64,
    /// The user's username.
    pub username: String,
    /// Phantom data in order to allow adding any additional fields in the future.
    _phantom: PhantomData<()>,
}

impl EndpointResult for LeaderboardPage {
    type RequestResult = Response;
    type ErrorResult = data::ApiError;

    fn from_raw(raw: Response) -> Result<LeaderboardPage> {
        let Response {
            ok,
            count: total_count,
            list,
            users: user_details,
        } = raw;

        if ok != 1 {
            return Err(ApiError::NotOk(ok).into());
        }

        Ok(LeaderboardPage {
            total_count: total_count,
            ranks: list.into_iter()
                .map(|info| {
                    RankedUser {
                        season_id: info.season,
                        user_id: info.user,
                        rank: info.rank,
                        raw_score: info.score,
                        _phantom: PhantomData,
                    }
                })
                .collect(),
            user_details: user_details
                .into_iter()
                .map(|(user_id, data)| {
                    (
                        user_id,
                        UserDetails {
                            user_id: data._id,
                            badge: data.badge,
                            gcl_points: data.gcl,
                            username: data.username,
                            _phantom: PhantomData,
                        },
                    )
                })
                .collect(),
            _phantom: PhantomData,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::LeaderboardPage;
    use EndpointResult;
    use serde_json;

    fn test_parse(json: serde_json::Value) {
        let response = serde_json::from_value(json).unwrap();

        let _ = LeaderboardPage::from_raw(response).unwrap();
    }

    #[test]
    fn parse_sample_gcl() {
        test_parse(json! ({
            "count": 2322,
            "list": [
                {
                    "_id": "589125339c76d517f2424784",
                    "rank": 2,
                    "score": 408713655i64,
                    "season": "2017-02",
                    "user": "5769c4e64673b865097b926e"
                },
                {
                    "_id": "589125339c76d517f242475e",
                    "rank": 3,
                    "score": 362128212i64,
                    "season": "2017-02",
                    "user": "56dce30812162c1c122ae1b5"
                }
            ],
            "ok": 1,
            "users": {
                "56dce30812162c1c122ae1b5": {
                    "_id": "56dce30812162c1c122ae1b5",
                    "badge": {
                        "color1": "#511e00",
                        "color2": "#003058",
                        "color3": "#00450e",
                        "flip": true,
                        "param": -47,
                        "type": 1
                    },
                    "gcl": 2097651263i64,
                    "username": "taiga"
                },
                "5769c4e64673b865097b926e": {
                    "_id": "5769c4e64673b865097b926e",
                    "badge": {
                        "color1": "#000000",
                        "color2": "#000000",
                        "color3": "#430000",
                        "flip": false,
                        "param": 49,
                        "type": 8
                    },
                    "gcl": 1845096413i64,
                    "username": "Bovius"
                }
            }
        }));
    }
    #[test]
    fn parse_sample_power() {
        test_parse(json! ({
            "count": 60,
            "list": [
                {
                    "_id": "589125359c76d517f2424cea",
                    "rank": 2,
                    "score": 2653410i64,
                    "season": "2017-02",
                    "user": "567d9401f60a26fc4c41bd38"
                },
                {
                    "_id": "5891259a9c76d517f2427e9d",
                    "rank": 3,
                    "score": 2320123i64,
                    "season": "2017-02",
                    "user": "576aa0ce36db3b70321317fe"
                }
            ],
            "ok": 1,
            "users": {
                "567d9401f60a26fc4c41bd38": {
                    "_id": "567d9401f60a26fc4c41bd38",
                    "badge": {
                        "color1": 73,
                        "color2": 31,
                        "color3": 8,
                        "flip": false,
                        "param": 68,
                        "type": 10
                    },
                    "gcl": 2993541124i64,
                    "username": "bonzaiferroni"
                },
                "576aa0ce36db3b70321317fe": {
                    "_id": "576aa0ce36db3b70321317fe",
                    "badge": {
                        "color1": "#f9a603",
                        "color2": "#f7efe2",
                        "color3": "#f25c00",
                        "flip": false,
                        "param": 0,
                        "type": 9
                    },
                    "gcl": 1982028763i64,
                    "username": "Atavus"
                }
            }
        }));
    }
}
