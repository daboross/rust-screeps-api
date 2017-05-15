//! Interpreting login responses.
use std::marker::PhantomData;

use EndpointResult;
use data;
use error::{Result, ApiError};
use std::borrow::Cow;

/// Login details
#[derive(Serialize, Debug, Clone)]
pub struct Details<'a> {
    /// The email or username to log in with (either works)
    pub email: Cow<'a, str>,
    /// The password to log in with (steam auth is not supported)
    pub password: Cow<'a, str>,
}

impl<'a> Details<'a> {
    /// Create a new login details with the given username and password
    pub fn new<'b, T1: Into<Cow<'b, str>>, T2: Into<Cow<'b, str>>>(email: T1, password: T2) -> Details<'b> {
        Details {
            email: email.into(),
            password: password.into(),
        }
    }
}

/// Login raw result.
#[derive(Deserialize, Debug)]
#[doc(hidden)]
pub struct Response {
    ok: i32,
    token: Option<String>,
}

/// The result of a call to log in.
#[derive(Debug, Clone)]
pub struct LoginResult {
    /// The token which can be used to make future authenticated API calls.
    pub token: String,
    /// Phantom data in order to allow adding any additional fields in the future.
    _phantom: PhantomData<()>,
}

impl EndpointResult for LoginResult {
    type RequestResult = Response;
    type ErrorResult = data::ApiError;

    fn from_raw(raw: Response) -> Result<LoginResult> {
        let Response { ok, token } = raw;

        if ok != 1 {
            return Err(ApiError::NotOk(ok).into());
        }
        match token {
            Some(token) => Ok(LoginResult {
                token: token,
                _phantom: PhantomData,
            }),
            None => Err(ApiError::MissingField("token").into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LoginResult;
    use EndpointResult;
    use serde_json;

    fn test_parse(json: serde_json::Value) {
        let response = serde_json::from_value(json).unwrap();

        let _ = LoginResult::from_raw(response).unwrap();
    }

    #[test]
    fn parse_sample_login_success() {
        test_parse(json! ({
            "ok": 1,
            "token": "c07924d3f556a355eba7cd59f4c21f670fda76c2",
        }));
    }
}
