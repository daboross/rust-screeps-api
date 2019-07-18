//! Interpreting memory calls.
use crate::{
    data,
    error::{ApiError, Result},
    EndpointResult,
};

/// Call raw result.
#[derive(serde_derive::Deserialize, Clone, Hash, Debug)]
#[doc(hidden)]
pub(crate) struct Response {
    ok: i32,
    data: String,
}

/// Memory segment retrieval result
#[derive(Clone, Hash, Debug)]
pub(crate) struct MemorySegment {
    pub data: String,
    /// Phantom data in order to allow adding any additional fields in the future.
    _non_exhaustive: (),
}

impl EndpointResult for MemorySegment {
    type RequestResult = Response;
    type ErrorResult = data::ApiError;

    fn from_raw(raw: Response) -> Result<Self> {
        let Response { ok, data } = raw;

        if ok != 1 {
            return Err(ApiError::NotOk(ok).into());
        }

        Ok(MemorySegment {
            data,
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

        let _ = MemorySegment::from_raw(response).unwrap();
    }

    #[test]
    fn parse_sample() {
        test_parse(json! ({
            "ok": 1,
            "data": "asdf"
        }));
    }
}
