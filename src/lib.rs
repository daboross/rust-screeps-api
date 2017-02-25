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
    pub fn new<'b, T1: Into<Cow<'b, str>>, T2: Into<Cow<'b, str>>>(email: T1, password: T2) -> LoginDetails<'b> {
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

/// API Object, stores the current API token and allows access to making requests.
#[derive(Debug)]
pub struct API<'a> {
    pub url: hyper::Url,
    client: &'a hyper::Client,
    pub token: Option<String>,
}

impl<'a> API<'a> {
    pub fn new<'b>(client: &'b hyper::Client) -> API<'b> {
        API {
            //"https://httpbin.org/post"
            url: hyper::Url::parse("https://screeps.com/api").expect("expected pre-set url to parse, parsing failed"),
            client: client,
            token: None,
        }
    }

    pub fn with_url<'b, T: hyper::client::IntoUrl>(client: &'b hyper::Client, url: T) -> Result<API<'b>> {
        Ok(API {
            url: url.into_url()?,
            client: client,
            token: None,
        })
    }

    fn make_request<T: serde::Serialize, R: serde::Deserialize>(&mut self, endpoint: &str, request_text: &T) -> Result<R> {
        let body = serde_json::to_string(request_text)?;

        let mut headers = Headers::new();
        headers.set(ContentType::json());
        if let Some(ref token) = self.token {
            headers.set_raw("X-Token", vec![token.as_bytes().to_vec()]);
            headers.set_raw("X-Username", vec![token.as_bytes().to_vec()]);
        }

        let mut response = self.client.post(self.url.join(endpoint)?)
            .body(&body)
            .headers(headers)
            .send()?;

        if !response.status.is_success() {
            return Err(Error::new(response.status, Some(response.url.clone())));
        }

        let result: R = match serde_json::from_reader(&mut response) {
            Ok(v) => v,
            Err(e) => {
                return Err(Error::new(e, Some(response.url.clone())))
            }
        };

        Ok(result)
    }

    pub fn login(&mut self, login_details: &LoginDetails) -> Result<()> {
        let result: LoginResponse = self.make_request("/api/auth/signin", login_details)?;

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
    use ::error::{Error, ErrorType};
    extern crate hyper;
    extern crate hyper_rustls;
    use hyper::client::Client;
    use hyper::net::HttpsConnector;

    #[test]
    fn anonymous_creation() {
        let client = Client::with_connector(HttpsConnector::new(hyper_rustls::TlsClient::new()));
        let _unused = API::new(&client);
    }

    #[test]
    fn login_creation_auth_failure() {
        let client = Client::with_connector(HttpsConnector::new(hyper_rustls::TlsClient::new()));
        let login = LoginDetails::new("username", "password");
        let mut api = API::new(&client);
        let result = api.login(&login);

        match result {
            Err(Error { err: ErrorType::Unauthorized, .. }) => println!("Success!"),
            other => panic!("Expected unauthorized error, found {:?}", other),
        }
    }
}
