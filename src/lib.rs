#[macro_use]
extern crate serde_derive;
extern crate hyper;
extern crate serde;
extern crate serde_json;

mod error;
mod endpoints;

use endpoints::{login, my_info};
pub use endpoints::login::Details as LoginDetails;
pub use endpoints::my_info::MyInfo;
pub use error::{Error, Result};
use error::ApiError;
use hyper::header::{Headers, ContentType};

/// API Object, stores the current API token and allows access to making requests.
#[derive(Debug)]
pub struct API<'a> {
    pub url: hyper::Url,
    client: &'a hyper::Client,
    pub token: Option<Vec<u8>>,
}

impl<'a> API<'a> {
    pub fn new<'b>(client: &'b hyper::Client) -> API<'b> {
        API {
            url: hyper::Url::parse("https://screeps.com/api/").expect("expected pre-set url to parse, parsing failed"),
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

    fn make_post_request<T: serde::Serialize, R: serde::Deserialize>(&mut self,
                                                                     endpoint: &str,
                                                                     request_text: T)
                                                                     -> Result<R> {
        let body = serde_json::to_string(&request_text)?;

        let mut headers = Headers::new();
        headers.set(ContentType::json());
        if let Some(ref token) = self.token {
            headers.set_raw("X-Token", vec![token.clone()]);
            headers.set_raw("X-Username", vec![token.clone()]);
        }

        let mut response = self.client
            .post(self.url.join(endpoint)?)
            .body(&body)
            .headers(headers)
            .send()?;

        if !response.status.is_success() {
            return Err(Error::new(response.status, Some(response.url.clone())));
        }

        if let Some(token_vec) = response.headers.get_raw("X-Token") {
            if let Some(token_bytes) = token_vec.first() {
                self.token = Some(Vec::from(&**token_bytes));
            }
        }

        let result: R = match serde_json::from_reader(&mut response) {
            Ok(v) => v,
            Err(e) => return Err(Error::new(e, Some(response.url.clone()))),
        };

        Ok(result)
    }
    fn make_get_request<R: serde::Deserialize>(&mut self, endpoint: &str) -> Result<R> {
        let mut headers = Headers::new();
        headers.set(ContentType::json());
        if let Some(ref token) = self.token {
            headers.set_raw("X-Token", vec![token.clone()]);
            headers.set_raw("X-Username", vec![token.clone()]);
        }

        let mut response = self.client
            .get(self.url.join(endpoint)?)
            .headers(headers)
            .send()?;

        if !response.status.is_success() {
            return Err(Error::new(response.status, Some(response.url.clone())));
        }

        if let Some(token_vec) = response.headers.get_raw("X-Token") {
            if let Some(token_bytes) = token_vec.first() {
                self.token = Some(Vec::from(&**token_bytes));
            }
        }

        let result: R = match serde_json::from_reader(&mut response) {
            Ok(v) => v,
            Err(e) => return Err(Error::new(e, Some(response.url.clone()))),
        };

        Ok(result)
    }

    pub fn login(&mut self, login_details: &LoginDetails) -> Result<()> {
        let result: login::Response = self.make_post_request("auth/signin", login_details)?;

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

    pub fn my_info(&mut self) -> Result<MyInfo> {
        let result: my_info::Response = self.make_get_request("auth/me")?;
        Ok(result.into())
    }
}

#[cfg(test)]
mod tests {
    use {API, LoginDetails};
    use error::{Error, ErrorType};
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

        match api.login(&login) {
            Err(Error { err: ErrorType::Unauthorized, .. }) => (),
            other => panic!("Expected unauthorized error, found {:?}", other),
        }
    }
}
