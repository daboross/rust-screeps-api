use serde::{Deserialize, Deserializer};

/// Deserializes either a number or a string into an integer.
pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    Option<T>: Deserialize<'de>,
    T: Default,
    D: Deserializer<'de>,
{
    let res: Option<T> = Deserialize::deserialize(deserializer)?;
    Ok(res.unwrap_or_default())
}
