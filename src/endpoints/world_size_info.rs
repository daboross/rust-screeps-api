//! Interpreting generic template calls.
use serde::Deserialize;

use crate::{
    data,
    error::{ApiError, Result},
    EndpointResult,
};

#[derive(Deserialize, Clone, Hash, Debug)]
pub(crate) struct Response {
    ok: i32,
    width: usize,
    height: usize,
}

/// World size infomation
#[derive(Clone, Hash, Debug)]
pub struct WorldSizeInfo {
    /// The width of the world
    pub width: usize,
    /// The height of the world
    pub height: usize,
    /// Phantom data in order to allow adding any additional fields in the future.
    _non_exhaustive: (),
}

impl EndpointResult for WorldSizeInfo {
    type RequestResult = Response;
    type ErrorResult = data::ApiError;

    fn from_raw(raw: Response) -> Result<WorldSizeInfo> {
        let Response { ok, width, height } = raw;

        if ok != 1 {
            return Err(ApiError::NotOk(ok).into());
        }

        Ok(WorldSizeInfo {
            width,
            height,
            _non_exhaustive: (),
        })
    }
}
