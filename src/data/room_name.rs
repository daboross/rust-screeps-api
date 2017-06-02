//! Structures relating to room name parsing.

use std::{error, fmt, ops};
use std::borrow::Cow;

/// A structure representing a room name.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct RoomName {
    /// Inner x coordinate representation.
    ///
    /// 0 represents E0, positive numbers represent E(x)
    ///
    /// -1 represents W0, negative numbers represent W((-x) - 1)
    pub x_coord: i32,
    /// Inner y coordinate representation.
    ///
    /// 0 represents N0, positive numbers represent N(y)
    ///
    /// -1 represents S0, negative numbers represent S((-y) - 1)
    pub y_coord: i32,
}

impl fmt::Display for RoomName {
    /// Formats this room name into the format the game expects.
    ///
    /// Resulting string will be `(E|W)[0-9]+(N|S)[0-9]+`, and will result
    /// in an equal same RoomName if passed into [`into_room_name`].
    ///
    /// [`into_room_name`]: trait.IntoRoomName.html
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.x_coord >= 0 {
            write!(f, "E{}", self.x_coord)?;
        } else {
            write!(f, "W{}", (-self.x_coord) - 1)?;
        }

        if self.y_coord >= 0 {
            write!(f, "N{}", self.y_coord)?;
        } else {
            write!(f, "S{}", (-self.y_coord) - 1)?;
        }

        Ok(())
    }
}

impl RoomName {
    /// Creates a new room name from the given input.
    ///
    /// This will parse the input, and return an error if it is in an invalid format.
    #[inline]
    pub fn new<T>(x: &T) -> Result<Self, RoomNameParseError>
        where T: IntoRoomName + ?Sized
    {
        x.into_room_name()
    }

    /// Creates a new room name from the given position parameters.
    #[inline]
    pub fn from_pos(east: bool, north: bool, x_pos: i32, y_pos: i32) -> Self {
        RoomName {
            x_coord: if east { x_pos } else { -x_pos - 1 },
            y_coord: if north { y_pos } else { -y_pos - 1 },
        }
    }
}

impl ops::Add<(i32, i32)> for RoomName {
    type Output = RoomName;

    /// Adds an (x, y) coordinate pair to this room name.
    #[inline]
    fn add(self, (x, y): (i32, i32)) -> RoomName {
        RoomName {
            x_coord: self.x_coord + x,
            y_coord: self.y_coord + y,
        }
    }
}

/// Something that can be turned into a room name.
pub trait IntoRoomName {
    /// Turns this data into a room name, erroring if the format is not as expected.
    fn into_room_name(&self) -> Result<RoomName, RoomNameParseError>;
}

impl IntoRoomName for RoomName {
    #[inline]
    fn into_room_name(&self) -> Result<RoomName, RoomNameParseError> {
        // data is copy
        Ok(*self)
    }
}

impl<T> IntoRoomName for T
    where T: AsRef<str> + ?Sized
{
    fn into_room_name(&self) -> Result<RoomName, RoomNameParseError> {
        let s = self.as_ref();

        let mut chars = s.char_indices();

        let east = match chars.next() {
            Some((_, 'E')) | Some((_, 'e')) => true,
            Some((_, 'W')) | Some((_, 'w')) => false,
            _ => return Err(RoomNameParseError::new(s)),
        };

        let (x_coord, north) = {
            // we assume there's at least one number character. If there isn't,
            // we'll catch it when we try to parse this substr.
            let (start_index, _) = chars.next().ok_or_else(|| RoomNameParseError::new(s))?;
            let end_index;
            let north;
            loop {
                match chars.next().ok_or_else(|| RoomNameParseError::new(s))? {
                    (i, 'N') | (i, 'n') => {
                        end_index = i;
                        north = true;
                        break;
                    }
                    (i, 'S') | (i, 's') => {
                        end_index = i;
                        north = false;
                        break;
                    }
                    _ => continue,
                }
            }

            let x_coord = s[start_index..end_index].parse().map_err(|_| RoomNameParseError::new(s))?;

            (x_coord, north)
        };

        let y_coord = {
            let (start_index, _) = chars.next().ok_or_else(|| RoomNameParseError::new(s))?;

            s[start_index..s.len()].parse().map_err(|_| RoomNameParseError::new(s))?
        };

        Ok(RoomName::from_pos(east, north, x_coord, y_coord))
    }
}

/// An error representing when a string can't be parsed into a [`RoomName`].
///
/// [`RoomName`]: struct.RoomName.html
#[derive(Clone, Debug)]
pub struct RoomNameParseError<'a>(Cow<'a, str>);

impl<'a> RoomNameParseError<'a> {
    /// Private method to construct a `RoomNameParseError`.
    fn new<T: Into<Cow<'a, str>>>(failed_room_name: T) -> Self {
        RoomNameParseError(failed_room_name.into())
    }

    /// Turns this error into a 'static error, cloning any inner data that represents
    /// what failed.
    pub fn into_owned(self) -> RoomNameParseError<'static> {
        let RoomNameParseError(cow) = self;
        RoomNameParseError(cow.into_owned().into())
    }

    /// Retrieves the room name that failed to parse into a [`RoomName`].
    ///
    /// [`RoomName`]: struct.RoomName.html
    pub fn get_failed_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl<'a> error::Error for RoomNameParseError<'a> {
    fn description(&self) -> &str {
        "string failed to parse into room name"
    }
}

impl<'a> fmt::Display for RoomNameParseError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "expected room name formatted `(E|W)[0-9]+(N|S)[0-9]+`, found `{}`",
               self.0.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::RoomName;

    #[test]
    fn parse_and_format() {
        let strings = ["E0N0", "W0S0", "E20N33", "W7777N7777", "W20N33", "E50S50"];

        for string in strings.iter() {
            let parsed = RoomName::new(string).expect("failed to parse test room name");

            assert_eq!(&*parsed.to_string(), &**string);
        }
    }

    #[test]
    fn parse_and_test_result() {
        let pairs = [("E0N0", RoomName::from_pos(true, true, 0, 0)),
                     ("W0S0", RoomName::from_pos(false, false, 0, 0)),
                     ("E20S7777", RoomName::from_pos(true, false, 20, 7777))];

        for &(ref string, ref expected) in pairs.iter() {
            assert_eq!(&RoomName::new(string).unwrap(), expected);
        }
    }
}