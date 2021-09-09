//! Interpreting shard info calls.
use serde::Deserialize;

use crate::{
    data,
    error::{ApiError, Result},
    EndpointResult,
};

/// Shard info raw result.
#[derive(Deserialize, Clone, Debug)]
pub(crate) struct Response {
    ok: i32,
    shards: Vec<ShardResponse>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
struct ShardResponse {
    users: u32,
    name: String,
    tick: f64,
    rooms: u32,
    cpu_limit: i32,
    last_ticks: Vec<u32>,
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
    /// Cpu limit
    pub cpu_limit: i32,
    /// Last ticks
    pub last_ticks: Vec<u32>,
    /// Phantom data in order to allow adding any additional fields in the future.
    _non_exhaustive: (),
}

impl AsRef<str> for ShardInfo {
    fn as_ref(&self) -> &str {
        &self.name
    }
}

impl Into<String> for ShardInfo {
    fn into(self) -> String {
        self.name.into()
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
                    cpu_limit,
                    last_ticks,
                } = response;
                ShardInfo {
                    name: name,
                    room_count: rooms,
                    user_count: users,
                    tick_avg_milliseconds: tick,
                    cpu_limit,
                    last_ticks,
                    _non_exhaustive: (),
                }
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::ShardInfo;
    use crate::EndpointResult;
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
                    "tick": 5726.411601456489,
                    "cpuLimit": 20,
                    "lastTicks":[1111, 2222, 3333],
                },
                {
                    "users": 584,
                    "rooms": 4816,
                    "name": "shard1",
                    "tick": 2153.4476171877614,
                    "cpuLimit": 20,
                    "lastTicks":[],
                }
            ],
            "ok": 1
        }));
    }
}
