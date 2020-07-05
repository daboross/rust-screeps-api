//! Error types for the screeps api.
use std::{error::Error as StdError, fmt, io, str};

use crate::data::RoomNameParseError;

use self::ErrorKind::*;

#[derive(Debug)]
/// Possible error types for library errors.
pub enum ErrorKind {
    /// Unauthorized access. This is caused by either attempting to access a login-only endpoint without a token,
    /// attempting to access a login-only endpoint with an expired token, or providing incorrect login details to the
    /// login endpoint.
    Unauthorized,
    /// Error parsing a server response. This is most likely caused by the server providing unparsable JSON, but it
    /// could also be the server's API response structure has changed and no longer matches the expected data structure.
    SerdeJson(serde_json::error::Error),
    /// URL parsing error.
    Url(url::ParseError),
    /// Error connecting to the server, or error parsing a URL provided.
    Hyper(hyper::error::Error),
    /// IO error.
    Io(io::Error),
    /// Error for when the server responds with a non-success HTTP status code.
    StatusCode(hyper::StatusCode),
    /// API Error: when the server responds with a successful HTTP response, but the returned format is not what we
    /// expected.
    Api(ApiError),
    /// Error parsing a room name.
    RoomNameParse(RoomNameParseError<'static>),
    /// A marker variant that tells the compiler that users of this enum cannot match it exhaustively.
    #[doc(hidden)]
    __Nonexhaustive,
}

/// Error deriving from some API call.
#[derive(Debug)]
pub struct Error {
    /// The type specifying what kind of error, and a detailed description if available.
    err: ErrorKind,
    /// The whole URL which was being accessed when this error occurred (not included for URL parsing errors).
    url: Option<url::Url>,
    /// The json or body data from the request which resulted in this error
    /// (not included for URL parsing errors).
    data: AdditionalData,
}

#[derive(Debug)]
enum AdditionalData {
    Json(serde_json::Value),
    Body(bytes::Bytes),
    None,
}

impl From<Option<serde_json::Value>> for AdditionalData {
    fn from(value: Option<serde_json::Value>) -> Self {
        match value {
            Some(v) => AdditionalData::Json(v),
            None => AdditionalData::None,
        }
    }
}
impl From<Option<bytes::Bytes>> for AdditionalData {
    fn from(value: Option<bytes::Bytes>) -> Self {
        match value {
            Some(v) => AdditionalData::Body(v),
            None => AdditionalData::None,
        }
    }
}

impl AdditionalData {
    fn or(self, other: AdditionalData) -> Self {
        match self {
            AdditionalData::Json(v) => AdditionalData::Json(v),
            AdditionalData::Body(v) => AdditionalData::Body(v),
            AdditionalData::None => other,
        }
    }
    fn json(&self) -> Option<&serde_json::Value> {
        match *self {
            AdditionalData::Json(ref v) => Some(v),
            _ => None,
        }
    }
    fn body(&self) -> Option<&bytes::Bytes> {
        match *self {
            AdditionalData::Body(ref v) => Some(v),
            _ => None,
        }
    }
}

impl Error {
    /// Creates a new error from the given error and the given possible url.
    pub fn with_url<T: Into<Error>>(err: T, url: Option<url::Url>) -> Error {
        Error::with_json(err, url, None)
    }
    /// Creates a new error from the given error, the given possible url, and the given possible JSON data.
    pub fn with_json<T: Into<Error>>(
        err: T,
        url: Option<url::Url>,
        json: Option<serde_json::Value>,
    ) -> Error {
        let err = err.into();
        Error {
            err: err.err,
            url: url.or(err.url),
            data: AdditionalData::from(json).or(err.data),
        }
    }

    /// Creates a new error from the given error, the given possible url, and the given possible body.
    pub fn with_body<T: Into<Error>>(
        err: T,
        url: Option<url::Url>,
        body: Option<bytes::Bytes>,
    ) -> Error {
        let err = err.into();
        Error {
            err: err.err,
            url: url.or(err.url),
            data: AdditionalData::from(body).or(err.data),
        }
    }

    /// Retrieves the type specifying what kind of error, and a detailed description if available.
    pub fn kind(&self) -> &ErrorKind {
        &self.err
    }

    /// Retrieves the URL associated with this error, if any.
    pub fn url(&self) -> Option<&url::Url> {
        self.url.as_ref()
    }

    /// Retrieves the JSON data associated with this error, if any.
    pub fn json(&self) -> Option<&serde_json::Value> {
        self.data.json()
    }

