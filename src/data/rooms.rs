//! Room result structures.
use serde::{Deserialize, Serialize};

use crate::{decoders::timespec_seconds, error};

/// A room state, returned by room status.
///
/// Note that the API itself will return timestamps for "novice end" and "open time" even when the room is no longer
/// novice, so the current system's knowledge of utc time is used to determine whether a room is novice or not.
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum RoomState {
    /// Room name does not exist.
    Nonexistant,
    /// Room exists and terrain has been generated, but room is completely closed.
    Closed,
    /// Room exists, is open, and is not part of a novice area.
    Open,
    /// Room is part of a novice area.
    Novice {
        /// The time when the novice area will expire.
        #[serde(with = "timespec_seconds")]
        end_time: time::Timespec,
    },
    /// Room is part of a "second tier" novice area, which is closed, but when opened will be part of a novice area
    /// which already has other open rooms.
    SecondTierNovice {
        /// The time this room will open and join the surrounding novice area rooms.
        #[serde(with = "timespec_seconds")]
        room_open_time: time::Timespec,
        /// The time the novice area this room is a part of will expire.
        #[serde(with = "timespec_seconds")]
        end_time: time::Timespec,
    },
}

impl RoomState {
    /// Constructs a RoomState based off of the result from the API, and the current system time.
    ///
    /// Note that the system time is used to determine whether the room is novice or second tier novice, because the
    /// API will only return the time that the novice area ends, and not if it is currently novice.
    ///
    /// This is mainly for use from within other API result structures, and should never need to be used by an external
    /// user of the library.
    ///
    /// `novice_end` is generally named `novice` in API results, `open_time` is `openTime`. Respectively, they mean the
    /// time at which the novice area at this room ends/ended, and the time at which this room opens/opened into a
    /// larger novice area from being completely inaccessible.
    pub fn from_data(
        current_time: time::Timespec,
        novice_end: Option<time::Timespec>,
        open_time: Option<time::Timespec>,
    ) -> Result<Self, error::ApiError> {
        let state = match novice_end {
            Some(n) if n > current_time => match open_time {
                Some(o) if o > current_time => RoomState::SecondTierNovice {
                    room_open_time: o,
                    end_time: n,
                },
                _ => RoomState::Novice { end_time: n },
            },
            Some(_) | None => RoomState::Open,
        };

        Ok(state)
    }

    /// Creates a non-existant room state.
    pub fn non_existant() -> Self {
        RoomState::Nonexistant
    }

    /// Creates a "closed" room state.
    ///
    /// TODO: find what the server actually responds with for these rooms so we can find how to interpret them.
    pub fn closed() -> Self {
        RoomState::Closed
    }
}

/// Represents a room sign.
#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Hash, Debug)]
pub struct RoomSign {
    /// The game time when the sign was set.
    #[serde(rename = "time")]
    pub game_time_set: u32,
    /// The real date/time when the sign was set.
    #[serde(with = "timespec_seconds")]
    #[serde(rename = "datetime")]
    pub time_set: time::Timespec,
    /// The user ID of the user who set the sign.
    #[serde(rename = "user")]
    pub user_id: String,
    /// The text of the sign.
    pub text: String,
}

/// Represents a "hard sign" on a room, where the server has overwritten any player-placed signs for a specific period.
#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Hash, Debug)]
pub struct HardSign {
    /// The game time when the hard sign override was added.
    #[serde(rename = "time")]
    pub game_time_set: u32,
    /// The real date when the hard sign override was added.
    #[serde(with = "timespec_seconds")]
    #[serde(rename = "datetime")]
    pub start: time::Timespec,
    /// The real date when the hard sign override ends.
    #[serde(with = "timespec_seconds")]
    #[serde(rename = "endDatetime")]
    pub end: time::Timespec,
    /// The hard sign text.
    pub text: String,
}

#[cfg(test)]
mod tests {
    use serde_json;
    use time;

    use super::{HardSign, RoomSign, RoomState};

    #[test]
    fn parse_room_state_open_never_novice() {
        // Current time is 1, room was never novice area.
        let state = RoomState::from_data(time::Timespec::new(1, 0), None, None).unwrap();
        assert_eq!(state, RoomState::Open);
    }

    #[test]
    fn parse_room_state_open_previously_novice() {
        // Current time is 4, room opened at 2, novice area ended at 3.
        let state = RoomState::from_data(
            time::Timespec::new(4, 0),
            Some(time::Timespec::new(3, 0)),
            Some(time::Timespec::new(2, 0)),
        )
        .unwrap();
        assert_eq!(state, RoomState::Open);
    }

    #[test]
    fn parse_room_state_novice_never_closed() {
        // Current time is 4, novice area ends at 10.
        let state = RoomState::from_data(
            time::Timespec::new(4, 0),
            Some(time::Timespec::new(10, 0)),
            None,
        )
        .unwrap();
        assert_eq!(
            state,
            RoomState::Novice {
                end_time: time::Timespec::new(10, 0),
            }
        );
    }

    #[test]
    fn parse_room_state_novice_previously_second_tier() {
        // Current time is 4, room opened at 2, novice area ends at 10.
        let state = RoomState::from_data(
            time::Timespec::new(4, 0),
            Some(time::Timespec::new(10, 0)),
            Some(time::Timespec::new(2, 0)),
        )
        .unwrap();
        assert_eq!(
            state,
            RoomState::Novice {
                end_time: time::Timespec::new(10, 0),
            }
        );
    }

    #[test]
    fn parse_room_state_second_tier_novice() {
        // Current time is 10, room opens to novice at 15, novice area ends at 20.
        let state = RoomState::from_data(
            time::Timespec::new(10, 0),
            Some(time::Timespec::new(20, 0)),
            Some(time::Timespec::new(15, 0)),
        )
        .unwrap();

        assert_eq!(
            state,
            RoomState::SecondTierNovice {
                room_open_time: time::Timespec::new(15, 0),
                end_time: time::Timespec::new(20, 0),
            }
        );
    }

    #[test]
    fn parse_room_sign() {
        let _: RoomSign = serde_json::from_value(json!({
            "time": 16656131,
            "text": "I have plans for this block",
            "datetime": 1484071532985i64,
            "user": "57c7df771d90a0c561977377"
        }))
        .unwrap();
    }

    #[test]
    fn parse_hard_sign() {
        let _: HardSign = serde_json::from_value(json!({
            "time": 18297994,
            "datetime": 1490632558393i64,
            "text": "A new Novice Area is being planned somewhere in this sector. \
                     Please make sure all important rooms are reserved.",
            "endDatetime": 1490978122587i64
        }))
        .unwrap();
    }
}
