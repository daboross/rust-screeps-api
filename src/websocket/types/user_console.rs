//! Update parsing for console message updates.
use serde::de::{Deserialize, Deserializer};

/// Update for a user's log messages during the last tick.
#[derive(Clone, Hash, Debug)]
pub enum UserConsoleUpdate {
    /// All log messages which occurred during the last tick. Will always be sent
    /// once per tick unless an error occurs *and* this would be empty.
    Messages {
        /// All messages the screeps script code logged last tick.
        log_messages: Vec<String>,
        /// All result strings from console commands executed last tick.
        result_messages: Vec<String>,
    },
    /// An error occurred in the user script. May be sent multiple times per tick,
    /// and will not be sent unless an error did occur.
    Error {
        /// The error which occurred.
        message: String,
    },
}

// Separate representation for deserializing needed in order to have 'Error' variant be a
// struct-like variant rather than a tuple-like variant.

#[derive(Deserialize, Debug)]
enum InnerUpdateRepresentation {
    #[serde(rename = "messages")]
    Messages {
        #[serde(rename = "log")]
        log_messages: Vec<String>,
        #[serde(rename = "results")]
        result_messages: Vec<String>,
    },
    #[serde(rename = "error")]
    Error(String),
}

impl<'de> Deserialize<'de> for UserConsoleUpdate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let intermediate = InnerUpdateRepresentation::deserialize(deserializer)?;

        Ok(match intermediate {
            InnerUpdateRepresentation::Messages { log_messages, result_messages } => {
                UserConsoleUpdate::Messages {
                    log_messages: log_messages,
                    result_messages: result_messages,
                }
            }
            InnerUpdateRepresentation::Error(message) => UserConsoleUpdate::Error { message: message },
        })
    }
}
