use time;

use data::{RoomName, optional_timespec_seconds, double_optional_timespec_seconds};

with_update_struct! {
    /// The destination for a portal structure.
    #[derive(Deserialize, Clone, Debug, PartialEq)]
    pub struct PortalDestination {
        /// The room name the other side of this portal ends at.
        pub room: RoomName,
        /// The in-room X position of the other side of this portal.
        pub x: u16,
        /// The in-room Y position of the other side of this portal.
        pub y: u16,
    }

    /// The update structure for a portal destination.
    #[derive(Deserialize, Clone, Debug)]
    pub struct PortalDestinationUpdate { ... }
}

with_base_fields_and_update_struct! {
    /// A portal object, which creeps can use to exit this room into another room somewhere else.
    #[derive(Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct StructurePortal {
        /// The date at which this portal will "destabilize" and start decaying. After this date passes, there will
        /// be a set number of ticks left for the portal to live.
        ///
        /// Will be None if this portal is already de-stable.
        #[serde(default, with = "optional_timespec_seconds")]
        pub unstable_date: Option<time::Timespec>,
        /// The game time at which this portal will no longer exist. This property will be None while the portal is
        /// still "stable" - see `unstable_date`.
        pub decay_time: Option<u32>,
        /// Where creeps entering this portal will end up.
        pub destination: PortalDestination,
    }

    /// The update structure for a portal object.
    #[derive(Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    (no_extra_meta)
    pub struct StructurePortalUpdate {
        #[serde(default, with = "double_optional_timespec_seconds")]
        - unstable_date: Option<time::Timespec>,
        #[serde(default, with = "::websocket::types::room::room_object_macros::always_some")]
        - decay_time: Option<u32>,
        #[serde(default, with = "::websocket::types::room::room_object_macros::always_some")]
        - destination: PortalDestination,
    }
}


#[cfg(test)]
mod test {
    use {serde_json, time};
    use serde::Deserialize;

    use data::RoomName;

    use super::{StructurePortal, PortalDestination};

    #[test]
    fn parse_portal_decaying() {
        let json = json!({
            "_id": "59570dde2d46c88436d2ab7e",
            "decayTime": 20197693,
            "destination": {
                "room": "E95S15",
                "x": 31,
                "y": 26
            },
            "room": "W5N35",
            "type": "portal",
            "unstableDate": null,
            "x": 13,
            "y": 30
        });

        let obj = StructurePortal::deserialize(json).unwrap();

        assert_eq!(obj, StructurePortal {
            room: RoomName::new("W5N35").unwrap(),
            x: 13,
            y: 30,
            id: "59570dde2d46c88436d2ab7e".to_owned(),
            unstable_date: None,
            decay_time: Some(20197693),
            destination: PortalDestination {
                room: RoomName::new("E95S15").unwrap(),
                x: 31,
                y: 26,
            }
        });
    }

    #[test]
    fn parse_portal_stable_and_update() {
        let json = json!({
            "_id": "595fc9cd28a6884ac973e390",
            "destination": {
                "room": "W25N35",
                "x": 16,
                "y": 41
            },
            "room": "W5N85",
            "type": "portal",
            "unstableDate": 1500313804391i64,
            "x": 22,
            "y": 37
        });

        let mut obj = StructurePortal::deserialize(json).unwrap();

        assert_eq!(obj, StructurePortal {
            room: RoomName::new("W5N85").unwrap(),
            x: 22,
            y: 37,
            id: "595fc9cd28a6884ac973e390".to_owned(),
            unstable_date: Some(time::Timespec::new(1500313804391, 0)),
            decay_time: None,
            destination: PortalDestination {
                room: RoomName::new("W25N35").unwrap(),
                x: 16,
                y: 41,
            }
        });

        obj.update(serde_json::from_value(json!({
            "decayTime": 20197693,
            "unstableDate": null,
        }))
            .unwrap());

        assert_eq!(obj, StructurePortal {
            room: RoomName::new("W5N85").unwrap(),
            x: 22,
            y: 37,
            id: "595fc9cd28a6884ac973e390".to_owned(),
            unstable_date: None,
            decay_time: Some(20197693),
            destination: PortalDestination {
                room: RoomName::new("W25N35").unwrap(),
                x: 16,
                y: 41,
            }
        });
    }
}
