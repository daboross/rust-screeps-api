//! Semi-internal functionality related to networking.
use std::fmt;

use {hyper, serde_json, serde_ignored};

use futures::{Poll, Future, Stream};
use url::Url;

use {Token, TokenStorage, EndpointType, Error};


/// Struct mirroring `hyper`'s FutureResponse, but with parsing that happens after the request is finished.
#[must_use = "futures do nothing unless polled"]
pub struct FutureResponse<R: EndpointType>(Box<Future<Item = R, Error = Error>>);

impl<R: EndpointType> fmt::Debug for FutureResponse<R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ScreepsFutureResponse")
            .field("inner", &"<boxed future>")
            .finish()
    }
}

impl<R> Future for FutureResponse<R>
    where R: EndpointType
{
    type Item = R;
    type Error = Error;

    fn poll(&mut self) -> Poll<R, Error> {
        self.0.poll()
    }
}

/// Interpret a hyper result as the result from a specific endpoint.
///
/// The returned future will:
///
/// - Wait for the hyper request to finish
/// - Send any auth token found in headers back to token storage
/// - Wait for hyper request body, collecting it into a single chunk
/// - Parse JSON body as the given `EndpointType`, and return result/error.
///
/// All errors returned will have the given `Url` contained as part of the context.
///
/// # Parameters
///
/// - `token_storage`: token storage to store any tokens that were refreshed.
/// - `used_token`: token that was used when sending this request, if any.
///   if the server doesn't return a new token, and this is Some, the inner
///   token will be returned to the token storage.
/// - `url`: url that is being queried, used only for error and warning messages.
/// - `response`: actual hyper response that we're interpreting
pub fn interpret<T, R>(token_storage: T,
                       used_token: Option<Token>,
                       url: Url,
                       response: hyper::client::FutureResponse)
                       -> FutureResponse<R>
    where T: TokenStorage,
          R: EndpointType
{
    FutureResponse(Box::new(response.then(move |result| {
            let new_result = match result {
                Ok(v) => Ok((token_storage, used_token, url, v)),
                Err(e) => Err(Error::with_url(e, Some(url))),
            };

            new_result
        })
        .and_then(|(token_storage, used_token, url, mut response)| {
            let token_to_return = {
                header! { (TokenHeader, "X-Token") => [String] }

                let new_token = response.headers_mut().remove::<TokenHeader>().map(|h| h.0);

                match new_token {
                    Some(token) => {
                        if token.is_empty() {
                            Some(token)
                        } else {
                            used_token
                        }
                    }
                    None => used_token,
                }
            };

            if let Some(token) = token_to_return {
                token_storage.return_token(token);
            }

            Ok((url, response))
        })
        .and_then(|(url, response)| {
            let status = response.status();

            response.body().concat2().then(move |result| match result {
                Ok(v) => Ok((status, url, v)),
                Err(e) => Err(Error::with_url(e, Some(url))),
            })
        })
        .and_then(|(status, url, data): (hyper::status::StatusCode, _, hyper::Chunk)| {
            let json_result = serde_json::from_slice(&data);

            // insert this check here so we can include response body in status errors.
            if !status.is_success() {
                if let Ok(json) = json_result {
                    return Err(Error::with_json(status, Some(url), Some(json)));
                } else {
                    return Err(Error::with_url(status, Some(url)));
                }
            }

            let parsed = match json_result {
                Ok(json) => deserialize_with_warnings::<R>(&json, &url)?,
                Err(e) => return Err(Error::with_url(e, Some(url))),
            };

            R::from_raw(parsed)
        })))
}

fn deserialize_with_warnings<T: EndpointType>(input: &serde_json::Value, url: &Url) -> Result<T::RequestResult, Error> {
    let mut unused = Vec::new();

    let res = match serde_ignored::deserialize::<_, _, T::RequestResult>(input, |path| unused.push(path.to_string())) {
        Ok(v) => Ok(v),
        Err(e1) => {
            unused.clear();
            match serde_ignored::deserialize::<_, _, T::ErrorResult>(input, |path| unused.push(path.to_string())) {
                Ok(v) => Err(Error::with_json(v, Some(url.clone()), Some(input.clone()))),
                // Favor the primary parsing error if one occurs parsing the error type as well.
                Err(_) => Err(Error::with_json(e1, Some(url.clone()), Some(input.clone()))),
            }
        }
    };


    if !unused.is_empty() {
        warn!("screeps API lib didn't parse some data retrieved from: {}\n\
               full data: {}\n\
               unparsed fields: {:?}",
              url,
              serde_json::to_string(input).unwrap(),
              unused);
    }

    res
}
