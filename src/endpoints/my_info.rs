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

use serde_json;
use std::marker::PhantomData;

/// User info result struct.
#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Response {
    pub ok: i32,
    _id: String,
    username: String,
    password: bool,
    cpu: i32,
    gcl: i32,
    credits: f64,
    lastChargeTime: Option<String>,
    lastTweetTime: Option<String>,
    badge: Option<serde_json::Value>,
    github: Option<serde_json::Value>,
    twitter: Option<serde_json::Value>,
    notifyPrefs: Option<serde_json::Value>,
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

impl From<Response> for MyInfo {
    fn from(result: Response) -> MyInfo {
        MyInfo {
            user_id: result._id,
            username: result.username,
            has_password: result.password,
            cpu: result.cpu,
            gcl: result.gcl,
            credits: result.credits,
            _phantom: PhantomData,
        }
    }
}