    /// Retrieves the body data associated with this error, if any.
    pub fn body(&self) -> Option<&bytes::Bytes> {
        self.data.body()
    }
}

/// Result type for screeps API operations.
pub type Result<T> = ::std::result::Result<T, Error>;

impl From<ErrorKind> for Error {
    fn from(err: ErrorKind) -> Error {
        Error {
            err: err,
            url: None,
            data: AdditionalData::None,
        }
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(err: serde_json::error::Error) -> Error {
        ErrorKind::SerdeJson(err).into()
    }
}

impl From<hyper::error::Error> for Error {
    fn from(err: hyper::error::Error) -> Error {
        ErrorKind::Hyper(err).into()
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Error {
        ErrorKind::Url(err).into()
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        ErrorKind::Io(err).into()
    }
}

impl From<hyper::StatusCode> for Error {
    fn from(code: hyper::StatusCode) -> Error {
        if code == hyper::StatusCode::UNAUTHORIZED {
            ErrorKind::Unauthorized.into()
        } else {
            ErrorKind::StatusCode(code).into()
        }
    }
}

impl From<ApiError> for Error {
    fn from(err: ApiError) -> Error {
        ErrorKind::Api(err).into()
    }
}

impl<'a> From<RoomNameParseError<'a>> for Error {
    fn from(err: RoomNameParseError<'a>) -> Error {
        ErrorKind::RoomNameParse(err.into_owned()).into()
    }
}

impl From<NoToken> for Error {
    /// Creates an `Error` with `ErrorKind::Unauthorized`.
    // NoToken is a no-value struct.
    fn from(_: NoToken) -> Error {
        ErrorKind::Unauthorized.into()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.err {
            SerdeJson(ref err) => err.fmt(f)?,
            Hyper(ref err) => err.fmt(f)?,
            Url(ref err) => err.fmt(f)?,
            Io(ref err) => err.fmt(f)?,
            StatusCode(ref status) => status.fmt(f)?,
            Api(ref err) => err.fmt(f)?,
            RoomNameParse(ref err) => err.fmt(f)?,
            Unauthorized => {
                write!(
                    f,
                    "access not authorized: token expired, username/password
                       incorrect or no login provided"
                )?;
            }
            ErrorKind::__Nonexhaustive => unreachable!(),
        }
        if let Some(ref url) = self.url {
            write!(f, " | at url '{}'", url)?;
        }
        match self.data {
            AdditionalData::Json(ref json) => write!(f, " | return json: '{}'", json)?,
            AdditionalData::Body(ref body) => match str::from_utf8(body) {
                Ok(v) => write!(f, " | return body: '{}'", v)?,
                Err(_) => write!(f, " | return body: '{:?}'", &*body)?,
            },
            AdditionalData::None => (),
        }
        Ok(())
    }
}

impl StdError for Error {
    fn cause(&self) -> Option<&dyn StdError> {
        match self.err {
            SerdeJson(ref err) => Some(err),
            Hyper(ref err) => Some(err),
            Url(ref err) => Some(err),
            Io(ref err) => Some(err),
            Api(ref err) => Some(err),
            RoomNameParse(ref err) => Some(err),
            StatusCode(_) | Unauthorized => None,
            __Nonexhaustive => unreachable!(),
        }
    }
}

/// Error representing when an authenticated call is made, but there is no token currently available.
#[derive(Debug, Clone, Copy)]
pub struct NoToken;

const NO_TOKEN: &str = "token storage empty when attempting to make authenticated call.";

impl fmt::Display for NoToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        NO_TOKEN.fmt(f)
    }
}

impl StdError for NoToken {}

/// Error representing some abnormal response from the API.
#[derive(Debug, Clone)]
pub enum ApiError {
    /// The server responded with an "ok" code which was not `1`.
    NotOk(i32),
    /// The server is offline.
    ServerDown,
    /// A known response to a query about an invalid room.
    InvalidRoom,
    /// A known response to a query about an invalid shard.
    InvalidShard,
    /// The data being requested was not found.
    ResultNotFound,
    /// The user whose data was being requested was not found.
    UserNotFound,
    /// Registration is not allowed.
    RegistrationNotAllowed,
    /// The username that was attempted to register already existed.
    UsernameAlreadyExists,
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
            ApiError::MalformedResponse(ref desc) => {
                write!(f, "malformed field from api result: {}", desc)
            }
            ApiError::GenericError(ref err) => write!(f, "api call resulted in error: {}", err),
            ApiError::InvalidRoom => "malformed api call: invalid room".fmt(f),
            ApiError::InvalidShard => "malformed apic all: invalid shard".fmt(f),
            ApiError::ResultNotFound => "specific data requested was not found".fmt(f),
            ApiError::UserNotFound => "the user requested was not found".fmt(f),
            ApiError::RegistrationNotAllowed => "registering users via the API is disabled: \
                                                 a server password has been set.fmt(f)"
                .fmt(f),
            ApiError::UsernameAlreadyExists => "the username used already exists".fmt(f),
            ApiError::InvalidParameters => {
                "one or more parameters to the function were invalid".fmt(f)
            }
            ApiError::ServerDown => "the server requested is offline".fmt(f),
            ApiError::__Nonexhaustive => unreachable!(),
        }
    }
}

impl StdError for ApiError {}
