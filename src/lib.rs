#[macro_use]
extern crate serde_derive;
extern crate hyper;
extern crate serde;
extern crate serde_json;

mod error;

pub use error::{Error, ApiError, Result};
use std::borrow::Cow;
use hyper::header::{Headers, ContentType};


/// Login details
#[derive(Serialize, Debug)]
pub struct LoginDetails<'a> {
    /// The email or username to log in with (either works)
    pub email: Cow<'a, str>,
    /// The password to log in with (steam auth is not supported)
    pub password: Cow<'a, str>,
}

impl<'a> LoginDetails<'a> {
    /// Create a new login details with the given username and password
    pub fn new<'b, T: Into<Cow<'b, str>>>(email: T, password: T) -> LoginDetails<'b> {
        LoginDetails {
            email: email.into(),
            password: password.into()
        }
    }
}

#[derive(Deserialize, Debug)]
struct LoginResponse {
    ok: i32,
    token: Option<String>,
}

#[derive(Debug)]
pub struct API<'a> {
    token: Option<Cow<'a, str>>,
}

impl<'a> API<'a> {
    pub fn new_anonymous() -> API<'static> {
        API { token: None }
    }

    pub fn new_logged_in<'b>(client: &hyper::Client, login_details: &LoginDetails) -> Result<API<'b>> {
        let mut api = API::new_anonymous();

        api.get_token(client, login_details)?;

        Ok(api)
    }

    fn get_token(&mut self, client: &hyper::Client, login_details: &LoginDetails) -> Result<()> {
        let body = serde_json::to_string(&login_details)?;

        let mut headers = Headers::new();
        headers.set(ContentType::json());

        let mut response = client.post("https://screeps.com/api/auth/signin")
            .body(&body)
            .headers(headers)
            .send()?;

        if !response.status.is_success() {
            return Err(response.status.into());
        }

        let result: LoginResponse = serde_json::from_reader(&mut response)?;

        if result.ok != 1 {
            return Err(ApiError::NotOk(result.ok).into());
        }

        if let Some(token) = result.token {
            self.token = Some(token.into());
            Ok(())
        } else {
            Err(ApiError::MissingField("token").into())
        }
    }
}

#[cfg(test)]
mod tests {
    use ::{API, LoginDetails};
    extern crate hyper;
    extern crate hyper_rustls;
    use hyper::client::Client;
    use hyper::net::HttpsConnector;

    #[test]
    fn anonymous_creation() {
        let _unused = API::new_anonymous();
    }

    #[test]
    fn login_creation_auth_failure() {
        let client = Client::with_connector(HttpsConnector::new(hyper_rustls::TlsClient::new()));
        let login = LoginDetails::new("username", "password");
        let result = API::new_logged_in(&client, &login);

        match result {
            Err(::error::Error::StatusCode(hyper::status::StatusCode::Unauthorized)) => println!("Success!"),
            other => panic!("Expected unauthorized error, found {:?}", other),
        }
    }
}
