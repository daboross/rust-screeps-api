//! Error types for the screeps api.
extern crate hyper;
extern crate serde_json;

use self::ErrorType::*;
use std::error::Error as StdError;
use std::fmt;
use std::io;
use std::marker;

#[derive(Debug)]
/// Possible error types for library errors.
pub enum ErrorType {
    /// Unauthorized access. This is caused by either attempting to access a login-only endpoint without a token,
    /// attempting to access a login-only endpoint with an expired token, or providing incorrect login details to the
    /// login endpoint.
    Unauthorized,
    /// Error parsing a server response. This is most likely caused by the server providing unparsable JSON, but it
    /// could also be the server's API response structure has changed and no longer matches the expected data structure.
    SerdeJson(serde_json::error::Error),
    /// Error connecting to the server, or error parsing a URL provided.
    Hyper(hyper::error::Error),
    /// IO error.
    Io(io::Error),
    /// Error for when the server responds with a non-success status code.
    StatusCode(hyper::status::StatusCode),
    /// API Error. Either missing fields in the response that were expected, or the API did not include the regular
    /// `"ok": 1` which indicates success.
    Api(ApiError),
    /// A marker variant that tells the compiler that users of this enum cannot match it exhaustively.
    #[doc(hidden)]
    __Nonexhaustive,
}

/// Error deriving from some API call.
#[derive(Debug)]
pub struct Error {
    /// The type specifying what kind of error, and a detailed description if available.
    pub err: ErrorType,
    /// The whole URL which was being accessed when this error occurred (not included for URL parsing errors).
    pub url: Option<hyper::Url>,
    /// The json data from the request which resulted in this error (not included for URL or JSON parsing errors).
    pub json: Option<serde_json::Value>,
    /// Phantom data in order to allow adding any additional fields in the future.
    #[doc(hidden)]
    pub _phantom: marker::PhantomData<()>,
}

impl Error {
    /// Creates a new error from the given error and the given possible url.
    pub fn with_url<T: Into<Error>>(err: T, url: Option<hyper::Url>) -> Error {
        Error::with_json(err, url, None)
    }
    /// Creates a new error from the given error, the given possible url, and the given possible JSON data.
    pub fn with_json<T: Into<Error>>(err: T, url: Option<hyper::Url>, json: Option<serde_json::Value>) -> Error {
        let err = err.into();
        Error {
            err: err.err,
            url: url.or(err.url),
            json: json.or(err.json),
            _phantom: marker::PhantomData,
        }
    }
}

/// Result type for screeps API operations.
pub type Result<T> = ::std::result::Result<T, Error>;

impl From<ErrorType> for Error {
    fn from(err: ErrorType) -> Error {
        Error {
            err: err,
            url: None,
            json: None,
            _phantom: marker::PhantomData,
        }
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(err: serde_json::error::Error) -> Error {
        ErrorType::SerdeJson(err).into()
    }
}

impl From<hyper::error::Error> for Error {
    fn from(err: hyper::error::Error) -> Error {
        ErrorType::Hyper(err).into()
    }
}

impl From<hyper::error::ParseError> for Error {
    fn from(err: hyper::error::ParseError) -> Error {
        ErrorType::Hyper(hyper::Error::Uri(err)).into()
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        ErrorType::Io(err).into()
    }
}

impl From<hyper::status::StatusCode> for Error {
    fn from(code: hyper::status::StatusCode) -> Error {
        if code == hyper::status::StatusCode::Unauthorized {
            ErrorType::Unauthorized.into()
        } else {
            ErrorType::StatusCode(code).into()
        }
    }
}

impl From<ApiError> for Error {
    fn from(err: ApiError) -> Error {
        ErrorType::Api(err).into()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.err {
            SerdeJson(ref err) => fmt::Display::fmt(err, f)?,
            Hyper(ref err) => fmt::Display::fmt(err, f)?,
            Io(ref err) => fmt::Display::fmt(err, f)?,
            StatusCode(ref status) => fmt::Display::fmt(status, f)?,
            Api(ref err) => fmt::Display::fmt(err, f)?,
            Unauthorized => {
                write!(f,
                       "access not authorized: token expired, username/password
                       incorrect or no login provided")?;
            }
            ErrorType::__Nonexhaustive => unreachable!(),
        }
        if let Some(ref url) = self.url {
            write!(f, " | at url '{}'", url)?;
        }
        if let Some(ref json) = self.json {
            write!(f, " | return json: '{}'", json)?;
        }
        Ok(())
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match self.err {
            SerdeJson(ref err) => err.description(),
            Hyper(ref err) => err.description(),
            Io(ref err) => err.description(),
            StatusCode(ref status) => {
                match status.canonical_reason() {
                    Some(reason) => reason,
                    None => {
                        use hyper::status::StatusClass::*;
                        match status.class() {
                            Informational => "status code error: informational",
                            Success => "status code error: success",
                            Redirection => "status code error: redirection",
                            ClientError => "status code error: client error",
                            ServerError => "status code error: server error",
                            NoClass => "status code error: strange status",
                        }
                    }
                }
            }
            Api(ref err) => err.description(),
            Unauthorized => "access not authorized: token expired, username/password incorrect or no login provided",
            __Nonexhaustive => unreachable!(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match self.err {
            SerdeJson(ref err) => Some(err),
            Hyper(ref err) => Some(err),
            Io(ref err) => Some(err),
            Api(ref err) => Some(err),
            StatusCode(_) | Unauthorized => None,
            __Nonexhaustive => unreachable!(),
        }
    }
}

/// Error representing some abnormal response from the API.
#[derive(Debug, Clone)]
pub enum ApiError {
    /// The server responded with an "ok" code which was not `1`.
    NotOk(i32),
    /// A known response to a query about an invalid room.
    InvalidRoom,
    /// The data being requested was not found.
    ResultNotFound,
    /// The user whose data was being requested was not found.
    UserNotFound,
    /// The API returned that invalid parameters were passed.
    InvalidParameters,
    /// An error found from the API. Data is the raw error string reported by the server.
    GenericError(String),
    /// The server response was missing a top-level JSON field that was expected.
    MissingField(&'static str),
    /// A malformed response, including a formatted String description of the error.
    MalformedResponse(String),
    /// A marker variant that tells the compiler that users of this enum cannot match it exhaustively.
    #[doc(hidden)]
    __Nonexhaustive,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ApiError::NotOk(code) => write!(f, "non-ok result from api result: {}", code),
            ApiError::MissingField(field) => write!(f, "missing field from api result: {}", field),
            ApiError::MalformedResponse(ref desc) => write!(f, "malformed field from api result: {}", desc),
            ApiError::GenericError(ref err) => write!(f, "api call resulted in error: {}", err),
            ApiError::InvalidRoom |
            ApiError::ResultNotFound |
            ApiError::UserNotFound |
            ApiError::InvalidParameters => write!(f, "{}", self.description()),
            ApiError::__Nonexhaustive => unreachable!(),
        }
    }
}

impl StdError for ApiError {
    fn description(&self) -> &str {
        match *self {
            ApiError::NotOk(_) => "non-ok result from api call",
            ApiError::MissingField(_) => "missing field in api result",
            ApiError::MalformedResponse(_) => "malformed field in api result",
            ApiError::GenericError(_) => "api call resulted in error",
            ApiError::InvalidRoom => "malformed api call: invalid room",
            ApiError::ResultNotFound => "specific data requested was not found",
            ApiError::UserNotFound => "the user requested was not found",
            ApiError::InvalidParameters => "one or more parameters to the function were invalid",
            ApiError::__Nonexhaustive => unreachable!(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        None
    }
}
