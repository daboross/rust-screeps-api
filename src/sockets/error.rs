use super::ws;
use serde_json;

use std::fmt;

use error::Error as HttpError;

/// Result type for socket-related functions. Union between Screeps HTTP error and ws-rs error.
pub type Result<T> = ::std::result::Result<T, Error>;

/// Error representing either an HTTP error or a websockets error.
#[derive(Debug)]
pub enum Error {
    /// A raw websockets error.
    Ws(ws::Error),
    /// A SockJS parse error.
    ParseError(ParseError),
    /// An HTTP error.
    Other(HttpError),
    /// A marker variant that tells the compiler that users of this enum cannot match it exhaustively.
    #[doc(hidden)]
    __Nonexhaustive,
}

impl From<ws::Error> for Error {
    fn from(err: ws::Error) -> Error {
        Error::Ws(err)
    }
}

impl From<ParseError> for Error {
    fn from(err: ParseError) -> Error {
        Error::ParseError(err)
    }
}

impl From<HttpError> for Error {
    fn from(err: HttpError) -> Error {
        Error::Other(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Ws(ref err) => write!(f, "websocket error: {}", err),
            Error::Other(ref err) => write!(f, "http error: {}", err),
            Error::ParseError(ref err) => write!(f, "parse error: {}", err),
            Error::__Nonexhaustive => unreachable!(),
        }
    }
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Ws(ref err) => err.description(),
            Error::Other(ref err) => err.description(),
            Error::ParseError(ref err) => err.description(),
            Error::__Nonexhaustive => unreachable!(),
        }
    }

    fn cause(&self) -> Option<&::std::error::Error> {
        match *self {
            Error::Ws(ref err) => Some(err),
            Error::Other(ref err) => Some(err),
            Error::ParseError(ref err) => Some(err),
            Error::__Nonexhaustive => unreachable!(),
        }
    }
}

/// A SockJS parse error occurred. TODO: more detailed info.
#[derive(Debug)]
pub enum ParseError {
    /// Some error occurred.
    Other(String),
    /// Serde json error
    Serde {
        /// Error description
        error_desc: &'static str,
        /// Full string being parsed.
        full_string: String,
        /// Inner error
        err: serde_json::Error,
    },
}

impl ParseError {
    /// Creates a serde parse error.
    pub fn serde(desc: &'static str, string: String, error: serde_json::Error) -> Self {
        ParseError::Serde {
            error_desc: desc,
            full_string: string,
            err: error,
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseError::Other(ref s) => write!(f, "parse error: {}", s),
            ParseError::Serde { ref error_desc, ref full_string, ref err } => {
                write!(f,
                       "error parsing `{}`: {}: {}",
                       full_string,
                       error_desc,
                       err)
            }
        }
    }
}

impl ::std::error::Error for ParseError {
    fn description(&self) -> &str {
        match *self {
            ParseError::Other(_) => "a parsing error occurred",
            ParseError::Serde { ref error_desc, .. } => error_desc,
        }
    }

    fn cause(&self) -> Option<&::std::error::Error> {
        match *self {
            ParseError::Serde { ref err, .. } => Some(err),
            ParseError::Other(_) => None,
        }
    }
}
