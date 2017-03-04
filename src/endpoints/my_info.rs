//! User information retrieval

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

#[cfg(test)]
mod tests {
    use super::{Response, MyInfo};
    use EndpointResult;
    use serde_json;

    fn test_parse(json: serde_json::Value) {
        let response: Response = serde_json::from_value(json).unwrap();

        let _ = MyInfo::from_raw(response).unwrap();
    }

    #[test]
    fn test_daboross_result() {
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
