//! Various data structs that different calls might use in results.

use error;

/// Badge type!
#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum BadgeType {
    Fixed(i32),
    Dynamic { path1: String, path2: String },
}

/// Badge color!
#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum BadgeColor {
    Set(i32),
    Hex(String),
}

/// badge: { type, color1, color2, color3, param, flip }
#[derive(Deserialize, Debug, Clone)]
pub struct Badge {
    /// Badge type, used for different badge formats
    #[serde(rename="type")]
    pub badge_type: BadgeType,
    /// First color, use depends on badge type.
    pub color1: BadgeColor,
    /// Second color, use depends on badge type.
    pub color2: BadgeColor,
    /// Third color, use depends on badge type.
    pub color3: BadgeColor,
    /// Integer parameter to badge display, changes the shape of the badge in a different way depending on badge type.
    pub param: i32,
    /// Flips the badge either horizontally or vertically, depending on badge type.
    pub flip: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ApiError {
    pub error: String,
}

impl Into<error::Error> for ApiError {
    fn into(self) -> error::Error {
        if self.error == "invalid room" {
            error::ApiError::InvalidRoom.into()
        } else {
            error::ApiError::GenericError(self.error).into()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Badge, ApiError};
    use error;
    use serde_json;

    #[test]
    fn parse_sample_parse_badge_simple_colors_simple_pattern() {
        let _: Badge = serde_json::from_value(json!({
            "type": 19,
            "color1": 37,
            "color2": 57,
            "color3": 1,
            "param": 0,
            "flip": false,
        }))
            .unwrap();
    }

    #[test]
    fn parse_sample_badge_hex_colors_simple_pattern() {
        let _: Badge = serde_json::from_value(json!({
            "type": 21,
            "color1": "#260d0d",
            "color2": "#6b2e41",
            "color3": "#ffe56d",
            "param": -100,
            "flip": false,
        }))
            .unwrap();
    }

    #[test]
    fn parse_sample_badge_custom_dissi() {
        let _: Badge = serde_json::from_value(json!({
            "type": {
                "path1": "M0,50c0,27.3,21.8,49.5,48.9,50v-9.6c-2.5-0.8-5.5-4.6-7.8-10.2c-0.9-2.3-1.7-4.7-2.4-7.4   \
                c3.3,0.3,6.7,0.5,10.2,0.5v-9.5c-4.2,0-8.2-0.3-12-0.8c-0.5-4.1-0.8-8.5-0.8-13c0-4.5,0.3-8.9,0.8-13c3.8-\
                0.5,7.9-0.8,12-0.8v-9.5   c-3.5,0-6.9,0.2-10.2,0.5c0.7-2.7,1.5-5.1,2.4-7.4c2.3-5.6,5.2-9.4,7.8-10.2V0C\
                21.8,0.5,0,22.7,0,50L0,50z M9.5,50   c0-2.7,4-6.2,10.2-8.7c2.3-0.9,4.7-1.7,7.4-2.4c-0.4,3.6-0.5,7.3-0.\
                5,11.1c0,3.8,0.2,7.5,0.5,11.1c-2.6-0.7-5.1-1.5-7.4-2.4   C13.5,56.2,9.5,52.7,9.5,50L9.5,50z \
                M12.8,66c1.1,0.5,2.2,1.1,3.4,1.5c3.7,1.5,7.9,2.8,12.5,3.7c0.9,4.6,2.2,8.8,3.7,12.5   \
                c0.5,1.2,1,2.3,1.5,3.4C24.4,83.1,16.8,75.5,12.8,66L12.8,66z \
                M33.9,12.8c-0.5,1.1-1,2.2-1.5,3.4c-1.5,3.7-2.8,8-3.7,12.5   \
                c-4.5,0.9-8.8,2.2-12.5,3.7c-1.2,0.5-2.3,1-3.4,1.5C16.8,24.5,24.4,16.9,33.9,12.8L33.9,12.8z",
                "path2": "M97,59l-6-3.4c0.3-1.8,0.4-3.7,0.4-5.6c0-1.9-0.1-3.7-0.4-5.6l6-3.4c2.8-1.6,3.8-5.2,2.2-8.1   \
                l-8.6-15.2c-0.8-1.4-2.1-2.4-3.6-2.8c-1.5-0.4-3.1-0.2-4.5,0.6l-6.1,3.5c-3.1-2.5-6.6-4.6-10.3-6v-7c0-3.3\
                -2.6-5.9-5.9-5.9h-7.7   v23.4C66.8,23.9,78.1,35.7,78.1,50c0,14.3-11.4,26.1-25.5,26.6V100h7.7c3.3,0,5.9\
                -2.7,5.9-5.9v-7c3.7-1.5,7.2-3.5,10.3-6l6.1,3.5   c0.9,0.5,1.9,0.8,2.9,0.8c0.5,0,1.1-0.1,1.6-0.2c1.5-0.\
                4,2.8-1.4,3.6-2.8l8.6-15.2C100.8,64.2,99.8,60.6,97,59L97,59z",
            },
            "color1": "#000000",
            "color2": "#028300",
            "color3": "#8b5c00",
            "param": 0,
            "flip": false,
        }))
            .unwrap();
    }

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
