//! Room terrain retrieval. This assumes getting terrain with encoded=true in the request.

// TODO: testing "error" responses for all other queries!

use EndpointResult;
use data;
use error::{Result, ApiError};
use std::marker::PhantomData;
use time;

/// Room overview raw result.
#[derive(Deserialize, Debug)]
pub struct Response {
    ok: i32,
    room: Option<InnerRoom>,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct InnerRoom {
    /// The room's name
    _id: String,
    /// The "status" string, usually "normal"? Unknown what else it could be.
    status: String,
    /// The end time for the novice area this room is or was last in.
    novice: Option<StringNumberTimeSpec>,
    /// The time this room will open or did open into the novice area as a second tier novice room.
    openTime: Option<StringNumberTimeSpec>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum StringNumberTimeSpec {
    String(String),
    Number(i64),
}

impl StringNumberTimeSpec {
    fn to_timespec(&self) -> Result<time::Timespec> {
        let time = match *self {
            StringNumberTimeSpec::String(ref s) => {
                match s.parse() {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(ApiError::MalformedResponse(format!("expected timestamp string to be a \
                                                                        valid integer, found {}: {:?}",
                                                                       s,
                                                                       e))
                            .into())
                    }
                }
            }
            StringNumberTimeSpec::Number(v) => v,
        };

        Ok(time::Timespec::new(time, 0))
    }
}

/// A room state, returned by room status.
/// Note that the API itself will return timestamps for "novice end" and "open time" even when the room is no longer
/// novice, so the current system's knowledge of utc time is used to determine whether a room is novice or not.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RoomState {
    /// Room name does not exist.
    Nonexistant,
    /// Room exists and terrain has been generated, but room is completely closed.
    Closed,
    /// Room exists, is open, and is not part of a novice area.
    Open,
    /// Room is part of a novice area.
    Novice {
        /// The time when the novice area will expire.
        end_time: time::Timespec,
    },
    /// Room is part of a "second tier" novice area, which is closed, but when opened will be part of a novice area
    /// which already has other open rooms.
    SecondTierNovice {
        /// The time this room will open and join the surrounding novice area rooms.
        room_open_time: time::Timespec,
        /// The time the novice area this room is a part of will expire.
        end_time: time::Timespec,
    },
}

/// Struct describing the status of a room
#[derive(Clone, Debug)]
pub struct RoomStatus {
    /// The name of the room, or None if the room does not exist.
    pub room_name: Option<String>,
    /// The state of the room, determined by comparing the API response timestamps with the current UTC time, as
    /// retrieved from the system.
    pub state: RoomState,
    /// Phantom data in order to allow adding any additional fields in the future
    #[doc(hidden)]
    pub _phantom: PhantomData<()>,
}

impl EndpointResult for RoomStatus {
    type RequestResult = Response;
    type ErrorResult = data::ApiError;

    fn from_raw(raw: Response) -> Result<RoomStatus> {
        let Response { ok, room } = raw;

        if ok != 1 {
            return Err(ApiError::NotOk(ok).into());
        }

        let InnerRoom { _id: room_name, status, novice, openTime: open_time } = match room {
            Some(v) => v,
            None => {
                return Ok(RoomStatus {
                    room_name: None,
                    state: RoomState::Nonexistant,
                    _phantom: PhantomData,
                });
            }
        };

        if status != "normal" {
            return Err(ApiError::MalformedResponse(format!("expected room status to be \"normal\", \
                                                            found \"{}\".",
                                                           &status))
                .into());
        }

        // This turns Option<Result<A, B>> into Result<Option<A>, B>
        let novice_time_spec = novice.map_or(Ok(None), |t| t.to_timespec().map(Some))?;
        let open_time_spec = open_time.map_or(Ok(None), |t| t.to_timespec().map(Some))?;
        let sys_time = time::get_time();

        let state = match novice_time_spec {
            Some(n) if n > sys_time => {
                match open_time_spec {
                    Some(o) if o > sys_time => {
                        RoomState::SecondTierNovice {
                            room_open_time: o,
                            end_time: n,
                        }
                    }
                    _ => RoomState::Novice { end_time: n },
                }
            }
            Some(_) => RoomState::Open,
            None => RoomState::Open,
        };

        Ok(RoomStatus {
            room_name: Some(room_name),
            state: state,
            _phantom: PhantomData,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{Response, RoomStatus};
    use EndpointResult;
    use serde_json;

    fn test_parse(json: serde_json::Value) {
        let response: Response = serde_json::from_value(json).unwrap();

        let _ = RoomStatus::from_raw(response).unwrap();
    }

    #[test]
    fn test_room_with_novice() {
        test_parse(json! ({
            "ok": 1,
            "room": {
                "_id": "W6S83",
                "status": "normal",
                "novice": 1488394267175i64,
            }
        }));
    }

    #[test]
    fn test_highway_room() {
        test_parse(json! ({
            "ok": 1,
            "room": {
                "_id": "E0N0",
                "status": "normal"
            }
        }));
    }

    #[test]
    fn test_center_novice_room() {
        test_parse(json! ({
            "ok": 1,
            "room": {
                "_id": "E15N51",
                "status": "normal",
                "openTime": "1474674699273",
                "novice": 1475538699273i64
            }
        }));
    }
}
