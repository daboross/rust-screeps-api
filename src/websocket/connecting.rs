//! Websocket url utilities.
use std::str;

use rand::{self, Rng, seq::SliceRandom};
use url::Url;

mod error {
    use std::{error, fmt};
    use url;

    /// Error turning a screeps API url into a websocket url.
    #[derive(Clone, Debug)]
    pub enum Error {
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

    impl Error {
        /// Creates an error given the `ParseError` and url `String`.
        pub fn from_err(error: url::ParseError, url: String) -> Self {
            Error::Parse {
                err: error,
                url: url,
            }
        }

        /// Creates an error given the scheme `String` and the url `Url`.
        pub fn wrong_scheme(scheme: String, url: url::Url) -> Self {
            Error::WrongScheme {
                url: url,
                scheme: scheme,
            }
        }
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                Error::Parse { ref err, ref url } => {
                    write!(f, "URL parse error: {} | url: {}", err, url)
                }
                Error::WrongScheme {
                    ref scheme,
                    ref url,
                } => write!(
                    f,
                    "expected HTTP or HTTPS url, found {} | url: {}",
                    scheme, url
                ),
            }
        }
    }

    impl error::Error for Error {
        fn description(&self) -> &str {
            match *self {
                Error::Parse { .. } => "expected URL to succeed parsing, but it failed",
                Error::WrongScheme { .. } => "expected HTTP or HTTPS url, found a different scheme",
            }
        }

        fn cause(&self) -> Option<&error::Error> {
            match *self {
                Error::Parse { ref err, .. } => Some(err),
                Error::WrongScheme { .. } => None,
            }
        }
    }
}

pub use self::error::Error as UrlError;

/// Creates a new (random) websocket URL to connect to the official server.
pub fn default_url() -> Url {
    transform_url(::DEFAULT_OFFICIAL_API_URL)
        .expect("expected known good default URL to parse successfully.")
}

/// Method for finding a websocket URL given the screeps API URL.
///
/// This method uses the thread-local `rand` crate rng to come up with a unique
/// session id, and the resulting url should not be reused over multiple connections.
///
/// The input URL should be an API url in the format of `https://screeps.com/api/`.
pub fn transform_url<U: AsRef<str> + ?Sized>(url: &U) -> Result<Url, UrlError> {
    use std::fmt;

    let mut url = match url.as_ref().parse::<Url>() {
        Ok(v) => v,
        Err(e) => return Err(UrlError::from_err(e, url.as_ref().to_owned())),
    };

    let new_scheme = match url.scheme() {
        "http" => Ok("ws"),
        "https" => Ok("wss"),
        other => Err(other.to_string()),
    };

    let new_scheme = match new_scheme {
        Ok(v) => v,
        Err(other) => return Err(UrlError::wrong_scheme(other, url)),
    };

    url.set_scheme(new_scheme)
        .expect("expected `ws` and `wss` to be valid url schemes.");

    // we could probably just use gen_ascii_chars for the session ID, but to be safe
    // we just use the subset that `sockjs-client` does.
    const VALID_CHARS: &'static [u8] = b"abcdefghijklmnopqrstuvwxyz012345";

    // avoiding allocations!
    struct GenServerAndSessionId;

    impl fmt::Display for GenServerAndSessionId {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let mut rng = rand::thread_rng();
            write!(f, "../socket/{:04}/", rng.gen_range(0, 1000))?;

            for _ in 0..8 {
                write!(f, "{}", *VALID_CHARS.choose(&mut rng).unwrap() as char)?;
            }
            write!(f, "/websocket")?;

            Ok(())
        }
    }

    let result = url
        .join(&GenServerAndSessionId.to_string())
        .expect("expected generated string known to be correct to parse successfully");

    Ok(result)
}
