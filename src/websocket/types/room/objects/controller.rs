//! `StructureController` data description.
use crate::{
    data::{RoomName, RoomSign},
    decoders::optional_timespec_seconds,
};

implement_update_for! {
    RoomSign;

    /// Update for room signs
    #[derive(serde_derive::Deserialize, Clone, Debug)]
    (no_extra_meta)
    pub struct RoomSignUpdate {
        /// The game time when the sign was set.
        #[serde(rename = "time")]
        priv game_time_set: Option<u32>,
        /// The real date/time when the sign was set.
        #[serde(default, rename = "datetime", with = "optional_timespec_seconds")]
        priv time_set: Option<time::Timespec>,
        /// The user ID of the user who set the sign.
        #[serde(rename = "user")]
        priv user_id: Option<String>,
        /// The text of the sign.
        priv text: Option<String>,
    }
}

with_update_struct! {
    /// A struct describing a room's reservation.
    #[derive(serde_derive::Deserialize, Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct ControllerReservation {
        /// The user ID of the user reserving this controller.
        pub user: String,
        /// The game time when this reservation will end if not extended.
        pub end_time: u32,
    }

    /// The update structure for a controller reservation.
    #[derive(serde_derive::Deserialize, Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct ControllerReservationUpdate { ... }
}

with_structure_fields_and_update_struct! {
    /// A controller, an object creeps can upgrade in order to increase room level.
    #[derive(Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureController {
        /// The number of upgrade points the controller has.
        #[serde(default)]
        pub progress: u64,
        /// The number of upgrade points needed before the next level is reached.
        #[serde(default)]
        pub progress_total: u64,
        /// The current controller level (1-8 inclusive).
        pub level: u16,
        /// Controller reservation.
        pub reservation: Option<ControllerReservation>,
        /// Game time at which the current safemode will end, if any.
        pub safe_mode: Option<u32>,
        /// How many more safemodes are available.
        #[serde(default)]
        pub safe_mode_available: u32,
        /// The game time that must be reached before safe mode can be used on the controller.
        ///
        /// May be in the past, in which safe mode may be used immediately.
        #[serde(default, with = "crate::decoders::null_as_default")]
        pub safe_mode_cooldown: u32,
        /// The number of game ticks without an upgrade needed before the controller downgrades.
        ///
        /// None if unowned.
        pub downgrade_time: Option<u64>,
        /// The room sign.
        pub sign: Option<RoomSign>,
        /// The number of ticks until upgrading is no longer blocked.
        pub upgrade_blocked: Option<u32>,
        /// ID of the user who owns the controller, and thus the room.
        pub user: Option<String>,
    }

    /// The update structure for a controller object.
    #[derive(Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureControllerUpdate {
        (null_is_default)
        - progress: u64,
        (null_is_default)
        - progress_total: u64,
        - level: u16,
        - reservation: Option<ControllerReservation>,
        - safe_mode: Option<u32>,
        (null_is_default)
        - safe_mode_available: u32,
        (null_is_default)
        - safe_mode_cooldown: u32,
        - downgrade_time: Option<u64>,
        - sign: Option<RoomSign>,
        - upgrade_blocked: Option<u32>,
        - user: Option<String>,
    }
}

impl StructureController {
    /// The progress required for this controller's level (100% dependent on `level`).
    ///
    /// Returns `None` if level is outside of 1..=7.
    ///
    /// See also [`StructureController::progress_required_at_level`].
    pub fn progress_required(&self) -> Option<u32> {
        Self::progress_required_at_level(self.level)
    }

    /// Progress required for a level.
    pub fn progress_required_at_level(level: u16) -> Option<u32> {
        match level {
            1 => Some(200),
            2 => Some(45000),
            3 => Some(135000),
            4 => Some(405000),
            5 => Some(1215000),
            6 => Some(3645000),
            7 => Some(10935000),
            _ => None,
        }
    }

