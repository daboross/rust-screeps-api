//! Interpreting generic template calls.

use EndpointResult;
use data;
use error::{Result, ApiError};
use std::marker::PhantomData;

/// Call raw result.
#[derive(Deserialize, Clone, Hash, Debug)]
#[allow(non_snake_case)]
#[doc(hidden)]
pub struct Response {
    ok: i32,
}

/// Call info
#[derive(Clone, Hash, Debug)]
pub struct CallInfo {
    /// Phantom data in order to allow adding any additional fields in the future.
    _phantom: PhantomData<()>,
}

impl EndpointResult for CallInfo {
    type RequestResult = Response;
    type ErrorResult = data::ApiError;

    fn from_raw(raw: Response) -> Result<CallInfo> {
        let Response { ok } = raw;

        if ok != 1 {
            return Err(ApiError::NotOk(ok).into());
        }

        Ok(CallInfo { _phantom: PhantomData })
    }
}

#[cfg(test)]
mod tests {
    use super::CallInfo;
    use EndpointResult;
    use serde_json;

    fn test_parse(json: serde_json::Value) {
        let response = serde_json::from_value(json).unwrap();

        let _ = CallInfo::from_raw(response).unwrap();
    }

    #[test]
    fn parse_sample() {
        test_parse(json! ({
            "ok": 1,
        }));
    }
}
