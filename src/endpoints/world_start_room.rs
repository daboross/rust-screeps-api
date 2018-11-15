//! Interpreting generic template calls.

use std::marker::PhantomData;

use data;
use error::{ApiError, Result};
use EndpointResult;

/// World start room raw result.
#[derive(Deserialize, Clone, Hash, Debug)]
#[doc(hidden)]
pub struct Response {
    ok: i32,
    room: Vec<String>,
}

/// Structure describing the shard and room the client should start at.
#[derive(Clone, Hash, Debug)]
pub struct WorldStartRoom {
    /// The room name to start viewing.
    pub room_name: String,
    /// The shard name to start viewing, or None if a shard was provided for the query or the server is out of date.
    pub shard: Option<String>,
    /// Phantom data in order to allow adding any additional fields in the future.
    _phantom: PhantomData<()>,
}

impl EndpointResult for WorldStartRoom {
    type RequestResult = Response;
    type ErrorResult = data::ApiError;

    fn from_raw(raw: Response) -> Result<WorldStartRoom> {
        let Response { ok, mut room } = raw;

        if ok != 1 {
            return Err(ApiError::NotOk(ok).into());
        }

        if room.len() < 1 {
            return Err(ApiError::MalformedResponse(
                "expected response.room to be an array of \
                 length 1 or greater, found empty array"
                    .into(),
            )
            .into());
        }

        let room_string = room.swap_remove(0);

        let (room_name, shard) = match room_string.find('/') {
            Some(split_at) => (
                room_string[(split_at + 1)..].to_owned(),
                Some(room_string[..split_at].to_owned()),
            ),
            None => (room_string, None),
        };

        Ok(WorldStartRoom {
            room_name: room_name,
            shard: shard,
            _phantom: PhantomData,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::WorldStartRoom;
    use serde_json;
    use EndpointResult;

    fn test_parse(json: serde_json::Value, expected_room: &str, expected_shard: Option<&str>) {
        let response = serde_json::from_value(json).unwrap();

        let result = WorldStartRoom::from_raw(response).unwrap();
        assert_eq!(&result.room_name, expected_room);
        assert_eq!(result.shard.as_ref().map(AsRef::as_ref), expected_shard);
    }

    #[test]
    fn parse_sample() {
        test_parse(
            json! ({
                "ok": 1,
                "room": [
                    "shard0/E4S61",
                ]
            }),
            "E4S61",
            Some("shard0"),
        );
    }

    #[test]
    fn parse_sample_no_shard() {
        test_parse(
            json! ({
                "ok": 1,
                "room": [
                    "E0N0",
                ]
            }),
            "E0N0",
            None,
        );
    }
}
