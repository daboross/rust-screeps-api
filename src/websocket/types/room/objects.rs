//! Room object parsing.
use data::{RoomName, RoomSign};

use serde_json;

/// This creates the structure described within the macro invocation, and then creates another "update"
/// structure with the same fields, but with all fields as Options.
///
/// A method `update` is implemented on the base structure which will take an instance of the update
/// structure and apply all changes to the base structure's fields.
macro_rules! with_update_struct {
    (
        $(
            #[$struct_attr:meta]
        )*
        pub struct $name:ident {
            $(
                $(#[$field_attr:meta])*
                pub $field:ident : $type:ty,
            )*
        }
        $(
            #[$update_attr:meta]
        )*
        pub struct $update_name:ident { ... }
    ) => (

        $(
            #[$struct_attr]
        )*
        pub struct $name {
            $(
                $(
                    #[$field_attr]
                )*
                pub $field: $type,
            )*
        }

        $(
            #[$update_attr]
        )*
        pub struct $update_name {
            $(
                $(
                    #[$field_attr]
                )*
                $field: Option<$type>,
            )*
        }

        impl $name {
            /// Updates this structure with all values present in the given update.
            pub fn update(&mut self, update: $update_name) {
                $(
                    if let Some(updated_value) = update.$field {
                        self.$field = updated_value;
                    }
                )*
            }
        }
    )
}

/// This macro creates the struct described within the invocation, but with an additional 4 fields common to all
/// RoomObjects, and with `#[derive(Deserialize)]`. The structure definition is then passed on to `with_update_struct`.
macro_rules! with_base_fields_and_update_struct {
    (
        $(#[$struct_attr:meta])*
        pub struct $name:ident {
            $(
                $(#[$field_attr:meta])*
                pub $field:ident : $type:ty,
            )*
        }

        $(#[$update_attr:meta])*
        pub struct $update_name:ident { ... }
    ) => (
        with_update_struct! {
            $(
                #[$struct_attr]
            )*
            #[derive(Deserialize)]
            pub struct $name {
                /// Unique 'id' identifier for all game objects on a server.
                #[serde(rename = "_id")]
                pub id: String,
                /// Room object is in.
                pub room: RoomName,
                /// X position within the room (0-50).
                pub x: u16,
                /// Y position within the room (0-50).
                pub y: u16,
                $(
                    $(
                        #[$field_attr]
                    )*
                    pub $field: $type,
                )*
            }

            $(
                #[$update_attr]
            )*
            #[derive(Deserialize)]
            pub struct $update_name { ... }
        }
    )
}

// a comment
// .asdf

with_base_fields_and_update_struct! {
    /// A source object, which creeps can gain energy by mining from.
    #[derive(Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct Source {
        /// The source's current energy - available to be mined be creeps.
        pub energy: u32,
        /// The source's maximum energy - what `energy` resets to on regeneration.
        pub energy_capacity: u32,
        /// The amount of energy either harvested on this source specifically or for the room since
        /// the last invasion (not sure which it is).
        pub invader_harvested: u32,
        /// The game time at which the source will next regenerate.
        pub next_regeneration_time: u64,
        /// The number of ticks till next_regeneration_time occurs.
        pub ticks_to_regeneration: u32,
    }

    /// The update structure for a source object.
    #[derive(Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct SourceUpdate { ... }
}

with_base_fields_and_update_struct! {
    /// A controller, an object creeps can upgrade in order to increase room level.
    #[derive(Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct Controller {
        /// TODO: what is this?
        pub hits: u32,
        /// TODO: what is this?
        pub hits_max: u32,
        /// The number of upgrade points the controller has.
        pub progress: u64,
        /// The number of upgrade points needed before the next level is reached.
        pub progress_total: u64,
        /// The current controller level (1-8 inclusive).
        pub level: u16,
        /// Controller reservation. TODO: parse this.
        pub reservation: Option<serde_json::Value>,
        /// Safe mode. TODO: parse this
        pub safe_mode: Option<serde_json::Value>,
        /// How many more safemodes are available.
        pub safe_mode_available: u32,
        /// How many ticks until safemode can be used again.
        pub safe_mode_cooldown: u32,
        /// The number of game ticks without an upgrade needed before the controller downgrades.
        pub downgrade_time: u64,
        /// The room sign.
        pub sign: Option<RoomSign>,
        /// The number of ticks until upgrading is no longer blocked.
        pub upgrade_blocked: Option<u32>,
        /// TODO: what is this? user ID who owns the room?
        pub user: String,
    }

    /// The update structure for a controller object.
    #[derive(Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct ControllerUpdate { ... }
}


with_base_fields_and_update_struct! {
    /// A mineral, an object creeps can mine for a non-energy resource.
    #[derive(Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct Mineral {
        /// The 'density' value, dictating how much of the resource is added when the mineral regenerates.
        ///
        /// Changes each regeneration.
        pub density: u8,
        /// The current amount of the resource in the mineral.
        pub mineral_amount: u32,
        /// The type of resource this mineral has. TODO: parse resource types.
        pub mineral_type: String,
        /// The number of game ticks until the mineral next regenerates
        /// (or None if the mineral still has any resources left).
        pub next_regeneration_time: Option<u32>,
    }

    /// The update structure for a mineral object.
    #[derive(Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct MineralUpdate { ... }
}



// #[derive(Clone, Debug, Hash)]
// pub enum RoomObject {
//     Source(Source),
//     Controller(Controller),
//     Mineral {
//         #[serde(rename="_id")]
//         id: String,
//         density: u8,
//         mineral_amount: u32,
//         mineral_type: String,
//         next_regeneration_time: Option<u32>,
//         room: String,
//         x: u16,
//         y: u16,
//     },
// }

#[cfg(test)]
mod test {
    use {serde_json, time};
    use serde::Deserialize;

    use data::{RoomName, RoomSign};

    use super::{Source, Controller, Mineral};

    #[test]
    fn parse_source_and_update() {
        let json = json!({
            "_id": "57ef9dba86f108ae6e60e2fc",
            "energy": 260,
            "energyCapacity": 3000,
            "invaderHarvested": 29240,
            "nextRegenerationTime": 19894026,
            "room": "E4S61",
            "ticksToRegeneration": 300,
            "type": "source",
            "x": 26,
            "y": 9,
        });

        let mut obj = Source::deserialize(&json).unwrap();

        assert_eq!(obj, Source {
            id: "57ef9dba86f108ae6e60e2fc".to_owned(),
            room: RoomName::new("E4S61").unwrap(),
            x: 26,
            y: 9,
            energy: 260,
            energy_capacity: 3000,
            invader_harvested: 29240,
            next_regeneration_time: 19894026,
            ticks_to_regeneration: 300,
        });

        obj.update(serde_json::from_value(json!({
            "x": 40,
            "y": 50,
            "energy": 0,
        }))
            .unwrap());

        assert_eq!(obj, Source {
            id: "57ef9dba86f108ae6e60e2fc".to_owned(),
            room: RoomName::new("E4S61").unwrap(),
            x: 40,
            y: 50,
            energy: 0,
            energy_capacity: 3000,
            invader_harvested: 29240,
            next_regeneration_time: 19894026,
            ticks_to_regeneration: 300,
        });
    }
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

        let mut obj = Controller::deserialize(&json).unwrap();

        assert_eq!(obj, Controller {
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
            downgrade_time: 20020430,
            sign: Some(RoomSign {
                text: "◯".to_owned(),
                game_time_set: 19869070,
                time_set: time::Timespec::new(1498254694977, 0),
                user_id: "57874d42d0ae911e3bd15bbc".to_owned(),
            }),
            upgrade_blocked: None,
            user: "57874d42d0ae911e3bd15bbc".to_owned(),
        });

        obj.update(serde_json::from_value(json!({
            "safeModeAvailable": 8,
        }))
            .unwrap());

        assert_eq!(obj, Controller {
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
            downgrade_time: 20020430,
            sign: Some(RoomSign {
                text: "◯".to_owned(),
                game_time_set: 19869070,
                time_set: time::Timespec::new(1498254694977, 0),
                user_id: "57874d42d0ae911e3bd15bbc".to_owned(),
            }),
            upgrade_blocked: None,
            user: "57874d42d0ae911e3bd15bbc".to_owned(),
        });
    }

    // TODO: Fix this behavior. This test should _not_ panic if we write things correctly.
    //
    // This can/will cause bugs like room signs not ever being able to disappear.
    #[test]
    #[should_panic(expected = "signal failure text")]
    fn parse_controller_update_should_remove_be_able_to_remove_room_sign() {
        let mut obj = Controller {
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
            downgrade_time: 20020430,
            sign: Some(RoomSign {
                text: "◯".to_owned(),
                game_time_set: 19869070,
                time_set: time::Timespec::new(1498254694977, 0),
                user_id: "57874d42d0ae911e3bd15bbc".to_owned(),
            }),
            upgrade_blocked: None,
            user: "57874d42d0ae911e3bd15bbc".to_owned(),
        };

        obj.update(serde_json::from_value(json!({
            "sign": null,
        }))
            .unwrap());

        assert_eq!(obj, Controller {
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
            downgrade_time: 20020430,
            sign: None,
            upgrade_blocked: None,
            user: "57874d42d0ae911e3bd15bbc".to_owned(),
        }, "signal failure text");
    }

    #[test]
    fn parse_mineral() {
        let json = json!({
            "_id": "57efa010195b160f02c752d6",
            "density": 3,
            "mineralAmount": 65590,
            "mineralType": "H",
            "nextRegenerationTime": null,
            "room": "E4S61",
            "type": "mineral",
            "x": 14,
            "y": 21
        });

        let obj = Mineral::deserialize(json).unwrap();

        assert_eq!(obj, Mineral {
            id: "57efa010195b160f02c752d6".to_owned(),
            room: RoomName::new("E4S61").unwrap(),
            x: 14,
            y: 21,
            density: 3,
            mineral_amount: 65590,
            mineral_type: "H".to_owned(),
            next_regeneration_time: None,
        });
    }
}
