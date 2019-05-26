//! Creating registration calls and interpreting registration results.
use std::borrow::Cow;

use crate::{
    data,
    error::{ApiError, Result},
    EndpointResult,
};

/// Registration details
#[derive(Serialize, Clone, Hash, Debug)]
pub struct RegistrationDetails<'a> {
    /// The username to register.
    username: Cow<'a, str>,
    /// The email to register with, or None.
    email: Option<Cow<'a, str>>,
    /// The password to register with.
    password: Cow<'a, str>,
}

impl<'a> RegistrationDetails<'a> {
    /// Create a new registration details with the given username and password
    pub fn new<T, U>(username: T, password: U) -> Self
    where
        T: Into<Cow<'a, str>>,
        U: Into<Cow<'a, str>>,
    {
        RegistrationDetails {
            username: username.into(),
            email: None,
            password: password.into(),
        }
    }
    /// Create a new registration details with the given username and password
    pub fn with_email<T, U, V>(username: T, password: U, email: V) -> Self
    where
        T: Into<Cow<'a, str>>,
        U: Into<Cow<'a, str>>,
        V: Into<Cow<'a, str>>,
    {
        RegistrationDetails {
            username: username.into(),
            password: password.into(),
            email: Some(email.into()),
        }
    }
}

/// Raw registration response.
#[derive(serde_derive::Deserialize, Clone, Hash, Debug)]
pub(crate) struct Response {
    ok: i32,
}

/// Registration success response.
#[derive(Clone, Hash, Debug)]
pub struct RegistrationSuccess {
    /// Phantom data in order to allow adding any additional fields in the future.
    _non_exhaustive: (),
}

impl EndpointResult for RegistrationSuccess {
    type RequestResult = Response;
    type ErrorResult = data::ApiError;

    fn from_raw(raw: Response) -> Result<RegistrationSuccess> {
        let Response { ok } = raw;

        if ok != 1 {
            return Err(ApiError::NotOk(ok).into());
        }

        Ok(RegistrationSuccess {
            _non_exhaustive: (),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::RegistrationSuccess;
    use crate::EndpointResult;
    use serde_json;

    fn test_parse(json: serde_json::Value) {
        let response = serde_json::from_value(json).unwrap();

        let _ = RegistrationSuccess::from_raw(response).unwrap();
    }

    #[test]
    fn parse_sample() {
        test_parse(json! ({
            "ok": 1,
        }));
    }
}
