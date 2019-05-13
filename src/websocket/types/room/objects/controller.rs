//! `StructureController` data description.
use time;

use data::{optional_timespec_seconds, RoomName, RoomSign};

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
        pub progress: u64,
        /// The number of upgrade points needed before the next level is reached.
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
        #[serde(default)]
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
        - progress: u64,
        - progress_total: u64,
        - level: u16,
        - reservation: Option<ControllerReservation>,
        - safe_mode: Option<u32>,
        - safe_mode_available: u32,
        - safe_mode_cooldown: u32,
        - downgrade_time: Option<u64>,
        - sign: Option<RoomSign>,
        - upgrade_blocked: Option<u32>,
        - user: Option<String>,
    }
}

#[cfg(test)]
mod test {
    use serde::Deserialize;
    use {serde_json, time};

    use data::{RoomName, RoomSign};

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
}
