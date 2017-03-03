//! User information retrieval
//! endpoint: auth/me
//! Data: {
//!     ok,
//!     _id,
//!     email,
//!     username,
//!     cpu,
//!     badge: { type, color1, color2, color3, param, flip },
//!     password,
//!     notifyPrefs: { sendOnline, errorsInterval, disabledOnMessages, disabled, interval },
//!     gcl,
//!     credits,
//!     lastChargeTime,
//!     lastTweetTime,
//!     github: { id, username },
//!     twitter: { username, followers_count }
//! }
use EndpointResult;
use data::{self, Badge};
use error::{ApiError, Result};
use serde_json;
use std::marker::PhantomData;

/// User info raw result.
#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Response {
    ok: i32,
    _id: Option<String>,
    username: Option<String>,
    password: Option<bool>,
    cpu: Option<i32>,
    gcl: Option<i32>,
    credits: Option<f64>,
    lastChargeTime: Option<String>,
    lastTweetTime: Option<String>,
    badge: Option<Badge>,
    github: Option<serde_json::Value>,
    twitter: Option<serde_json::Value>,
    notifyPrefs: Option<serde_json::Value>,
}

/// Result of a call to get the information for the logged in user.
#[derive(Clone, Debug)]
pub struct MyInfo {
    /// Unique user ID referring to this user.
    pub user_id: String,
    /// Unique username referring to this user.
    pub username: String,
    /// Whether or not a password can be used to login for this user.
    pub has_password: bool,
    /// This user's current CPU allowance.
    pub cpu: i32,
    /// This user's current GCL.
    pub gcl: i32,
    /// This user's current credit balance.
    pub credits: f64,
    /// Phantom data in order to allow adding any additional fields in the future.
    #[doc(hidden)]
    pub _phantom: PhantomData<()>,
}

impl EndpointResult for MyInfo {
    type RequestResult = Response;
    type ErrorResult = data::ApiError;

    fn from_raw(raw: Response) -> Result<MyInfo> {
        let Response { ok, _id, username, password, cpu, gcl, credits, .. } = raw;

        if ok != 1 {
            return Err(ApiError::NotOk(ok).into());
        }
        let user_id = match _id {
            Some(v) => v,
            None => return Err(ApiError::MissingField("_id").into()),
        };
        let username = match username {
            Some(v) => v,
            None => return Err(ApiError::MissingField("username").into()),
        };
        let password = match password {
            Some(v) => v,
            None => return Err(ApiError::MissingField("password").into()),
        };
        let cpu = match cpu {
            Some(v) => v,
            None => return Err(ApiError::MissingField("cpu").into()),
        };
        let gcl = match gcl {
            Some(v) => v,
            None => return Err(ApiError::MissingField("gcl").into()),
        };
        let credits = match credits {
            Some(v) => v,
            None => return Err(ApiError::MissingField("cerdits").into()),
        };

        Ok(MyInfo {
            user_id: user_id,
            username: username,
            has_password: password,
            cpu: cpu,
            gcl: gcl,
            credits: credits,
            _phantom: PhantomData,
        })
    }
}
