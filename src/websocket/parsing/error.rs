//! Websocket message parsing errors.
use std::fmt;
use serde_json;

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
    /// A marker variant that tells the compiler that users of this enum cannot match it exhaustively.
    #[doc(hidden)]
    __Nonexhaustive,
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
            ParseError::__Nonexhaustive => unreachable!(),
        }
    }
}

impl ::std::error::Error for ParseError {
    fn description(&self) -> &str {
        match *self {
            ParseError::Other(_) => "a parsing error occurred",
            ParseError::Serde { ref error_desc, .. } => error_desc,
            ParseError::__Nonexhaustive => unreachable!(),
        }
    }

    fn cause(&self) -> Option<&::std::error::Error> {
        match *self {
            ParseError::Serde { ref err, .. } => Some(err),
            ParseError::Other(_) => None,
            ParseError::__Nonexhaustive => unreachable!(),
        }
    }
}
