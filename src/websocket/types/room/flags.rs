//! Types for user flags which can appear in rooms.
use std::fmt;

use serde::de::{Deserializer, Error, Unexpected, Visitor};

/// Single flag.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Flag {
    /// The name of the flag, unique per user.
    pub name: String,
    /// The primary color of the flag.
    pub primary_color: FlagColor,
    /// The secondary color of the flag.
    pub secondary_color: FlagColor,
    /// The X position of the flag.
    pub x: u32,
    /// The Y position of the flag.
    pub y: u32,
}

/// All possible colors a flag can have.
#[repr(u8)]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum FlagColor {
    /// COLOR_RED = 1
    Red = 1,
    /// COLOR_PURPLE = 2
    Purple = 2,
    /// COLOR_BLUE = 3
    Blue = 3,
    /// COLOR_CYAN = 4
    Cyan = 4,
    /// COLOR_GREEN = 5
    Green = 5,
    /// COLOR_YELLOW = 6
    Yellow = 6,
    /// COLOR_ORANGE = 7
    Orange = 7,
    /// COLOR_BROWN = 8
    Brown = 8,
    /// COLOR_GREY = 9
    Grey = 9,
    /// COLOR_WHITE = 10
    White = 10,
}

/// An error resulting from trying to convert a [`FlagColor`] that's out of bounds.
///
/// [`FlagColor`]: enum.FlagColor.html
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct FlagColorError;

impl FlagColor {
    /// Converts an integer color code into a flag color.
    #[inline]
    pub fn from(v: u8) -> Result<Self, FlagColorError> {
        match v {
            1 => Ok(FlagColor::Red),
            2 => Ok(FlagColor::Purple),
            3 => Ok(FlagColor::Blue),
            4 => Ok(FlagColor::Cyan),
            5 => Ok(FlagColor::Green),
            6 => Ok(FlagColor::Yellow),
            7 => Ok(FlagColor::Orange),
            8 => Ok(FlagColor::Brown),
            9 => Ok(FlagColor::Grey),
            10 => Ok(FlagColor::White),
            _ => Err(FlagColorError),
        }
    }

    #[inline]
    fn from_serde<E: Error>(v: u8) -> Result<Self, E> {
        match v {
            1 => Ok(FlagColor::Red),
            2 => Ok(FlagColor::Purple),
            3 => Ok(FlagColor::Blue),
            4 => Ok(FlagColor::Cyan),
            5 => Ok(FlagColor::Green),
            6 => Ok(FlagColor::Yellow),
            7 => Ok(FlagColor::Orange),
            8 => Ok(FlagColor::Brown),
            9 => Ok(FlagColor::Grey),
            10 => Ok(FlagColor::White),
            other => Err(E::invalid_value(
                Unexpected::Unsigned(other as u64),
                &"an integer between 1 and 10",
            )),
        }
    }
}

struct FlagStringVisitor;

impl<'de> Visitor<'de> for FlagStringVisitor {
    type Value = Vec<Flag>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string with flag formatting (`Flag571~2~3~14~7|`...)")
    }

    #[inline]
    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Vec::new())
    }

    #[inline]
    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Vec::new())
    }

    #[inline]
    fn visit_some<D>(self, d: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        d.deserialize_str(self)
    }

    #[inline]
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        if v.is_empty() {
            return Ok(Vec::new());
        }

        // TODO: nocopy version of this maybe?
        v.split('|')
            .map(|flag_str| {
                let mut iter = flag_str.split('~');

                macro_rules! next {
                    () => {{
                        iter.next().ok_or_else(|| {
                            E::invalid_type(
                                Unexpected::Str(flag_str),
                                &"a string in the format of name~4~2~4~2",
                            )
                        })?
                    }};
                }

                macro_rules! next_u8 {
                    () => {{
                        let next = next!();
                        next.parse().map_err(|_| {
                            E::invalid_type(Unexpected::Str(next), &"an unsigned integer")
                        })?
                    }};
                }

                macro_rules! next_color {
                    () => {{
                        FlagColor::from_serde(next_u8!())?
                    }};
                }

                Ok(Flag {
                    name: next!().to_owned(),
                    primary_color: next_color!(),
                    secondary_color: next_color!(),
                    x: next_u8!(),
                    y: next_u8!(),
                })
            })
            .collect::<Result<Vec<_>, _>>()
    }
}

pub(super) fn deserialize_flags<'de, D>(deserializer: D) -> Result<Vec<Flag>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_option(FlagStringVisitor)
}
