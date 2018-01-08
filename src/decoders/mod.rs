macro_rules! str_mod {
    (
        $mod_name:ident,
        $optional_mod_name:ident,
        $ty:ident
    ) => {
        macro_rules! visit_method {
            (
                $method_type:ident,
                $method_name:ident,
                $from_method:ident,
                $surround:expr,
                $group:ident,
                $group_ty:ident
            ) => {
                #[inline]
                fn $method_name <E>(self, value: $method_type) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    match ::num::cast::FromPrimitive::$from_method(value) {
                        Some(v) => Ok($surround(v)),
                        None => Err(Error::invalid_value(Unexpected::$group(value as $group_ty), &self))
                    }
                }
            }
        }


        #[allow(dead_code)]
        pub mod $mod_name {
            use serde::{Deserializer, Serializer, Serialize};
            use std::fmt;
            use serde::de::{Error, Unexpected, Visitor};

            struct StringOrNumberVisitor;

            impl<'de> Visitor<'de> for StringOrNumberVisitor {
                type Value = $ty;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str(concat!(stringify!($ty), " or string containing ", stringify!($ty)))
                }

                #[inline]
                fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    value
                        .parse()
                        .map_err(|_| E::invalid_value(Unexpected::Str(value), &self))
                }


                // Idea roughly taken from: https://github.com/serde-rs/serde/
                // blob/4751627f1cd14cacdf216188ccbb9ab0831e2b3f/serde/src/de/impls.rs#L136
                visit_method!(i8, visit_i8, from_i8, |x| x, Signed, i64);
                visit_method!(i16, visit_i16, from_i16, |x| x,  Signed, i64);
                visit_method!(i32, visit_i32, from_i32, |x| x, Signed, i64);
                visit_method!(i64, visit_i64, from_i64, |x| x, Signed, i64);

                visit_method!(u8, visit_u8, from_u8, |x| x, Unsigned, u64);
                visit_method!(u16, visit_u16, from_u16, |x| x, Unsigned, u64);
                visit_method!(u32, visit_u32, from_u32, |x| x, Unsigned, u64);
                visit_method!(u64, visit_u64, from_u64, |x| x, Unsigned, u64);
            }

            /// Serializes an integer directly.
            pub fn serialize<S>(data: &i64, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                data.serialize(serializer)
            }

            /// Deserializes either a number or a string into an integer.
            pub fn deserialize<'de, D>(deserializer: D) -> Result<$ty, D::Error>
            where
                D: Deserializer<'de>,
            {
                deserializer.deserialize_any(StringOrNumberVisitor)
            }
        }

        #[allow(dead_code)]
        pub mod $optional_mod_name {
            use serde::{Deserializer, Serializer, Serialize};
            use std::fmt;
            use serde::de::{Error, Unexpected, Visitor};

            struct OptionalStringOrNumberVisitor;

            impl<'de> Visitor<'de> for OptionalStringOrNumberVisitor {
                type Value = Option<$ty>;

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
                    value
                        .parse()
                        .map(Some)
                        .map_err(|_| E::invalid_value(Unexpected::Str(value), &self))
                }

                // Idea roughly taken from: https://github.com/serde-rs/serde/
                // blob/4751627f1cd14cacdf216188ccbb9ab0831e2b3f/serde/src/de/impls.rs#L136
                visit_method!(i8, visit_i8, from_i8, Some,  Signed, i64);
                visit_method!(i16, visit_i16, from_i16, Some, Signed, i64);
                visit_method!(i32, visit_i32, from_i32, Some, Signed, i64);
                visit_method!(i64, visit_i64, from_i64, Some, Signed, i64);

                visit_method!(u8, visit_u8, from_u8, Some, Unsigned, u64);
                visit_method!(u16, visit_u16, from_u16, Some, Unsigned, u64);
                visit_method!(u32, visit_u32, from_u32, Some, Unsigned, u64);
                visit_method!(u64, visit_u64, from_u64, Some, Unsigned, u64);
            }

            /// Serializes an integer directly.
            pub fn serialize<S>(data: &Option<$ty>, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                match *data {
                    Some(ref d) => d.serialize(serializer),
                    None => serializer.serialize_unit(),
                }
            }

            /// Deserializes either a number or a string into an integer.
            ///
            /// Nothing / a unit will be parsed as None.
            pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<$ty>, D::Error>
            where
                D: Deserializer<'de>,
            {
                deserializer.deserialize_any(OptionalStringOrNumberVisitor)
            }
        }
    }
}

// str_mod!(i64_or_str_containing, optional_i64_or_str_containing, i64);
// str_mod!(u64_or_str_containing, optional_u64_or_str_containing, u64);
// str_mod!(i32_or_str_containing, optional_i32_or_str_containing, i32);
str_mod!(u32_or_str_containing, optional_u32_or_str_containing, u32);
// str_mod!(i16_or_str_containing, optional_i16_or_str_containing, i16);
str_mod!(u16_or_str_containing, optional_u16_or_str_containing, u16);
