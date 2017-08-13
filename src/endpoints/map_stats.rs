//! Interpreting bulk room statistics (map stats).
//!
//! Note: currently only supports "owner0" stats, not any other statistic that can also be retrieved with the same API.
use std::marker::PhantomData;
use std::convert::AsRef;

use time;
use serde::{Serialize, Serializer};

use data::{self, optional_timespec_seconds, RoomName};

use EndpointResult;
use error::ApiError;
use error::Result as ScapiResult;

/// Stat name argument to the map stats call. Only one possible argument implemented, more to come!
#[derive(Serialize, Clone, Eq, PartialEq, Hash, Debug)]
pub enum StatName {
    /// Gets the room owner (always gotten even if other stats are requested).
    #[serde(rename = "owner0")]
    RoomOwner,
    /// A marker variant that tells the compiler that users of this enum cannot match it exhaustively.
    #[doc(hidden)]
    #[serde(rename = "owner0")]
    __Nonexhaustive,
}

/// Arguments to a map stats call, holds a single value which can be iterated to get rooms.
#[derive(Serialize, Clone, Debug)]
#[serde(bound = "")]
pub struct MapStatsArgs<'a, T, I>
where
    I: AsRef<str>,
    T: 'a,
    &'a T: IntoIterator<Item = I>,
{
    rooms: MapStatsArgsInner<'a, T, I>,
    #[serde(rename = "statName")]
    stat: StatName,
    shard: &'a str,
}

#[derive(Clone, Debug)]
struct MapStatsArgsInner<'a, T, I>
where
    I: AsRef<str>,
    T: 'a,
    &'a T: IntoIterator<Item = I>,
{
    rooms: &'a T,
}

impl<'a, T, I> MapStatsArgs<'a, T, I>
where
    I: AsRef<str>,
    &'a T: IntoIterator<Item = I>,
{
    /// Creates a new MapStatsArgs with the given iterator.
    pub fn new(shard: &'a str, rooms: &'a T, stat: StatName) -> Self {
        MapStatsArgs {
            shard: shard,
            rooms: MapStatsArgsInner { rooms: rooms },
            stat: stat,
        }
    }
}

impl<'a, T, I> Serialize for MapStatsArgsInner<'a, T, I>
where
    I: AsRef<str>,
    &'a T: IntoIterator<Item = I>,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeSeq;

        let iter = self.rooms.into_iter();
        let len_hint = match iter.size_hint() {
            (lo, Some(hi)) if lo == hi => Some(lo),
            _ => None,
        };
        let mut seq_serializer = serializer.serialize_seq(len_hint)?;
        for item in iter {
            seq_serializer.serialize_element(item.as_ref())?;
        }
        seq_serializer.end()
    }
}



/// Map stats raw result.
#[derive(Deserialize, Clone, Hash, Debug)]
#[doc(hidden)]
pub struct Response {
    ok: i32,
    #[serde(with = "::tuple_vec_map")]
    stats: Vec<(String, RoomResponse)>,
    #[serde(with = "::tuple_vec_map")]
    users: Vec<(String, UserResponse)>,
}

#[derive(Deserialize, Clone, Hash, Debug)]
#[serde(rename_all = "camelCase")]
struct RoomResponse {
    status: String,
    own: Option<RoomOwner>,
    /// The end time for the novice area this room is or was last in.
    #[serde(with = "optional_timespec_seconds")]
    #[serde(default)]
    novice: Option<time::Timespec>,
    /// The time this room will open or did open into the novice area as a second tier novice room.
    #[serde(with = "optional_timespec_seconds")]
    #[serde(default)]
    open_time: Option<time::Timespec>,
    sign: Option<data::RoomSign>,
    hard_sign: Option<data::HardSign>,
}

#[derive(Deserialize, Clone, Hash, Debug)]
struct UserResponse {
    badge: data::Badge,
    _id: String,
    username: String,
}

