/// Serialization / deserialization of `time::Timespec`.
pub mod timespec_seconds {
    use serde::{Deserializer, Serializer};
    use time::Timespec;

    /// Serializes a Timespec by just serializing the seconds as a number.
    pub fn serialize<S>(date: &Timespec, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(date.sec)
    }

    /// Deserializes either a number or a string into a `Timespec`, interpreting both as the timespec's seconds.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Timespec, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::{Error, Unexpected, Visitor};
        use std::fmt;

        struct TimeVisitor;

        impl<'de> Visitor<'de> for TimeVisitor {
            type Value = Timespec;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an integer or string containing an integer")
            }

            #[inline]
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                let seconds = value
                    .parse()
                    .map_err(|_| E::invalid_value(Unexpected::Str(value), &self))?;

                Ok(Timespec::new(seconds, 0))
            }
            #[inline]
            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Timespec::new(value, 0))
            }

            #[inline]
            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Timespec::new(value as i64, 0))
            }
        }

        deserializer.deserialize_any(TimeVisitor)
    }
}

/// Serialization / deserialization of `Option<time::Timespec>`.
pub mod optional_timespec_seconds {
    use serde::{Deserializer, Serializer};
    use time::Timespec;

    /// Serializes an `Option<Timespec>` as the timespec's seconds as a number.
    ///
    /// A unit / nothing will be serialized if the Option is None.
    #[allow(dead_code)]
    pub fn serialize<S>(date: &Option<Timespec>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *date {
            Some(ref d) => serializer.serialize_i64(d.sec),
            None => serializer.serialize_unit(),
        }
    }

    /// Deserializes either a string or a number into a `time::Timespec`.
    ///
    /// Strings must be parsable as numbers.
    ///
    /// Nothing / a unit will be parsed as None.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Timespec>, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::{Error, Unexpected, Visitor};
        use std::fmt;

        struct TimeVisitor;

        impl<'de> Visitor<'de> for TimeVisitor {
            type Value = Option<Timespec>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an integer or string containing an integer")
            }

            #[inline]
            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(None)
            }

            #[inline]
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                let seconds = value
                    .parse()
                    .map_err(|_| E::invalid_value(Unexpected::Str(value), &self))?;

                Ok(Some(Timespec::new(seconds, 0)))
            }

            #[inline]
            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Some(Timespec::new(value, 0)))
            }

            #[inline]
            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Some(Timespec::new(value as i64, 0)))
            }
        }

        deserializer.deserialize_any(TimeVisitor)
    }
}

/// Serialization / deserialization of `Option<time::Timespec>`.
///
/// A non-existent value will be None, but a JSON null will always deserialize into `Some(None)`.
///
/// Useful for updating structs.
pub mod double_optional_timespec_seconds {
    use serde::{Deserializer, Serializer};
    use time::Timespec;

    /// Serializes an `Option<Option<Timespec>>` as the timespec's seconds as a number.
    ///
    /// A unit / nothing will be serialized if the Option is None.
    #[allow(dead_code)]
    pub fn serialize<S>(date: &Option<Option<Timespec>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *date {
            Some(Some(ref d)) => serializer.serialize_i64(d.sec),
            _ => serializer.serialize_unit(),
        }
    }

    /// Deserializes either a string or a number into a time::Timespec.
    ///
    /// Strings must be parsable as numbers.
    ///
    /// Nothing / a unit will be parsed as None.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Option<Timespec>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::{Error, Unexpected, Visitor};
        use std::fmt;

        struct TimeVisitor;

        impl<'de> Visitor<'de> for TimeVisitor {
            type Value = Option<Option<Timespec>>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an integer or string containing an integer")
            }

            #[inline]
            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Some(None))
            }

            #[inline]
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                let seconds = value
                    .parse()
                    .map_err(|_| E::invalid_value(Unexpected::Str(value), &self))?;

                Ok(Some(Some(Timespec::new(seconds, 0))))
            }

            #[inline]
            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Some(Some(Timespec::new(value, 0))))
            }

            #[inline]
            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Some(Some(Timespec::new(value as i64, 0))))
            }
        }

        deserializer.deserialize_any(TimeVisitor)
    }
}

#[cfg(test)]
mod tests {
    use time;

    use super::{optional_timespec_seconds, timespec_seconds};

    #[test]
    fn parse_string_timespec() {
        let spec = timespec_seconds::deserialize(&json!("1474674699273")).unwrap();

        assert_eq!(spec, time::Timespec::new(1474674699273i64, 0));
    }

    #[test]
    fn parse_number_timespec() {
        let spec = timespec_seconds::deserialize(&json!(1475538699273i64)).unwrap();

        assert_eq!(spec, time::Timespec::new(1475538699273i64, 0));
    }

    #[test]
    fn parse_string_optional_timepsec() {
        let spec = optional_timespec_seconds::deserialize(&json!("1474674699273")).unwrap();

        assert_eq!(spec, Some(time::Timespec::new(1474674699273i64, 0)));
    }

    #[test]
    fn parse_number_optional_timepsec() {
        let spec = optional_timespec_seconds::deserialize(&json!(1474674699273i64)).unwrap();

        assert_eq!(spec, Some(time::Timespec::new(1474674699273i64, 0)));
    }

    #[test]
    fn parse_null_optional_timepsec() {
        let spec = optional_timespec_seconds::deserialize(&json!(null)).unwrap();

        assert_eq!(spec, None);
    }
}
