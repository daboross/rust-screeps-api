extern crate hyper;
extern crate serde_json;

use self::ErrorType::*;
use std::error::Error as StdError;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum ErrorType {
    Unauthorized,
    SerdeJson(serde_json::error::Error),
    Hyper(hyper::error::Error),
    Io(io::Error),
    StatusCode(hyper::status::StatusCode),
    Api(ApiError),
}

#[derive(Debug)]
pub struct Error {
    pub err: ErrorType,
    pub url: Option<hyper::Url>,
}

impl Error {
    pub fn new<T: Into<Error>>(err: T, url: Option<hyper::Url>) -> Error {
        Error {
            err: err.into().err,
            url: url,
        }
    }
}

pub type Result<T> = ::std::result::Result<T, Error>;

impl From<ErrorType> for Error {
    fn from(err: ErrorType) -> Error {
        Error {
            err: err,
            url: None,
        }
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(err: serde_json::error::Error) -> Error {
        Error {
            err: ErrorType::SerdeJson(err),
            url: None,
        }
    }
}

impl From<hyper::error::Error> for Error {
    fn from(err: hyper::error::Error) -> Error {
        Error {
            err: ErrorType::Hyper(err),
            url: None,
        }
    }
}

impl From<hyper::error::ParseError> for Error {
    fn from(err: hyper::error::ParseError) -> Error {
        Error {
            err: ErrorType::Hyper(hyper::Error::Uri(err)),
            url: None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error {
            err: ErrorType::Io(err),
            url: None,
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
        }
    }
}

impl From<ApiError> for Error {
    fn from(err: ApiError) -> Error {
        Error {
            err: ErrorType::Api(err),
            url: None,
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
                write!(f, "access not authorized: token expired, username/password incorrect or no login provided")
            }
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
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ApiError {
    NotOk(i32),
    MissingField(&'static str),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ApiError::NotOk(code) => write!(f, "non-ok result from api call: {}", code),
            ApiError::MissingField(field) => write!(f, "missing field from api call: {}", field),
        }
    }
}

impl StdError for ApiError {
    fn description(&self) -> &str {
        match *self {
            ApiError::NotOk(_) => "non-ok result from api call",
            ApiError::MissingField(_) => "missing field from api call",
        }
    }

    fn cause(&self) -> Option<&StdError> { None }
}
