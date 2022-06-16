//! Interpreting room terrain results.

use serde::Serialize;

use crate::{
    data,
    error::{ApiError, Result},
    EndpointResult, InnerResponse, RoomTerrain,
};

#[derive(Serialize, Clone, Debug)]
pub(crate) struct RoomsTerrainRequest<'a> {
    pub rooms: &'a [&'a str],
}

#[derive(serde::Deserialize, Clone, Hash, Debug)]
pub(crate) struct RoomsTerrainResponse {
    ok: i32,
    rooms: Option<Vec<InnerResponse>>,
}

/// Structure describing the terrain of a list of rooms
#[derive(Clone, Debug)]
pub struct RoomsTerrain {
    /// Rooms
    pub rooms: Vec<RoomTerrain>,
}

impl EndpointResult for RoomsTerrain {
    type RequestResult = RoomsTerrainResponse;
    type ErrorResult = data::ApiError;

    fn from_raw(raw: RoomsTerrainResponse) -> Result<RoomsTerrain> {
        let RoomsTerrainResponse {
            ok,
            rooms: terrain_array,
        } = raw;

        if ok != 1 {
            return Err(ApiError::NotOk(ok).into());
        }
        let terrain_array = match terrain_array {
            Some(v) => v,
            None => return Err(ApiError::MissingField("rooms").into()),
        };

        Ok(RoomsTerrain {
            rooms: terrain_array
                .into_iter()
                .map(|i| i.into_terrain().unwrap_or_default())
                .collect(),
        })
    }
}
