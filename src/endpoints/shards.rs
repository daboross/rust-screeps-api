//! Interpreting shard info calls.

use EndpointResult;
use data;
use error::{ApiError, Result};
use std::marker::PhantomData;

/// Shard info raw result.
#[derive(Deserialize, Clone, Debug)]
#[doc(hidden)]
pub struct Response {
    ok: i32,
    shards: Vec<ShardResponse>,
}

#[derive(Deserialize, Clone, Debug)]
struct ShardResponse {
    users: u32,
    name: String,
    tick: f64,
    rooms: u32,
}

/// Structure describing information about a single game shard.
#[derive(Clone, Debug)]
pub struct ShardInfo {
    /// The name of this shard, useful for all shard-specific API calls.
    pub name: String,
    /// The total number of open rooms in this shard.
    pub room_count: u32,
    /// The total number of users spawned in this shard (TODO: confirm this is what this is).
    pub user_count: u32,
    /// The average millisecond tick this shard has for some past period of time (TODO: more detail).
    pub tick_avg_milliseconds: f64,
    /// Phantom data in order to allow adding any additional fields in the future.
    _phantom: PhantomData<()>,
}

impl AsRef<str> for ShardInfo {
    fn as_ref(&self) -> &str {
        &self.name
    }
}

impl EndpointResult for Vec<ShardInfo> {
    type RequestResult = Response;
    type ErrorResult = data::ApiError;

    fn from_raw(raw: Response) -> Result<Vec<ShardInfo>> {
        let Response { ok, shards } = raw;

        if ok != 1 {
            return Err(ApiError::NotOk(ok).into());
        }

        Ok(shards
            .into_iter()
            .map(|response| {
                let ShardResponse {
                    users,
                    name,
                    tick,
                    rooms,
                } = response;
                ShardInfo {
                    name: name,
                    room_count: rooms,
                    user_count: users,
                    tick_avg_milliseconds: tick,
                    _phantom: PhantomData,
                }
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::ShardInfo;
    use EndpointResult;
    use serde_json;

    fn test_parse(json: serde_json::Value) {
        let response = serde_json::from_value(json).unwrap();

        let _ = Vec::<ShardInfo>::from_raw(response).unwrap();
    }

    #[test]
    fn parse_sample() {
        test_parse(json! ({
            "shards": [
                {
                    "users": 1246,
                    "rooms": 28858,
                    "name": "shard0",
                    "tick": 5726.411601456489
                },
                {
                    "users": 584,
                    "rooms": 4816,
                    "name": "shard1",
                    "tick": 2153.4476171877614
                }
            ],
            "ok": 1
        }));
    }
}
