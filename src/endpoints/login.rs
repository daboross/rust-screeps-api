//! Endpoint to log in to the API

use EndpointResult;
use data;
use error::{Result, ApiError};
use std::borrow::Cow;

/// Login details
#[derive(Serialize, Debug)]
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
pub struct Response {
    ok: i32,
    token: Option<String>,
}

/// The result of a call to log in.
pub struct LoginResult {
    /// The token which can be used to make future authenticated API calls.
    pub token: String,
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
            Some(token) => Ok(LoginResult { token: token }),
            None => Err(ApiError::MissingField("token").into()),
        }
    }
}
