//! Update parsing for console message updates.
use serde::de::{Deserialize, Deserializer, Error};

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
        /// The shard the update is from
        shard: Option<String>,
    },
    /// An error occurred in the user script. May be sent multiple times per tick,
    /// and will not be sent unless an error did occur.
    Error {
        /// The error which occurred.
        message: String,
        /// The shard the update is from
        shard: Option<String>,
    },
}

impl UserConsoleUpdate {
    /// Gets the shard this update is for.
    pub fn shard(&self) -> Option<&str> {
        match *self {
            UserConsoleUpdate::Messages { ref shard, .. } => shard.as_ref().map(|s| s.as_ref()),
            UserConsoleUpdate::Error { ref shard, .. } => shard.as_ref().map(|s| s.as_ref()),
        }
    }
}

// Separate representation for deserializing needed in order to have 'Error' variant be a
// struct-like variant rather than a tuple-like variant.

#[derive(Deserialize, Debug)]
struct InnerUpdateInnerMessages {
    log: Vec<String>,
    results: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct InnerUpdateRepresentation {
    error: Option<String>,
    messages: Option<InnerUpdateInnerMessages>,
    shard: Option<String>,
}

impl<'de> Deserialize<'de> for UserConsoleUpdate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let intermediate = InnerUpdateRepresentation::deserialize(deserializer)?;

        let InnerUpdateRepresentation {
            error: error_opt,
            messages: messages_opt,
            shard,
        } = intermediate;

        let parsed = match (error_opt, messages_opt) {
            (Some(e), None) => UserConsoleUpdate::Error {
                message: e,
                shard: shard,
            },
            (None, Some(m)) => UserConsoleUpdate::Messages {
                log_messages: m.log,
                result_messages: m.results,
                shard: shard,
            },
            (Some(_), Some(_)) => {
                return Err(D::Error::custom("expected either 'messages' property or 'error' property, not both"));
            }
            (None, None) => {
                return Err(D::Error::custom("expected either 'messages' property or 'error' property, neither found"));
            }
        };

        Ok(parsed)
    }
}
