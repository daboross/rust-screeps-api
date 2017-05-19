//! JSON deserializable API error structures.
use error;

/// JSON API error result from the server.
#[derive(Deserialize, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ApiError {
    /// The error string.
    pub error: String,
}

impl Into<error::Error> for ApiError {
    fn into(self) -> error::Error {
        let api_error = match &*self.error {
            "invalid room" => error::ApiError::InvalidRoom,
            "result not found" => error::ApiError::ResultNotFound,
            "invalid params" => error::ApiError::InvalidParameters,
            "user not found" => error::ApiError::UserNotFound,
            "server down" => error::ApiError::ServerDown,
            _ => error::ApiError::GenericError(self.error),
        };

        api_error.into()
    }
}

#[cfg(test)]
mod tests {
    use super::ApiError;
    use error;
    use serde_json;

    #[test]
    fn parse_sample_standard_server_error() {
        let _: ApiError = serde_json::from_value(json!({
            "error": "any error string",
        }))
            .unwrap();
    }

    #[test]
    fn parse_sample_invalid_room_error() {
        let result: ApiError = serde_json::from_value(json!({
            "error": "invalid room",
        }))
            .unwrap();

        let error: error::Error = result.into();

        match error.err {
            error::ErrorType::Api(error::ApiError::InvalidRoom) => (),
            _ => panic!("expected invalid room error, found {}", error),
        }
    }
}
