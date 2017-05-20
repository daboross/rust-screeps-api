//! Interpreting user self information.

use EndpointResult;
use data::{self, Badge};
use error::{ApiError, Result};
use std::marker::PhantomData;

/// User info raw result.
#[derive(Deserialize, Clone, Debug)]
#[doc(hidden)]
pub struct Response {
    ok: i32,
    _id: String,
    username: String,
    password: bool,
    cpu: i32,
    gcl: u64,
    credits: f64,
    // These can be added if needed
    // lastChargeTime: Option<String>,
    // lastTweetTime: Option<String>,
    // github: Option<serde_json::Value>,
    // twitter: Option<serde_json::Value>,
    // notifyPrefs: Option<serde_json::Value>,
    badge: Badge,
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
    /// This user's current total count of GCL points (perform calculation to find actual gcl level).
    pub gcl_points: u64,
    /// This user's current credit balance.
    pub credits: f64,
    /// Phantom data in order to allow adding any additional fields in the future.
    _phantom: PhantomData<()>,
}

impl EndpointResult for MyInfo {
    type RequestResult = Response;
    type ErrorResult = data::ApiError;

    fn from_raw(raw: Response) -> Result<MyInfo> {
        let Response { ok, _id: user_id, username, password, cpu, gcl, credits, .. } = raw;

        if ok != 1 {
            return Err(ApiError::NotOk(ok).into());
        }
        Ok(MyInfo {
            user_id: user_id,
            username: username,
            has_password: password,
            cpu: cpu,
            gcl_points: gcl,
            credits: credits,
            _phantom: PhantomData,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::MyInfo;
    use EndpointResult;
    use serde_json;

    fn test_parse(json: serde_json::Value) {
        let response = serde_json::from_value(json).unwrap();

        let _ = MyInfo::from_raw(response).unwrap();
    }

    #[test]
    fn parse_sample_info() {
        test_parse(json! ({
            "_id": "57874d42d0ae911e3bd15bbc",
            "badge": {
                "color1": "#260d0d",
                "color2": "#6b2e41",
                "color3": "#ffe56d",
                "flip": false,
                "param": -100,
                "type": 21
            },
            "cpu": 170,
            "credits": 0,
            "email": "daboross@daboross.net",
            "gcl": 571069296,
            "github": {
                "id": "1152146",
                "username": "daboross"
            },
            "lastRespawnDate": 1475270405700i64,
            "money": 3957697.9500000584f64,
            "notifyPrefs": {
                "errorsInterval": 0
            },
            "ok": 1,
            "password": true,
            "promoPeriodUntil": 1471635211172i64,
            "steam": {
                "displayName": "daboross",
                "id": "76561198033802814",
                "ownership": [
                    464350
                ]
            },
            "subscription": true,
            "subscriptionTokens": 0,
            "username": "daboross"
        }));
    }
}
