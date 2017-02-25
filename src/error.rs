extern crate hyper;
extern crate serde_json;

use std::fmt;
use std::io;
use std::error::Error as StdError;
use self::Error::*;

#[derive(Debug)]
pub enum Error {
    SerdeJson(serde_json::error::Error),
    Hyper(hyper::error::Error),
    Io(io::Error),
    StatusCode(hyper::status::StatusCode),
    Api(ApiError),
}

pub type Result<T> = ::std::result::Result<T, Error>;


impl From<serde_json::error::Error> for Error {
    fn from(err: serde_json::error::Error) -> Error { Error::SerdeJson(err) }
}

impl From<hyper::error::Error> for Error {
    fn from(err: hyper::error::Error) -> Error { Error::Hyper(err) }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error { Error::Io(err) }
}

impl From<hyper::status::StatusCode> for Error {
    fn from(code: hyper::status::StatusCode) -> Error { Error::StatusCode(code) }
}

impl From<ApiError> for Error {
    fn from(err: ApiError) -> Error { Error::Api(err) }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SerdeJson(ref err) => fmt::Display::fmt(err, f),
            Hyper(ref err) => fmt::Display::fmt(err, f),
            Io(ref err) => fmt::Display::fmt(err, f),
            StatusCode(ref status) => fmt::Display::fmt(status, f),
            Api(ref err) => fmt::Display::fmt(err, f),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            SerdeJson(ref err) => err.description(),
            Hyper(ref err) => err.description(),
            Io(ref err) => err.description(),
            StatusCode(ref status) => {
                if let Some(reason) = status.canonical_reason() {
                    return reason;
                }
                if status.is_success() {
                    return "status code error: success";
                }
                if status.is_informational() {
                    return "status code error: informational";
                }
                if status.is_redirection() {
                    return "status code error: redirection";
                }
                if status.is_client_error() {
                    return "status code error: client error";
                }
                if status.is_server_error() {
                    return "status code error: server error";
                }
                if status.is_strange_status() {
                    return "status code error: strange status";
                }
                return "status code error";
            },
            Api(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            SerdeJson(ref err) => Some(err),
            Hyper(ref err) => Some(err),
            Io(ref err) => Some(err),
            StatusCode(_) => None,
            Api(ref err) => Some(err),
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
