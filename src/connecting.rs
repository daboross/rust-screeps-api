//! Semi-internal functionality related to networking.
use futures::stream::TryStreamExt;
use url::Url;

use crate::{EndpointResult, Error, TokenStorage};

/// Interpret a hyper result as the result from a specific endpoint.
///
/// The returned future will:
///
/// - Wait for the hyper request to finish
/// - Wait for hyper request body, collecting it into a single chunk
/// - Parse JSON body as the given `EndpointResult`, and return result/error.
///
/// All errors returned will have the given `Url` contained as part of the context.
///
/// # Parameters
///
/// - `url`: url that is being queried, used only for error and warning messages
/// - `tokens`: where to put any tokens that were returned, if any
/// - `response`: actual hyper response that we're interpreting
pub(crate) async fn interpret<R>(
    tokens: TokenStorage,
    url: Url,
    response: hyper::client::ResponseFuture,
) -> Result<R, Error>
where
    R: EndpointResult,
{
    let response = response
        .await
        .map_err(|e| Error::with_url(e, Some(url.clone())))?;
    if let Some(token) = response.headers().get("X-Token") {
        debug!(
            "replacing stored auth_token with token returned from API: {:?}",
            token.to_str()
        );
        tokens.set(token.as_bytes().to_owned().into());
    }
    let status = response.status();

    let data: Vec<u8> = response
        .into_body()
        .try_fold(Vec::new(), |mut data, chunk| async move {
            data.extend_from_slice(&chunk);
            Ok(data)
        })
        .await
        .map_err(|e| Error::with_url(e, Some(url.clone())))?;
    let data = bytes::Bytes::from(data);
    let json_result = serde_json::from_slice(&data);

    // insert this check here so we can include response body in status errors.
    if !status.is_success() {
        if let Ok(json) = json_result {
            return Err(Error::with_json(status, Some(url), Some(json)));
        } else {
            return Err(Error::with_body(status, Some(url), Some(data)));
        }
    }

    let json = match json_result {
        Ok(v) => v,
        Err(e) => return Err(Error::with_body(e, Some(url), Some(data))),
    };
    let parsed = match deserialize_with_warnings::<R>(&json, &url) {
        Ok(v) => v,
        Err(e) => return Err(Error::with_json(e, Some(url), Some(json))),
    };

    R::from_raw(parsed).map_err(|e| Error::with_json(e, Some(url), Some(json)))
}

fn deserialize_with_warnings<T: EndpointResult>(
    input: &serde_json::Value,
    url: &Url,
) -> Result<T::RequestResult, Error> {
    let mut unused = Vec::new();

    let res = match serde_ignored::deserialize::<_, _, T::RequestResult>(input, |path| {
        unused.push(path.to_string())
    }) {
        Ok(v) => Ok(v),
        Err(e1) => {
            unused.clear();
            match serde_ignored::deserialize::<_, _, T::ErrorResult>(input, |path| {
                unused.push(path.to_string())
            }) {
                Ok(v) => Err(Error::with_json(v, Some(url.clone()), Some(input.clone()))),
                // Favor the primary parsing error if one occurs parsing the error type as well.
                Err(_) => Err(Error::with_json(e1, Some(url.clone()), Some(input.clone()))),
            }
        }
    };

    if !unused.is_empty() {
        warn!(
            "screeps API lib didn't parse some data retrieved from: {}\n\
             full data: {}\n\
             unparsed fields: {:#?}",
            url,
            serde_json::to_string_pretty(input).unwrap(),
            unused
        );
    }

    res
}