/// Description of the owner of an owned room.
#[derive(Serialize, Deserialize, Clone, Hash, Debug)]
pub struct RoomOwner {
    /// User ID of the room owner
    #[serde(rename = "user")]
    pub user_id: String,
    /// Room control level of the room.
    #[serde(rename = "level")]
    pub room_controller_level: u32,
    /// Phantom data in order to allow adding any additional fields in the future.
    #[serde(skip)]
    _phantom: PhantomData<()>,
}

/// Statistics on a number of rooms.
#[derive(Clone, Debug)]
pub struct MapStats {
    /// A list of results retrieved from this map stats call. Note: Invalid or non-existent room names will simply just
    /// not appear in this result!
    ///
    /// If you request some rooms, and only get part back, you can assume that all extra rooms requested simply do
    /// not exist.
    pub rooms: Vec<RoomInfo>,
    /// A list of user information for each user who either owns or signed a room that was requested.
    pub users: Vec<UserInfo>,
    /// Phantom data in order to allow adding any additional fields in the future.
    _phantom: PhantomData<()>,
}

/// Information on a room.
#[derive(Serialize, Deserialize, Clone, Hash, Debug)]
pub struct RoomInfo {
    /// The room name / id.
    pub name: RoomName,
    /// The room state.
    pub state: data::RoomState,
    /// Info on the room's owner, if any.
    pub owner: Option<RoomOwner>,
    /// The room's player-set sign, if any.
    pub sign: Option<data::RoomSign>,
    /// The room's system-set sign, if any.
    pub hard_sign: Option<data::HardSign>,
    /// Phantom data in order to allow adding any additional fields in the future.
    #[serde(skip)]
    _phantom: PhantomData<()>,
}

/// Information on a user.
#[derive(Serialize, Deserialize, Clone, Hash, Debug)]
pub struct UserInfo {
    /// The user's ID.
    pub user_id: String,
    /// The user's username.
    pub username: String,
    /// The user's badge.
    pub badge: data::Badge,
    /// Phantom data in order to allow adding any additional fields in the future.
    #[serde(skip)]
    _phantom: PhantomData<()>,
}

impl EndpointResult for MapStats {
    type RequestResult = Response;
    type ErrorResult = data::ApiError;