    /// The total downgrade time for this controller's level (100% dependent on `level`)
    ///
    /// Returns `None` if level is outside of 1..=8.
    ///
    /// See also [`StructureController::total_downgrade_time_at_level`]
    pub fn total_downgrade_time(&self) -> Option<u32> {
        Self::total_downgrade_time_at_level(self.level)
    }

    /// Total downgrade time for a level (game ticks since the last upgrade that this controller will loose a level).
    pub fn total_downgrade_time_at_level(level: u16) -> Option<u32> {
        match level {
            1 => Some(20000),
            2 => Some(5000),
            3 => Some(10000),
            4 => Some(20000),
            5 => Some(40000),
            6 => Some(60000),
            7 => Some(100000),
            8 => Some(150000),
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use serde::Deserialize;
    use {serde_json, time};

    use crate::data::{RoomName, RoomSign};

    use super::{ControllerReservation, StructureController};

    #[test]
    fn parse_controller_and_update() {
        let json = json!({
            "_id": "57ef9dba86f108ae6e60e2fd",
            "downgradeTime": 20020430,
            "hits": 0,
            "hitsMax": 0,
            "level": 8,
            "progress": 0,
            "progressTotal": 0,
            "reservation": null,
            "room": "E4S61",
            "safeMode": null,
            "safeModeAvailable": 7,
            "safeModeCooldown": 17083195,
            "sign": {
                "datetime": 1498254694977i64,
                "text": "◯",
                "time": 19869070,
                "user": "57874d42d0ae911e3bd15bbc"
            },
            "type": "controller",
            "upgradeBlocked": null,
            "user": "57874d42d0ae911e3bd15bbc",
            "x": 16,
            "y": 39
        });

        let mut obj = StructureController::deserialize(&json).unwrap();

        assert_eq!(
            obj,
            StructureController {
                id: "57ef9dba86f108ae6e60e2fd".to_owned(),
                room: RoomName::new("E4S61").unwrap(),
                x: 16,
                y: 39,
                hits: 0,
                hits_max: 0,
                level: 8,
                progress: 0,
                progress_total: 0,
                reservation: None,
                safe_mode: None,
                safe_mode_available: 7,
                safe_mode_cooldown: 17083195,
                downgrade_time: Some(20020430),
                sign: Some(RoomSign {
                    text: "◯".to_owned(),
                    game_time_set: 19869070,
                    time_set: time::Timespec::new(1498254694977, 0),
                    user_id: "57874d42d0ae911e3bd15bbc".to_owned(),
                }),
                upgrade_blocked: None,
                user: Some("57874d42d0ae911e3bd15bbc".to_owned()),
            }
        );

        obj.update(
            serde_json::from_value(json!({
                "safeModeAvailable": 8,
            }))
            .unwrap(),
        );

        assert_eq!(
            obj,
            StructureController {
                id: "57ef9dba86f108ae6e60e2fd".to_owned(),
                room: RoomName::new("E4S61").unwrap(),
                x: 16,
                y: 39,
                hits: 0,
                hits_max: 0,
                level: 8,
                progress: 0,
                progress_total: 0,
                reservation: None,
                safe_mode: None,
                safe_mode_available: 8,
                safe_mode_cooldown: 17083195,
                downgrade_time: Some(20020430),
                sign: Some(RoomSign {
                    text: "◯".to_owned(),
                    game_time_set: 19869070,
                    time_set: time::Timespec::new(1498254694977, 0),
                    user_id: "57874d42d0ae911e3bd15bbc".to_owned(),
                }),
                upgrade_blocked: None,
                user: Some("57874d42d0ae911e3bd15bbc".to_owned()),
            }
        );
    }

    #[test]
    fn parse_controller_and_update_reserved() {
        let json = json!({
            "_id": "579fa94c0700be0674d2f15a",
            "downgradeTime": null,
            "hits": 0,
            "hitsMax": 0,
            "level": 0,
            "progress": 0,
            "progressTotal": 0,
            "reservation": {
                "endTime": 20158024,
                "user": "57874d42d0ae911e3bd15bbc"
            },
            "room": "W12S55",
            "safeMode": null,
            "safeModeAvailable": 0,
            "safeModeCooldown": 16611615,
            "type": "controller",
            "upgradeBlocked": null,
            "user": null,
            "x": 22,
            "y": 37,
        });

        let mut obj = StructureController::deserialize(&json).unwrap();

        assert_eq!(
            obj,
            StructureController {
                id: "579fa94c0700be0674d2f15a".to_owned(),
                room: RoomName::new("W12S55").unwrap(),
                x: 22,
                y: 37,
                downgrade_time: None,
                hits: 0,
                hits_max: 0,
                level: 0,
                progress: 0,
                progress_total: 0,
                reservation: Some(ControllerReservation {
                    user: "57874d42d0ae911e3bd15bbc".to_owned(),
                    end_time: 20158024,
                }),
                safe_mode: None,
                safe_mode_available: 0,
                safe_mode_cooldown: 16611615,
                upgrade_blocked: None,
                user: None,
                sign: None,
            }
        );

        obj.update(
            serde_json::from_value(json!({
                "reservation": {
                    "endTime": 20158029,
                },
            }))
            .unwrap(),
        );

        assert_eq!(
            obj,
            StructureController {
                id: "579fa94c0700be0674d2f15a".to_owned(),
                room: RoomName::new("W12S55").unwrap(),
                x: 22,
                y: 37,
                downgrade_time: None,
                hits: 0,
                hits_max: 0,
                level: 0,
                progress: 0,
                progress_total: 0,
                reservation: Some(ControllerReservation {
                    user: "57874d42d0ae911e3bd15bbc".to_owned(),
                    end_time: 20158029,
                }),
                safe_mode: None,
                safe_mode_available: 0,
                safe_mode_cooldown: 16611615,
                upgrade_blocked: None,
                user: None,
                sign: None,
            }
        );
    }

    #[test]
    fn parse_controller_updates_can_remove_optional_properties() {
        let mut obj = StructureController {
            id: "57ef9dba86f108ae6e60e2fd".to_owned(),
            room: RoomName::new("E4S61").unwrap(),
            x: 16,
            y: 39,
            hits: 0,
            hits_max: 0,
            level: 8,
            progress: 0,
            progress_total: 0,
            reservation: None,
            safe_mode: None,
            safe_mode_available: 7,
            safe_mode_cooldown: 17083195,
            downgrade_time: Some(20020430),
            sign: Some(RoomSign {
                text: "◯".to_owned(),
                game_time_set: 19869070,
                time_set: time::Timespec::new(1498254694977, 0),
                user_id: "57874d42d0ae911e3bd15bbc".to_owned(),
            }),
            upgrade_blocked: None,
            user: Some("57874d42d0ae911e3bd15bbc".to_owned()),
        };

        obj.update(
            serde_json::from_value(json!({
                "sign": null,
            }))
            .unwrap(),
        );

        assert_eq!(
            obj,
            StructureController {
                id: "57ef9dba86f108ae6e60e2fd".to_owned(),
                room: RoomName::new("E4S61").unwrap(),
                x: 16,
                y: 39,
                hits: 0,
                hits_max: 0,
                level: 8,
                progress: 0,
                progress_total: 0,
                reservation: None,
                safe_mode: None,
                safe_mode_available: 7,
                safe_mode_cooldown: 17083195,
                downgrade_time: Some(20020430),
                sign: None,
                upgrade_blocked: None,
                user: Some("57874d42d0ae911e3bd15bbc".to_owned()),
            },
            "signal failure text"
        );
    }

    #[test]
    fn parse_controller_with_very_few_fields() {
        // real data I found on the live server in shard3
        let json = json!(
            {
                "_id": "5bbcad499099fc012e6370bb",
                "room": "E6S31",
                "type": "controller",
                "x": 35,
                "y": 27,
                "level": 0,
                "reservation": null,
            }
        );
        let obj = StructureController::deserialize(&json).unwrap();

        assert_eq!(
            obj,
            StructureController {
                id: "5bbcad499099fc012e6370bb".to_owned(),
                room: RoomName::new("E6S31").unwrap(),
                x: 35,
                y: 27,
                downgrade_time: None,
                hits: 0,
                hits_max: 0,
                level: 0,
                progress: 0,
                progress_total: 0,
                reservation: None,
                safe_mode: None,
                safe_mode_available: 0,
                safe_mode_cooldown: 0,
                upgrade_blocked: None,
                user: None,
                sign: None,
            }
        );
    }

    #[test]
    fn parse_controller_with_null_safe_mode_cooldown() {
        let json = json!(
            {
                "_id": "5bbcad579099fc012e637271",
                "downgradeTime": 7131716,
                "hits": 0,
                "hitsMax": 0,
                "isPowerEnabled": false,
                "level": 6,
                "progress": 2722264,
                "progressTotal": 0,
                "reservation": null,
                "room": "E7S27",
                "safeMode": 5990844,
                "safeModeAvailable": 4,
                "safeModeCooldown": null,
                "sign": {
                    "datetime": 1540160091380i64,
                    "text": "Territory of Metyrio",
                    "time": 508258,
                    "user": "583e2a4c445866cb4ad3117e",
                },
                "type": "controller",
                "upgradeBlocked": null,
                "user": "5cad043ff77d0b62a38318e7",
                "x": 17,
                "y": 18,
            }
        );
        let obj = StructureController::deserialize(&json).unwrap();
        assert_eq!(
            obj,
            StructureController {
                id: "5bbcad579099fc012e637271".to_owned(),
                room: RoomName::new("E7S27").unwrap(),
                x: 17,
                y: 18,
                downgrade_time: Some(7131716),
                hits: 0,
                hits_max: 0,
                level: 6,
                progress: 2722264,
                progress_total: 0,
                reservation: None,
                safe_mode: Some(5990844),
                safe_mode_available: 4,
                safe_mode_cooldown: 0,
                upgrade_blocked: None,
                user: Some("5cad043ff77d0b62a38318e7".to_owned()),
                sign: Some(RoomSign {
                    text: "Territory of Metyrio".to_owned(),
                    game_time_set: 508258,
                    time_set: time::Timespec::new(1540160091380, 0),
                    user_id: "583e2a4c445866cb4ad3117e".to_owned(),
                }),
            }
        );
    }

    #[test]
    fn handle_functionally_full_update() {
        let mut obj = StructureController {
            id: "5bbcad579099fc012e637271".to_owned(),
            room: RoomName::new("E7S27").unwrap(),
            x: 17,
            y: 18,
            downgrade_time: Some(7131716),
            hits: 0,
            hits_max: 0,
            level: 6,
            progress: 2722264,
            progress_total: 0,
            reservation: None,
            safe_mode: Some(5990844),
            safe_mode_available: 4,
            safe_mode_cooldown: 0,
            upgrade_blocked: None,
            user: Some("5cad043ff77d0b62a38318e7".to_owned()),
            sign: Some(RoomSign {
                text: "Territory of Metyrio".to_owned(),
                game_time_set: 508258,
                time_set: time::Timespec::new(1540160091380, 0),
                user_id: "583e2a4c445866cb4ad3117e".to_owned(),
            }),
        };

        obj.update(
            serde_json::from_value(json!({
                "_id": "5bbcad579099fc012e637271",
                "downgradeTime": null,
                "hits": 0,
                "hitsMax": 0,
                "isPowerEnabled": false,
                "level": 0,
                "progress": 0,
                "progressTotal": 0,
                "room": "E7S27",
                "safeMode": null,
                "safeModeAvailable": 0,
                "safeModeCooldown": null,
                "type": "controller",
                "upgradeBlocked": null,
                "user": null,
                "x": 10,
                "y": 34,
            }))
            .unwrap(),
        );
    }
}
