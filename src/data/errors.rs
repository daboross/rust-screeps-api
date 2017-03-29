//! JSON deserializable API error structures.
use error;

/// JSON API error result from the server.
#[derive(Deserialize, Debug, Clone)]
pub struct ApiError {
    /// The error string.
    pub error: String,
}

impl Into<error::Error> for ApiError {
    fn into(self) -> error::Error {
        if self.error == "invalid room" {
            error::ApiError::InvalidRoom.into()
        } else if self.error == "result not found" {
            error::ApiError::ResultNotFound.into()
        } else if self.error == "invalid params" {
            error::ApiError::InvalidParameters.into()
        } else if self.error == "user not found" {
            error::ApiError::UserNotFound.into()
        } else {
            error::ApiError::GenericError(self.error).into()
        }
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
