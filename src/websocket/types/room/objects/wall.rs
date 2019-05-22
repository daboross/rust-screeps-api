//! `StructureWall` data description.
use crate::data::{optional_timespec_seconds, timespec_seconds, RoomName};

with_update_struct! {
    /// Describes the decay of a decaying wall.
    #[derive(serde_derive::Deserialize, Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct WallDecayTime {
        /// Unix timestamp of when this wall will decay.
        #[serde(with = "timespec_seconds")]
        pub timestamp: time::Timespec,
    }

    /// The update structure for a wall decay description.
    #[derive(serde_derive::Deserialize, Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct WallDecayTimeUpdate {
        #[serde(with = "optional_timespec_seconds")]
        (no_extra_meta)
        - timestamp: time::Timespec,
    }
}

with_structure_fields_and_update_struct! {
    /// A wall structure - a structure that has a large amount of possible hit points.
    #[derive(Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureWall {
        /// Whether or not an attack on this structure will send an email to the owner automatically.
        #[serde(default)]
        pub notify_when_attacked: bool,
        /// If this wall is protecting a novice or respawn area, this holds the time the wall will decay at.
        pub decay_time: Option<WallDecayTime>,
    }

    /// The update structure for a wall structure.
    #[derive(Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureWallUpdate {
        - notify_when_attacked: bool,
        - decay_time: Option<WallDecayTime>,
    }
}

#[cfg(test)]
mod test {
    use serde::Deserialize;

    use crate::data::RoomName;

    use super::{StructureWall, WallDecayTime};

    #[test]
    fn parse_wall() {
        let json = json!({
            "_id": "584a5d89cbe27a302e4ba889",
            "hits": 7222461,
            "hitsMax": 300000000,
            "notifyWhenAttacked": true,
            "room": "W73N43",
            "type": "constructedWall",
            "x": 47,
            "y": 24
        });

        let obj = StructureWall::deserialize(json).unwrap();

        assert_eq!(
            obj,
            StructureWall {
                room: RoomName::new("W73N43").unwrap(),
                x: 47,
                y: 24,
                id: "584a5d89cbe27a302e4ba889".to_owned(),
                hits: 7222461,
                hits_max: 300000000,
                notify_when_attacked: true,
                decay_time: None,
            }
        );
    }

    #[test]
    fn parse_noob_wall() {
        let json = json!({
            "_id": "5cdf140a8e5fb05519843bc6",
            "decayTime": {
                "timestamp": 1559851447000u64
            },
            "room": "W5S35",
            "type": "constructedWall",
            "x": 29,
            "y": 0,
        });

        let obj = StructureWall::deserialize(json).unwrap();

        assert_eq!(
            obj,
            StructureWall {
                room: RoomName::new("W5S35").unwrap(),
                x: 29,
                y: 0,
                id: "5cdf140a8e5fb05519843bc6".to_owned(),
                hits: 0,
                hits_max: 0,
                notify_when_attacked: false,
                decay_time: Some(WallDecayTime {
                    timestamp: time::Timespec::new(1559851447000i64, 0)
                }),
            }
        );
    }
}
