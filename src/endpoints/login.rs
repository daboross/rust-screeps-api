//! Interpreting login responses.
use std::borrow::Cow;
use std::marker::PhantomData;

use data;
use error::{ApiError, Result};

use {EndpointResult, Token, TokenStorage};

/// Login details
#[derive(Serialize, Clone, Hash, Debug)]
pub struct Details<'a> {
    /// The email or username to log in with (either works)
    pub email: Cow<'a, str>,
    /// The password to log in with (steam auth is not supported)
    pub password: Cow<'a, str>,
}

impl<'a> Details<'a> {
    /// Create a new login details with the given username and password
    pub fn new<T, U>(email: T, password: U) -> Self
    where
        T: Into<Cow<'a, str>>,
        U: Into<Cow<'a, str>>,
    {
        Details {
            email: email.into(),
            password: password.into(),
        }
    }
}

/// Login raw result.
#[derive(serde_derive::Deserialize, Clone, Hash, Debug)]
#[doc(hidden)]
pub struct Response {
    ok: i32,
    token: Option<String>,
}

/// The result of a call to log in.
#[must_use = "LoggedIn does not do anything unless registered in a token store"]
#[derive(Clone, Hash, Debug)]
pub struct LoggedIn {
    /// The token which can be used to make future authenticated API calls.
    pub token: Token,
    /// Phantom data in order to allow adding any additional fields in the future.
    _phantom: PhantomData<()>,
}

impl LoggedIn {
    /// Stores the token into the given token storage.
    pub fn return_to(self, storage: &TokenStorage) {
        storage.set(self.token);
    }
}

impl EndpointResult for LoggedIn {
    type RequestResult = Response;
    type ErrorResult = data::ApiError;

    fn from_raw(raw: Response) -> Result<LoggedIn> {
        let Response { ok, token } = raw;

        if ok != 1 {
            return Err(ApiError::NotOk(ok).into());
        }
        match token {
            Some(token) => Ok(LoggedIn {
                token: token.into(),
                _phantom: PhantomData,
            }),
            None => Err(ApiError::MissingField("token").into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LoggedIn;
    use serde_json;
    use EndpointResult;

    fn test_parse(json: serde_json::Value) {
        let response = serde_json::from_value(json).unwrap();

        let _ = LoggedIn::from_raw(response).unwrap();
    }

    #[test]
    fn parse_sample_login_success() {
        test_parse(json! ({
            "ok": 1,
            "token": "c07924d3f556a355eba7cd59f4c21f670fda76c2",
        }));
    }
}
