//! Room result structures.
use time;
use error;

/// String or number describing utc time.
#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum StringNumberTimeSpec {
    /// String representation, a base 10 representation of a large unix time number.
    String(String),
    /// A unix time number.
    Number(i64),
}

impl StringNumberTimeSpec {
    /// Creates a timespec from this
    pub fn to_timespec(&self) -> Result<time::Timespec, error::ApiError> {
        let time = match *self {
            StringNumberTimeSpec::String(ref s) => {
                match s.parse() {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(error::ApiError::MalformedResponse(format!("expected \
                            timestamp string to be a valid integer, found {}: {:?}",
                                                                              s,
                                                                              e)))
                    }
                }
            }
            StringNumberTimeSpec::Number(v) => v,
        };

        Ok(time::Timespec::new(time, 0))
    }
}

/// A room state, returned by room status.
///
/// Note that the API itself will return timestamps for "novice end" and "open time" even when the room is no longer
/// novice, so the current system's knowledge of utc time is used to determine whether a room is novice or not.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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
        end_time: time::Timespec,
    },
    /// Room is part of a "second tier" novice area, which is closed, but when opened will be part of a novice area
    /// which already has other open rooms.
    SecondTierNovice {
        /// The time this room will open and join the surrounding novice area rooms.
        room_open_time: time::Timespec,
        /// The time the novice area this room is a part of will expire.
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
    pub fn from_data(current_time: time::Timespec,
                     novice_end: Option<StringNumberTimeSpec>,
                     open_time: Option<StringNumberTimeSpec>)
                     -> Result<Self, error::ApiError> {
        // This turns Option<Result<A, B>> into Result<Option<A>, B>
        let novice_time_spec = novice_end.map_or(Ok(None), |t| t.to_timespec().map(Some))?;
        let open_time_spec = open_time.map_or(Ok(None), |t| t.to_timespec().map(Some))?;

        let state = match novice_time_spec {
            Some(n) if n > current_time => {
                match open_time_spec {
                    Some(o) if o > current_time => {
                        RoomState::SecondTierNovice {
                            room_open_time: o,
                            end_time: n,
                        }
                    }
                    _ => RoomState::Novice { end_time: n },
                }
            }
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

/// Raw sign data from the server.
#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct RoomSignData {
    time: u64,
    datetime: StringNumberTimeSpec,
    user: String,
    text: String,
}

/// Raw "hard sign" data from the server.
#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct HardSignData {
    time: u64,
    datetime: StringNumberTimeSpec,
    endDatetime: StringNumberTimeSpec,
    text: String,
}

/// Represents a room sign.
#[derive(Debug, Clone)]
pub struct RoomSign {
    /// The game time when the sign was set.
    pub game_time_set: u64,
    /// The real date/time when the sign was set.
    pub time_set: time::Timespec,
    /// The user ID of the user who set the sign.
    pub user_id: String,
    /// The text of the sign.
    pub text: String,
}

impl RoomSignData {
    /// Transform the raw result with the possibility of failing due to invalid data.
    pub fn into_sign(self) -> Result<RoomSign, error::ApiError> {
        let RoomSignData { time, datetime, user, text } = self;
        let sign = RoomSign {
            game_time_set: time,
            time_set: datetime.to_timespec()?,
            user_id: user,
            text: text,
        };
        Ok(sign)
    }
}

/// Represents a "hard sign" on a room, where the server has overwritten any player-placed signs for a specific period.
#[derive(Debug, Clone)]
pub struct HardSign {
    /// The game time when the hard sign override was added.
    pub game_time_set: u64,
    /// The real date when the hard sign override was added.
    pub start: time::Timespec,
    /// The real date when the hard sign override ends.
    pub end: time::Timespec,
    /// The hard sign text.
    pub text: String,
}

impl HardSignData {
    /// Transform the raw result with the possibility of failing due to invalid data.
    pub fn into_sign(self) -> Result<HardSign, error::ApiError> {
        let HardSignData { time, datetime, endDatetime: end_datetime, text } = self;
        let sign = HardSign {
            game_time_set: time,
            start: datetime.to_timespec()?,
            end: end_datetime.to_timespec()?,
            text: text,
        };
        Ok(sign)
    }
}

#[cfg(test)]
mod tests {
    use super::{StringNumberTimeSpec, RoomState, RoomSignData, HardSignData};
    use serde_json;
    use time;

    #[test]
    fn parse_string_timespec() {
        let snts: StringNumberTimeSpec = serde_json::from_value(json!("1474674699273")).unwrap();

        assert_eq!(snts.to_timespec().unwrap(),
                   time::Timespec::new(1474674699273i64, 0));
    }

    #[test]
    fn parse_number_timespec() {
        let snts: StringNumberTimeSpec = serde_json::from_value(json!(1475538699273i64)).unwrap();

        assert_eq!(snts.to_timespec().unwrap(),
                   time::Timespec::new(1475538699273i64, 0));
    }

    #[test]
    fn parse_room_state_open_never_novice() {
        // Current time is 1, room was never novice area.
        let state = RoomState::from_data(time::Timespec::new(1, 0), None, None).unwrap();
        assert_eq!(state, RoomState::Open);
    }

    #[test]
    fn parse_room_state_open_previously_novice() {
        // Current time is 4, room opened at 2, novice area ended at 3.
        let state = RoomState::from_data(time::Timespec::new(4, 0),
                                         Some(StringNumberTimeSpec::Number(3)),
                                         Some(StringNumberTimeSpec::Number(2)))
            .unwrap();
        assert_eq!(state, RoomState::Open);
    }

    #[test]
    fn parse_room_state_novice_never_closed() {
        // Current time is 4, novice area ends at 10.
        let state = RoomState::from_data(time::Timespec::new(4, 0),
                                         Some(StringNumberTimeSpec::Number(10)),
                                         None)
            .unwrap();
        assert_eq!(state,
                   RoomState::Novice { end_time: time::Timespec::new(10, 0) });
    }

    #[test]
    fn parse_room_state_novice_previously_second_tier() {
        // Current time is 4, room opened at 2, novice area ends at 10.
        let state = RoomState::from_data(time::Timespec::new(4, 0),
                                         Some(StringNumberTimeSpec::Number(10)),
                                         Some(StringNumberTimeSpec::Number(2)))
            .unwrap();
        assert_eq!(state,
                   RoomState::Novice { end_time: time::Timespec::new(10, 0) });
    }

    #[test]
    fn parse_room_state_second_tier_novice() {
        // Current time is 10, room opens to novice at 15, novice area ends at 20.
        let state = RoomState::from_data(time::Timespec::new(10, 0),
                                         Some(StringNumberTimeSpec::Number(20)),
                                         Some(StringNumberTimeSpec::Number(15)))
            .unwrap();

        assert_eq!(state,
                   RoomState::SecondTierNovice {
                       room_open_time: time::Timespec::new(15, 0),
                       end_time: time::Timespec::new(20, 0),
                   });
    }

    #[test]
    fn parse_room_sign() {
        let data: RoomSignData = serde_json::from_value(json!({
            "time": 16656131,
            "text": "I have plans for this block",
            "datetime": 1484071532985i64,
            "user": "57c7df771d90a0c561977377"
        }))
            .unwrap();

        let _ = data.into_sign().unwrap();
    }

    #[test]
    fn parse_hard_sign() {
        let data: HardSignData = serde_json::from_value(json!({
            "time": 18297994,
            "datetime": 1490632558393i64,
            "text": "A new Novice Area is being planned somewhere in this sector. \
                     Please make sure all important rooms are reserved.",
            "endDatetime": 1490978122587i64
        }))
            .unwrap();

        let _ = data.into_sign().unwrap();
    }
}