    fn from_raw(raw: Response) -> ScapiResult<MapStats> {
        let Response { ok, stats, users } = raw;

        if ok != 1 {
            return Err(ApiError::NotOk(ok).into());
        }

        Ok(MapStats {
            rooms: stats
                .into_iter()
                .map(|(room_name, room_data)| {
                    let RoomResponse {
                        status,
                        own: owner,
                        novice,
                        open_time,
                        sign,
                        hard_sign,
                    } = room_data;
                    if status == "out of borders" {
                        // Oddity in Screeps: for shard0, all rooms which are out of bounds are simply left out of
                        // the result. For shard1, room names which would exist in shard0, but don't exist in shard1
                        // return an empty "out of bounds" status.
                        return Ok(None);
                    } else if status != "normal" {
                        return Err(
                            ApiError::MalformedResponse(format!(
                                "expected room status for \"{}\" to be \
                                 \"normal\", found \"{}\"",
                                room_name,
                                status
                            )).into(),
                        );
                    }

                    let info = RoomInfo {
                        name: RoomName::new(&room_name)?,
                        state: data::RoomState::from_data(time::get_time(), novice, open_time)?,
                        owner: owner,
                        // turn Option<Result<A, B>> into Result<Option<A>, B>
                        sign: sign,
                        hard_sign: hard_sign,
                        _phantom: PhantomData,
                    };

                    Ok(Some(info))
                })
                .flat_map(|result| match result {
                    Ok(Some(v)) => Some(Ok(v)),
                    Ok(None) => None,
                    Err(e) => Some(Err(e)),
                })
                .collect::<ScapiResult<_>>()?,
            users: users
                .into_iter()
                .map(|(user_id, user_data)| {
                    let UserResponse {
                        badge,
                        _id: user_id2,
                        username,
                    } = user_data;
                    if user_id != user_id2 {
                        return Err(
                            ApiError::MalformedResponse(format!(
                                "expected user id object key to match user \
                                 id, {} != {}",
                                user_id,
                                user_id2
                            )).into(),
                        );
                    }

                    let info = UserInfo {
                        user_id: user_id,
                        username: username,
                        badge: badge,
                        _phantom: PhantomData,
                    };

                    Ok(info)
                })
                .collect::<ScapiResult<_>>()?,
            _phantom: PhantomData,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::MapStats;
    use EndpointResult;
    use serde_json;

    fn test_parse(json: serde_json::Value) {
        let response = serde_json::from_value(json).unwrap();

        let _ = MapStats::from_raw(response).unwrap();
    }

    #[test]
    fn parse_sample() {
        test_parse(json! ({
            "ok": 1,
            "stats": {
                "E14S78": {
                    "own": {
                        "level": 0,
                        "user": "57fbb4ada59532b2194a4c4e"
                    },
                    "sign": {
                        "time": 18325590,
                        "text": "[Ypsilon Pact] Quad claimed: unauthorised rooms may be removed.",
                        "user": "57fbb4ada59532b2194a4c4e",
                        "datetime": 1490752580310i64
                    },
                    "status": "normal",
                    "novice": 1485278202869i64
                },
                "E15N52": {
                    "own": {
                        "level": 8,
                        "user": "57874d42d0ae911e3bd15bbc"
                    },
                    "openTime": "1474674699273",
                    "status": "normal",
                    "novice": 1475538699273i64
                },
                "E19S81": {
                    "status": "normal",
                    "novice": 1491937635414i64
                },
                "E19S79": {
                    "own": {
                        "level": 3,
                        "user": "57e0dde6adafdf710cc02af0"
                    },
                    "sign": {
                        "time": 18318966,
                        "text": "Outer reach settlement",
                        "user": "57e0dde6adafdf710cc02af0",
                        "datetime": 1490723256463i64
                    },
                    "status": "normal",
                    "novice": 1485278202869i64,
                    "safeMode": true
                },
                "W6S67": {
                    "sign": {
                        "time": 16656131,
                        "text": "I have plans for this block",
                        "user": "57c7df771d90a0c561977377",
                        "datetime": 1484071532985i64
                    },
                    "status": "normal",
                    "novice": 1482080519526i64,
                    "hardSign": {
                        "time": 18297994,
                        "endDatetime": 1490978122587i64,
                        "text": "A new Novice Area is being planned somewhere in this sector. \
                                 Please make sure all important rooms are reserved.",
                        "datetime": 1490632558393i64
                    }
                }
            },
            "gameTime": 18325591,
            "users": {
                "57e0dde6adafdf710cc02af0": {
                    "username": "Pav234",
                    "_id": "57e0dde6adafdf710cc02af0",
                    "badge": {
                        "color1": "#25009c",
                        "flip": false,
                        "param": 100,
                        "color3": "#00c7ff",
                        "type": 16,
                        "color2": "#00c7ff"
                    }
                },
                "57fbb4ada59532b2194a4c4e": {
                    "username": "Parthon",
                    "_id": "57fbb4ada59532b2194a4c4e",
                    "badge": {
                        "color1": "#0066ff",
                        "flip": false,
                        "param": -6,
                        "color3": "#2b2b2b",
                        "type": 16,
                        "color2": "#00ddff"
                    }
                },
                "57c7df771d90a0c561977377": {
                    "username": "ChaosDMG",
                    "_id": "57c7df771d90a0c561977377",
                    "badge": {
                        "color1": "#f25c00",
                        "flip": false,
                        "param": 0,
                        "color3": "#f7efe2",
                        "type": 17,
                        "color2": "#f9a603"
                    }
                },
                "57874d42d0ae911e3bd15bbc": {
                    "username": "daboross",
                    "_id": "57874d42d0ae911e3bd15bbc",
                    "badge": {
                        "color1": "#260d0d",
                        "flip": false,
                        "param": -100,
                        "color3": "#ffe56d",
                        "type": 21,
                        "color2": "#6b2e41"
                    }
                }
            }
        }));
    }
}
