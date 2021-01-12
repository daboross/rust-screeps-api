//! Interpreting memory calls.
use std::borrow::Cow;

use serde::Serialize;

use crate::{
    data,
    error::{ApiError, Result},
    EndpointResult,
};

/// Call raw result.
#[derive(serde::Deserialize, Clone, Hash, Debug)]
#[doc(hidden)]
pub(crate) struct Response {
    ok: i32,
}

/// SetMemorySegment details
#[derive(Serialize, Clone, Hash, Debug)]
pub struct SetMemorySegmentArgs<'a> {
    /// The segment to set.
    pub segment: u32,
    /// The shard to set it in (optional for private servers).
    pub shard: Option<Cow<'a, str>>,
    /// The data
    pub data: Cow<'a, str>,
}

/// Memory segment set result
#[derive(Clone, Hash, Debug)]
pub(crate) struct SetMemorySegment {
    /// Phantom data in order to allow adding any additional fields in the future.
    _non_exhaustive: (),
}

impl EndpointResult for SetMemorySegment {
    type RequestResult = Response;
    type ErrorResult = data::ApiError;

    fn from_raw(raw: Response) -> Result<Self> {
        let Response { ok } = raw;

        if ok != 1 {
            return Err(ApiError::NotOk(ok).into());
        }

        Ok(SetMemorySegment {
            _non_exhaustive: (),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EndpointResult;
    use serde_json;

    fn test_parse(json: serde_json::Value) {
        let response = serde_json::from_value(json).unwrap();

        let _ = SetMemorySegment::from_raw(response).unwrap();
    }

    #[test]
    fn parse_sample() {
        test_parse(json! ({
            "ok": 1,
        }));
    }
}
