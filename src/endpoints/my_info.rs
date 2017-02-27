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

use data::Badge;
use error::ApiError;
use serde_json;
use std::marker::PhantomData;

/// User info result struct.
#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Response {
    pub ok: i32,
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

impl Response {
    pub fn into_info(self) -> Result<MyInfo, ApiError> {
        let Response { ok, _id, username, password, cpu, gcl, credits, .. } = self;

        if ok != 1 {
            return Err(ApiError::NotOk(ok));
        }
        let user_id = match _id {
            Some(v) => v,
            None => return Err(ApiError::MissingField("_id")),
        };
        let username = match username {
            Some(v) => v,
            None => return Err(ApiError::MissingField("username")),
        };
        let password = match password {
            Some(v) => v,
            None => return Err(ApiError::MissingField("password")),
        };
        let cpu = match cpu {
            Some(v) => v,
            None => return Err(ApiError::MissingField("cpu")),
        };
        let gcl = match gcl {
            Some(v) => v,
            None => return Err(ApiError::MissingField("gcl")),
        };
        let credits = match credits {
            Some(v) => v,
            None => return Err(ApiError::MissingField("cerdits")),
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

#[derive(Clone, Debug)]
pub struct MyInfo {
    pub user_id: String,
    pub username: String,
    pub has_password: bool,
    pub cpu: i32,
    pub gcl: i32,
    pub credits: f64,
    #[doc(hidden)]
    pub _phantom: PhantomData<()>,
}
