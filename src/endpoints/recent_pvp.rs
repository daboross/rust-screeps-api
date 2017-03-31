//! Interpreting rooms in which PvP recently occurred. This is an "experimental" endpoint.

use EndpointResult;
use data;
use error::{Result, ApiError};
use std::marker::PhantomData;

/// Call parameters for requesting recent pvp
#[derive(Debug, Copy, Clone)]
pub enum PvpArgs {
    /// Retrieves rooms where pvp has occurred recently, with a given number of game ticks.
    WithinLast {
        /// The interval of game ticks to request. It is unknown the maximum interval that may be requested.
        ticks: i64,
    },
    /// Retrieves rooms where pvp has occurred since a given game time.
    Since {
        /// The game "time" (tick number) to request PvP since. It is unknown how far back of a time may be requested.
        time: i64,
    },
}

impl PvpArgs {
    /// Creates a new PvP call parameter to request any PvP occurring since the given game tick.
    pub fn since(tick: i64) -> PvpArgs {
        PvpArgs::Since { time: tick }
    }
    /// Creates a new PvP call parameter to request any PvP occurring within the last x ticks.
    pub fn within(ticks: i64) -> PvpArgs {
        PvpArgs::WithinLast { ticks: ticks }
    }
}

/// Recent PvP raw result.
#[derive(Deserialize, Debug)]
#[doc(hidden)]
pub struct Response {
    ok: i32,
    rooms: Vec<InnerRoom>,
    time: i64,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct InnerRoom {
    _id: String,
    lastPvpTime: i64,
}


/// Result storing recent pvp matches.
#[derive(Debug, Clone)]
pub struct RecentPvp {
    /// A list of room names in which pvp has recently occurred, and the time at which pvp last occurred.
    pub rooms: Vec<(String, i64)>,
    /// The current game time of the server when the call was completed, the tick up to which pvp has been reported.
    pub reported_up_to: i64,
    /// Phantom data in order to allow adding any additional fields in the future.
    #[doc(hidden)]
    pub _phantom: PhantomData<()>,
}

impl EndpointResult for RecentPvp {
    type RequestResult = Response;
    type ErrorResult = data::ApiError;

    fn from_raw(raw: Response) -> Result<RecentPvp> {
        let Response { ok, rooms, time } = raw;

        if ok != 1 {
            return Err(ApiError::NotOk(ok).into());
        }

        Ok(RecentPvp {
            rooms: rooms.into_iter().map(|r| (r._id, r.lastPvpTime)).collect(),
            reported_up_to: time,
            _phantom: PhantomData,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::RecentPvp;
    use EndpointResult;
    use serde_json;

    fn test_parse(json: serde_json::Value) {
        let response = serde_json::from_value(json).unwrap();

        let _ = RecentPvp::from_raw(response).unwrap();
    }

    #[test]
    fn parse_sample_pvp() {
        test_parse(json! ({
            "ok": 1,
            "rooms": [
                {
                    "_id": "W78S19",
                    "lastPvpTime": 17806851
                },
                {
                    "_id": "E23N77",
                    "lastPvpTime": 17806847
                },
                {
                    "_id": "E84N3",
                    "lastPvpTime": 17806844
                },
                {
                    "_id": "W87N58",
                    "lastPvpTime": 17806843
                }
            ],
            "time": 17806852
        }));
    }
}
