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
    /// Phantom data in order to allow adding any additional fields in the future.
    #[doc(hidden)]
    pub _phantom: marker::PhantomData<()>,
}

impl Error {
    /// Creates a new error from the given possible error type, and the given url.
    pub fn new<T: Into<Error>>(err: T, url: Option<hyper::Url>) -> Error {
        let err = err.into();
        Error {
            err: err.err,
            url: url.or(err.url),
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
            _phantom: marker::PhantomData,
        }
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(err: serde_json::error::Error) -> Error {
        Error {
            err: ErrorType::SerdeJson(err),
            url: None,
            _phantom: marker::PhantomData,
        }
    }
}

impl From<hyper::error::Error> for Error {
    fn from(err: hyper::error::Error) -> Error {
        Error {
            err: ErrorType::Hyper(err),
            url: None,
            _phantom: marker::PhantomData,
        }
    }
}

impl From<hyper::error::ParseError> for Error {
    fn from(err: hyper::error::ParseError) -> Error {
        Error {
            err: ErrorType::Hyper(hyper::Error::Uri(err)),
            url: None,
            _phantom: marker::PhantomData,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error {
            err: ErrorType::Io(err),
            url: None,
            _phantom: marker::PhantomData,
        }
    }
}

impl From<hyper::status::StatusCode> for Error {
    fn from(code: hyper::status::StatusCode) -> Error {
        Error {
            err: {
                if code == hyper::status::StatusCode::Unauthorized {
                    ErrorType::Unauthorized
                } else {
                    ErrorType::StatusCode(code)
                }
            },
            url: None,
            _phantom: marker::PhantomData,
        }
    }
}

impl From<ApiError> for Error {
    fn from(err: ApiError) -> Error {
        Error {
            err: ErrorType::Api(err),
            url: None,
            _phantom: marker::PhantomData,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.err {
            SerdeJson(ref err) => fmt::Display::fmt(err, f),
            Hyper(ref err) => fmt::Display::fmt(err, f),
            Io(ref err) => fmt::Display::fmt(err, f),
            StatusCode(ref status) => fmt::Display::fmt(status, f),
            Api(ref err) => fmt::Display::fmt(err, f),
            Unauthorized => {
                write!(f,
                       "access not authorized: token expired, username/password incorrect or no login provided")
            }
            ErrorType::__Nonexhaustive => unreachable!(),
        }
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
            StatusCode(_) => None,
            Api(ref err) => Some(err),
            Unauthorized => None,
            __Nonexhaustive => unreachable!(),
        }
    }
}

/// Error representing some abnormal response from the API.
#[derive(Debug, Clone)]
pub enum ApiError {
    /// The server responded with an "ok" code which was not `1`.
    NotOk(i32),
    /// The server response was missing a top-level JSON field that was expected.
    MissingField(&'static str),
    /// A malformed response in inner data
    MalformedResponse(String),
    /// A marker variant that tells the compiler that users of this enum cannot match it exhaustively.
    #[doc(hidden)]
    __Nonexhaustive,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ApiError::NotOk(code) => write!(f, "non-ok result from api call: {}", code),
            ApiError::MissingField(field) => write!(f, "missing field from api call: {}", field),
            ApiError::MalformedResponse(ref desc) => write!(f, "malformed field from api call: {}", desc),
            ApiError::__Nonexhaustive => unreachable!(),
        }
    }
}

impl StdError for ApiError {
    fn description(&self) -> &str {
        match *self {
            ApiError::NotOk(_) => "non-ok result from api call",
            ApiError::MissingField(_) => "missing field from api call",
            ApiError::MalformedResponse(_) => "malformed field from api call",
            ApiError::__Nonexhaustive => unreachable!(),
        }
    }

    fn cause(&self) -> Option<&StdError> { None }
}
