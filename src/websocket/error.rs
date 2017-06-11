use std::fmt;
use std::error::Error;

use url;

/// Error turning a screeps API url into a websocket url.
#[derive(Clone, Debug)]
pub enum UrlError {
    /// Error parsing the URL.
    Parse {
        /// The parse error.
        err: url::ParseError,
        /// URL that failed to parse.
        url: String,
    },
    /// Found an unexpected scheme, not `http` or `https`.
    WrongScheme {
        /// The scheme that wasn't `http` or `https`.
        scheme: String,
        /// The URL that had this scheme.
        url: url::Url,
    },
}

impl UrlError {
    /// Creates an error given the `ParseError` and url `String`.
    pub fn from_err(error: url::ParseError, url: String) -> Self {
        UrlError::Parse {
            err: error,
            url: url,
        }
    }

    /// Creates an error given the scheme `String` and the url `Url`.
    pub fn wrong_scheme(scheme: String, url: url::Url) -> Self {
        UrlError::WrongScheme {
            url: url,
            scheme: scheme,
        }
    }
}

impl fmt::Display for UrlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UrlError::Parse { ref err, ref url } => write!(f, "URL parse error: {} | url: {}", err, url),
            UrlError::WrongScheme { ref scheme, ref url } => {
                write!(f,
                       "expected HTTP or HTTPS url, found {} | url: {}",
                       scheme,
                       url)
            }
        }
    }
}


impl Error for UrlError {
    fn description(&self) -> &str {
        match *self {
            UrlError::Parse { .. } => "expected URL to succeed parsing, but it failed",
            UrlError::WrongScheme { .. } => "expected HTTP or HTTPS url, found a different scheme",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            UrlError::Parse { ref err, .. } => Some(err),
            UrlError::WrongScheme { .. } => None,
        }
    }
}
