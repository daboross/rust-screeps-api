//! Interpreting room terrain results.
use arrayvec::ArrayVec;
use serde::{Deserialize, Serialize};

use crate::{
    data,
    error::{ApiError, Result},
    EndpointResult,
};

/// Room overview raw result.
#[derive(serde::Deserialize, Clone, Hash, Debug)]
pub(crate) struct Response {
    ok: i32,
    terrain: Option<Vec<InnerResponse>>,
}

#[derive(serde::Deserialize, Clone, Hash, Debug)]
struct InnerResponse {
    // this is returned as part of the data, but what the heck is it even for?
    /// A cache key maybe?
    _id: String,
    /// Should be `terrain`.
    #[serde(rename = "type")]
    response_type: String,
    /// room name
    room: String,
    /// encoded data
    terrain: String,
}

/// Type of terrain
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash, Debug, PartialOrd, Ord)]
pub enum TerrainType {
    /// Plains terrain type, easy to move through
    Plains,
    /// Swamp terrain type, green, hard to move through
    Swamp,
    /// Wall terrain type, impossible to move through
    Wall,
    /// Wall with swamp underneath it, hard to move through
    SwampyWall,
}

/// A type alias for a single row in the terrain grid.
pub type TerrainRow = ArrayVec<[TerrainType; 50]>;

/// A type alias for a 50x50 terrain grid. `terrain[y_pos][x_pos]` can be used to access any terrain
/// from x 0-49, y 0-49.
pub type TerrainGrid = ArrayVec<[TerrainRow; 50]>;

/// Structure describing the terrain of a room
#[derive(Clone, Debug)]
pub struct RoomTerrain {
    /// The name of the room
    pub room_name: data::RoomName,
    /// Cache ID? Not sure exactly...
    pub response_id: String,
    /// A 50x50 grid of terrain squares. When coming from the API, this is guaranteed to be
    /// completely filled, and accessing any square between `(0, 0)` and `(49, 49)` inclusive will
    /// succeed.
    ///
    /// You can use `terrain[y_pos][x_pos]` to get any individual terrain square.
    pub terrain: TerrainGrid,
    /// Phantom data in order to allow adding any additional fields in the future.
    _non_exhaustive: (),
}

impl EndpointResult for RoomTerrain {
    type RequestResult = Response;
    type ErrorResult = data::ApiError;

    fn from_raw(raw: Response) -> Result<RoomTerrain> {
        let Response {
            ok,
            terrain: terrain_array,
        } = raw;

        if ok != 1 {
            return Err(ApiError::NotOk(ok).into());
        }

        let terrain_data = match terrain_array {
            Some(v) => match v.into_iter().next() {
                Some(v) => v,
                None => return Err(ApiError::MissingField("terrain.0").into()),
            },
            None => return Err(ApiError::MissingField("terrain").into()),
        };

        let InnerResponse {
            response_type,
            room: room_string,
            _id: response_id,
            terrain,
        } = terrain_data;

        if terrain.len() != 2500 {
            return Err(ApiError::MalformedResponse(format!(
                "expected response.terrain[0].\
                 terrain to be a 2500 byte string, found a {} byte string.",
                terrain.len()
            ))
            .into());
        }

        if response_type != "terrain" {
            return Err(ApiError::MalformedResponse(format!(
                "expected response.terrain[0].type \
                 to be 'terrain', found {:?}",
                response_type
            ))
            .into());
        }

        Ok(RoomTerrain {
            room_name: data::RoomName::new(&room_string)?,
            response_id: response_id,
            terrain: terrain
                .into_bytes()
                .chunks(50)
                .enumerate()
                .map(|(y, chunk)| {
                    chunk
                        .iter()
                        .enumerate()
                        .map(|(x, byte)| match *byte {
                            b'0' => Ok(TerrainType::Plains),
                            b'1' => Ok(TerrainType::Wall),
                            b'2' => Ok(TerrainType::Swamp),
                            b'3' => Ok(TerrainType::SwampyWall),
                            other => Err(ApiError::MalformedResponse(format!(
                                "expected terrain data to contain \
                                 only characters 0,1,2,3, found byte {} at x,y {},{}.",
                                other, x, y
                            ))
                            .into()),
                        })
                        .collect::<Result<_>>()
                })
                .collect::<Result<_>>()?,
            _non_exhaustive: (),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::RoomTerrain;
    use crate::EndpointResult;
    use serde_json;

    fn test_parse(json: serde_json::Value) {
        let response = serde_json::from_value(json).unwrap();

        let _ = RoomTerrain::from_raw(response).unwrap();
    }

    #[test]
    fn parse_sample_terrain() {
        test_parse(json! ({
            "ok": 1,
            "terrain": [
                {
                    "_id":
                    "579fa9920700be0674d2f893",
                    "terrain": "\
                    11111111111111111111111111111111111111111111111111\
                    11111111111111111111111111111111111111111111111111\
                    11111111111111111111111111111000000001111111111111\
                    11111111111111111111111111100000000000000111111111\
                    11111111111111111111111110000000002200000203111111\
                    11111111100111111111111000000013122200000002111111\
                    00111111100001111111100000000011130200000220011111\
                    00111111100000000002001110000013330000000000011111\
                    00011111112000000002001110000003100000000000011111\
                    00001111110001110002000000000000000000000000011111\
                    00000111111003131000000000000000000000000000011111\
                    00000011111003111100000000011111100000000022200000\
                    00000000200001111110000000111111110000000002222000\
                    00000000220001111110022000111111111000000222222200\
                    00000000220001111111022000011111111100000020132011\
                    00000000020001111111000000000111111110000021113111\
                    00000000000001111111100011100011111131000001111111\
                    11111111100001111111100011110011111111100000111111\
                    11111111110001111111110011111001111111100020111111\
                    11111111130000111111110013111003111111100220011111\
                    11111111110000031111110003133221111111100020011111\
                    11111111110000021111100202130223111111000000011111\
                    11111111110000000111000002022221111110000000111111\
                    00111111110000000000000200002001111100000000111111\
                    00111111110001110000000000002000110222000001111111\
                    00111111110011111000000000000000000000000001111111\
                    00111111110033111100020000001100000000000011111111\
                    00111111110001111000000000001110000000000011111111\
                    00111111100020000000000000000110000000000011111111\
                    00011111000220000000000011000000000000000011111111\
                    00001110000232000022000111100000000000000011111111\
                    00000000001133000022001111102000000000000011111111\
                    00000000111111100000011111100000000000000011111111\
                    00111111111111111000011111110000000000000011111111\
                    00111111111111111110111111110000002000000011111111\
                    00031111111111111113111111110000222111110001111111\
                    00001111111111111111111111110220003111110011111111\
                    00001111111111111111111111110000001111100211111111\
                    11000111111111111111111111110200000111000331111111\
                    11100111111111111111111111110220000000000111111111\
                    11100031111111111111111111110222000000001111111111\
                    11110011111111111111111111110000000000001111111111\
                    11110001111111111111111111132200000000011111111111\
                    11111001111111111111111111132200000200111111111111\
                    11111001111111111111111111100000000000111111111111\
                    11111301111111111111111111100000000000111111111111\
                    11111100111100111111111111100000000000111111111111\
                    11111100011100111111111111100000022000011111111111\
                    11111100000000111111111111100000000000001111111111\
                    11111100000000111111111111100000000000001111111111",
                    "type": "terrain",
                    "room": "E15N52"
                }
            ]
        }));
    }
}
