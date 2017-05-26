//! Interpreting room status results.

use EndpointResult;
use data::{self, RoomState, RoomName};
use error::{Result, ApiError};
use std::marker::PhantomData;
use time;

/// Room overview raw result.
#[derive(Deserialize, Clone, Hash, Debug)]
#[doc(hidden)]
pub struct Response {
    ok: i32,
    room: Option<InnerRoom>,
}

#[derive(Deserialize, Clone, Hash, Debug)]
struct InnerRoom {
    /// The room's name
    _id: String,
    /// The "status" string, usually "normal"? Unknown what else it could be.
    status: String,
    /// The end time for the novice area this room is or was last in.
    novice: Option<data::StringNumberTimeSpec>,
    /// The time this room will open or did open into the novice area as a second tier novice room.
    #[serde(rename = "openTime")]
    open_time: Option<data::StringNumberTimeSpec>,
}

/// Struct describing the status of a room
#[derive(Clone, Hash, Debug)]
pub struct RoomStatus {
    /// The name of the room, or None if the room does not exist.
    pub room_name: Option<RoomName>,
    /// The state of the room, determined by comparing the API response timestamps with the current UTC time, as
    /// retrieved from the system.
    pub state: RoomState,
    /// Phantom data in order to allow adding any additional fields in the future
    _phantom: PhantomData<()>,
}

impl EndpointResult for RoomStatus {
    type RequestResult = Response;
    type ErrorResult = data::ApiError;

    fn from_raw(raw: Response) -> Result<RoomStatus> {
        let Response { ok, room } = raw;

        if ok != 1 {
            return Err(ApiError::NotOk(ok).into());
        }

        let InnerRoom { _id: room_name, status, novice, open_time } = match room {
            Some(v) => v,
            None => {
                return Ok(RoomStatus {
                    room_name: None,
                    state: RoomState::non_existant(),
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

        let state = RoomState::from_data(time::get_time(), novice, open_time)?;

        Ok(RoomStatus {
            room_name: Some(RoomName::new(&room_name)?),
            state: state,
            _phantom: PhantomData,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::RoomStatus;
    use EndpointResult;
    use serde_json;

    fn test_parse(json: serde_json::Value) {
        let response = serde_json::from_value(json).unwrap();

        let _ = RoomStatus::from_raw(response).unwrap();
    }

    #[test]
    fn parse_sample_novice_room() {
        test_parse(json! ({
            "ok": 1,
            "room": {
                "_id": "W6S83",
                "status": "normal",
                "novice": 1488394267175i64
            }
        }));
    }

    #[test]
    fn parse_sample_highway_room() {
        test_parse(json! ({
            "ok": 1,
            "room": {
                "_id": "E0N0",
                "status": "normal"
            }
        }));
    }

    #[test]
    fn parse_sample_center_novice_room() {
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
