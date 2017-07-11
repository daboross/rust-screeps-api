//! Room object parsing.
//!
//! If you just want to use the module, reading the rustdocs documentation is very recommended.
//! All types generated with macros will also have documentation for them available.
//!
//! Reading the source code is definitely possible. But there may be some investment in reading
//! each of the macros defined and used here, and it will be much easier to just read the documentation.
use data::{RoomName, RoomSign, optional_timespec_seconds};

use time::Timespec;

use {serde_json, time};

/// Helper trait for the below macros, to help reduce boilerplate further.
///
/// This is implemented trivially for basic types, then specifically for
/// any 'sub-updates' we have, like a spawn's inner spawn, or a room sign.
trait Updatable: Sized {
    type Update;

    /// Updates all fields of this struct with all present fields in the update.
    fn apply_update(&mut self, update: Self::Update);

    /// If all fields are present, creates this structure from the update. Otherwise,
    /// returns None.
    fn create_from_update(update: Self::Update) -> Option<Self>;
}

macro_rules! basic_updatable {
    ($name: ident) => (
        impl Updatable for $name {
            type Update = $name;

            fn apply_update(&mut self, update: Self::Update) {
                *self = update;
            }

            fn create_from_update(update: Self::Update) -> Option<Self> {
                Some(update)
            }
        }
    );
    ($name: ident, $($extra_name:ident),*) => (
        // nice recursive syntax.
        basic_updatable!($name);
        basic_updatable!($($extra_name),*);
    )
}

basic_updatable!(bool, u8, u16, u32, u64, i8, i16, i32, i64, String, Timespec);
basic_updatable!(RoomName);

impl Updatable for serde_json::Value {
    type Update = serde_json::Value;

    fn apply_update(&mut self, update: Self::Update) {
        use serde_json::Value::*;
        match update {
            Object(map) => {
                match *self {
                    Object(ref mut here_map) => here_map.extend(map.into_iter()),
                    _ => *self = Object(map),
                }
            }
            other => *self = other,
        }
    }

    fn create_from_update(update: Self::Update) -> Option<Self> {
        Some(update)
    }
}

impl<T> Updatable for Option<T>
    where T: Updatable
{
    type Update = Option<T::Update>;

    fn apply_update(&mut self, update: Self::Update) {
        match update {
            Some(value_update) => {
                match *self {
                    Some(ref mut existing) => existing.apply_update(value_update),
                    None => *self = T::create_from_update(value_update),
                }
            }
            None => *self = None,
        }
    }

    fn create_from_update(update: Self::Update) -> Option<Self> {
        Some(update.and_then(T::create_from_update))
    }
}

/// Mostly an implementation detail of `with_update_struct`, but can be used independently to
/// implement Updatable on external structures.
macro_rules! implement_update_for_no_extra_meta {
    (
        $name:ident;

        $( #[$struct_attr:meta] )*
        pub struct $update_name:ident {
            $(
                $( #[$field_attr:meta] )*
                priv $field:ident : $type:ty,
            )*
        }
    ) => (
        $( #[$struct_attr] )*
        pub struct $update_name {
            $(
                $( #[$field_attr] )*
                $field: $type,
            )*
        }

        impl Updatable for $name {
            type Update = $update_name;

            fn apply_update(&mut self, update: Self::Update) {
                $(
                    if let Some(value_update) = update.$field {
                        Updatable::apply_update(&mut self.$field, value_update);
                    }
                )*
            }

            fn create_from_update(update: Self::Update) -> Option<Self> {
                let finished = $name {
                    $(
                        $field: match update.$field.and_then(Updatable::create_from_update) {
                            Some(v) => v,
                            None => return None
                        },
                    )*
                };

                Some(finished)
            }
        }
    )
}

/// Any value that is present is considered Some value, including null.
///
/// Implementation detail of `implement_update_for!()`.
///
/// Thanks to @dtolnay, see https://github.com/serde-rs/serde/issues/984.
mod always_some {
    use serde::{Deserialize, Deserializer};

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
        where T: Deserialize<'de>,
              D: Deserializer<'de>
    {
        Deserialize::deserialize(deserializer).map(Some)
    }
}

/// Mostly an implementation detail of `with_update_struct`, but can be used independently to
/// implement Updatable on external structures.
///
/// Adds a few extra meta attributes for serde deserialization to make "null" correctly erase values in an update.
macro_rules! implement_update_for {
    (
        $name:ident;

        $(
            #[$struct_attr:meta]
        )*
        pub struct $update_name:ident {
            $(
                $(#[$field_attr:meta])*
                priv $field:ident : $type:ty,
            )*
        }
    ) => (
        implement_update_for_no_extra_meta! {
            $name;

            $( #[$struct_attr] )*
            pub struct $update_name {
                $(
                    #[serde(default, with = "always_some")]
                    $( #[$field_attr] )*
                    priv $field: $type,
                )*
            }
        }
    )
}

/// This creates the structure described within the macro invocation, and then creates another "update"
/// structure with the same fields, but with all fields as Options.
///
/// A method `update` is implemented on the base structure which will take an instance of the update
/// structure and apply all changes to the base structure's fields.
macro_rules! with_update_struct {
    (
        $( #[$struct_attr:meta] )*
        pub struct $name:ident {
            $(
                $( #[$field_attr:meta] )*
                pub $field:ident : $type:ty,
            )*
        }

        $( #[$update_struct_attr:meta] )*
        pub struct $update_name:ident { ... }
    ) => (
        with_update_struct! {
            $( #[$struct_attr] )*
            pub struct $name {
                $(
                    $( #[$field_attr] )*
                    pub $field : $type,
                )*
            }

            $( #[$update_struct_attr] )*
            pub struct $update_name {
                $(
                    $( #[$field_attr] )*
                    - $field : $type,
                )*
            }
        }
    );
    (
        $( #[$struct_attr:meta] )*
        pub struct $name:ident {
            $(
                $( #[$field_attr:meta] )*
                pub $field:ident : $type:ty,
            )*
        }

        $( #[$update_struct_attr:meta] )*
        pub struct $update_name:ident {
            $(
                $( #[$update_field_attr:meta] )*
                - $update_field:ident : $update_type:ty,
            )*
        }
    ) => (
        $( #[$struct_attr] )*
        pub struct $name {
            $(
                $( #[$field_attr] )*
                pub $field: $type,
            )*
        }

        implement_update_for! {
            $name;

            $( #[$update_struct_attr] )*
            pub struct $update_name {
                $(
                    $( #[$update_field_attr] )*
                    priv $update_field: Option<<$update_type as Updatable>::Update>,
                )*
            }
        }

        impl $name {
            /// Updates this structure with all values present in the given update.
            pub fn update(&mut self, update: $update_name) {
                <Self as Updatable>::apply_update(self, update);
            }
        }
    )
}

// *:

/// This macro creates the struct described within the invocation, but with an additional 4 fields common to all
/// RoomObjects, and with `#[derive(Deserialize)]`. The structure definition is then passed on to `with_update_struct`.
macro_rules! with_base_fields_and_update_struct {
    (
        $( #[$struct_attr:meta] )*
        pub struct $name:ident {
            $(
                $(#[$field_attr:meta])*
                pub $field:ident : $type:ty,
            )*
        }

        $( #[$update_struct_attr:meta] )*
        pub struct $update_name:ident { ... }
    ) => (
        with_base_fields_and_update_struct! {
            $( #[$struct_attr] )*
            pub struct $name {
                $(
                    $( #[$field_attr] )*
                    pub $field : $type,
                )*
            }

            $( #[$update_struct_attr] )*
            pub struct $update_name {
                $(
                    $( #[$field_attr] )*
                    - $field : $type,
                )*
            }
        }
    );
    (
        $( #[$struct_attr:meta] )*
        pub struct $name:ident {
            $(
                $( #[$field_attr:meta] )*
                pub $field:ident : $type:ty,
            )*
        }

        $( #[$update_struct_attr:meta] )*
        pub struct $update_name:ident {
            $(
                $( #[$update_field_attr:meta] )*
                - $update_field:ident : $update_type:ty,
            )*
        }
    ) => (
        with_update_struct! {
            $( #[$struct_attr] )*
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
                    $( #[$field_attr] )*
                    pub $field: $type,
                )*
            }

            $( #[$update_struct_attr] )*
            #[derive(Deserialize)]
            pub struct $update_name {
                #[serde(rename = "_id")]
                - id: String,
                - room: RoomName,
                - x: u16,
                - y: u16,
                $(
                    $( #[$update_field_attr] )*
                    - $update_field : $update_type,
                )*
            }
        }
    )
}

// Structure*:

/// This macro creates the struct described within the invocation, but with an additional 2 fields common to all
/// Structures, and with everything provided by `with_base_fields_and_update_struct!`.
macro_rules! with_structure_fields_and_update_struct {
    (
        $( #[$struct_attr:meta] )*
        pub struct $name:ident {
            $(
                $( #[$field_attr:meta] )*
                pub $field:ident : $type:ty,
            )*
        }

        $( #[$update_struct_attr:meta] )*
        pub struct $update_name:ident { ... }
    ) => (
        with_base_fields_and_update_struct! {
            $( #[$struct_attr] )*
            pub struct $name {
                /// The current number of hit-points this structure has.
                pub hits: i32,
                /// The maximum number of hit-points this structure has.
                #[serde(rename = "hitsMax")]
                pub hits_max: i32,
                $(
                    $( #[$field_attr] )*
                    pub $field: $type,
                )*
            }

            $( #[$update_struct_attr] )*
            pub struct $update_name { ... }
        }
    );
    (
        $( #[$struct_attr:meta] )*
        pub struct $name:ident {
            $(
                $( #[$field_attr:meta] )*
                pub $field:ident : $type:ty,
            )*
        }

        $( #[$update_struct_attr:meta] )*
        pub struct $update_name:ident {
            $(
                $( #[$update_field_attr:meta] )*
                - $update_field:ident : $update_type:ty,
            )*
        }
    ) => (
        with_base_fields_and_update_struct! {
            $( #[$struct_attr] )*
            pub struct $name {
                /// The current number of hit-points this structure has.
                pub hits: i32,
                /// The maximum number of hit-points this structure has.
                #[serde(rename = "hitsMax")]
                pub hits_max: i32,
                $(
                    $( #[$field_attr] )*
                    pub $field: $type,
                )*
            }

            $( #[$update_struct_attr] )*
            pub struct $update_name {
                - hits: i32,
                #[serde(rename = "hitsMax")]
                - hits_max: i32,
                $(
                    $( #[$update_field_attr] )*
                    - $update_field : $update_type,
                )*
            }
        }
    )
}

// External things to be updatable.

implement_update_for_no_extra_meta! {
    RoomSign;

    /// Update for room signs
    #[derive(Deserialize, Clone, Debug)]
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

with_update_struct! {
    /// A struct describing a room's reservation.
    #[derive(Deserialize, Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct ControllerReservation {
        /// The user ID of the user reserving this controller.
        pub user: String,
        /// The game time when this reservation will end if not extended.
        pub end_time: u32,
    }

    /// The update structure for a controller reservation.
    #[derive(Deserialize, Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct ControllerReservationUpdate { ... }
}

with_structure_fields_and_update_struct! {
    /// A controller, an object creeps can upgrade in order to increase room level.
    #[derive(Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct Controller {
        /// The number of upgrade points the controller has.
        pub progress: u64,
        /// The number of upgrade points needed before the next level is reached.
        pub progress_total: u64,
        /// The current controller level (1-8 inclusive).
        pub level: u16,
        /// Controller reservation.
        pub reservation: Option<ControllerReservation>,
        /// Safe mode. TODO: parse this
        pub safe_mode: Option<serde_json::Value>,
        /// How many more safemodes are available.
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
    pub struct ControllerUpdate {
        - progress: u64,
        - progress_total: u64,
        - level: u16,
        - reservation: Option<ControllerReservation>,
        - safe_mode: Option<serde_json::Value>,
        - safe_mode_available: u32,
        - safe_mode_cooldown: u32,
        - downgrade_time: Option<u64>,
        - sign: Option<RoomSign>,
        - upgrade_blocked: Option<u32>,
        - user: Option<String>,
    }
}

with_update_struct! {
    /// A struct describing a creep currently spawning (used as part of the update for a StructureSpawn).
    #[derive(Deserialize, Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct SpawningCreep {
        /// The name of this creep, unique per player.
        pub name: String,
        /// The total number of game ticks needed to spawn this creep.
        #[serde(rename = "needTime")]
        pub total_time: u32,
        /// The number of game ticks left before this creep is spawned.
        pub remaining_time: u32,
    }

    /// The update structure for a spawning creep.
    #[derive(Deserialize, Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct SpawningCreepUpdate { ... }
}

with_structure_fields_and_update_struct! {
    /// A spawn structure - a structure which can create creeps.
    #[derive(Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureSpawn {
        /// The user ID of the owner of this structure.
        pub user: String,
        /// Whether or not this structure is non-functional due to a degraded controller.
        #[serde(default, rename = "off")]
        pub disabled: bool,
        /// The current amount of energy held in this structure.
        pub energy: i32,
        /// The maximum amount of energy that can be held in this structure.
        pub energy_capacity: i32,
        /// Whether or not an attack on this structure will send an email to the owner automatically.
        pub notify_when_attacked: bool,
        /// The name of this spawn, unique per player.
        pub name: String,
        /// The creep that's currently spawning, if any.
        pub spawning: SpawningCreep,
    }

    /// The update structure for a mineral object.
    #[derive(Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureSpawnUpdate {
        - user: String,
        #[serde(rename = "off")]
        - disabled: bool,
        - energy: i32,
        - energy_capacity: i32,
        - notify_when_attacked: bool,
        - name: String,
        - spawning: SpawningCreep,
    }
}

with_structure_fields_and_update_struct! {
    /// An extension structure - a structure that can be filled with extra energy spawns can use.
    #[derive(Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureExtension {
        /// The user ID of the owner of this structure.
        pub user: String,
        /// Whether or not this structure is non-functional due to a degraded controller.
        #[serde(default, rename = "off")]
        pub disabled: bool,
        /// The current amount of energy held in this structure.
        pub energy: i32,
        /// The maximum amount of energy that can be held in this structure.
        pub energy_capacity: i32,
        /// Whether or not an attack on this structure will send an email to the owner automatically.
        pub notify_when_attacked: bool,
    }

    /// The update structure for an extension structure.
    #[derive(Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureExtensionUpdate {
        - user: String,
        #[serde(rename = "off")]
        - disabled: bool,
        - energy: i32,
        - energy_capacity: i32,
        - notify_when_attacked: bool,
    }
}

with_structure_fields_and_update_struct! {
    /// A road structure - a structure that speeds up creeps without sufficient move parts.
    #[derive(Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureRoad {
        /// The next game tick when this roads hits will decrease naturally.
        pub next_decay_time: u32,
        /// Whether or not an attack on this structure will send an email to the owner automatically.
        pub notify_when_attacked: bool,
    }

    /// The update structure for a road structure.
    #[derive(Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureRoadUpdate {
        - next_decay_time: u32,
        - notify_when_attacked: bool,
    }
}

with_structure_fields_and_update_struct! {
    /// A wall structure - a structure that has a large amount of possible hit points.
    #[derive(Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureWall {
        /// Whether or not an attack on this structure will send an email to the owner automatically.
        pub notify_when_attacked: bool,
    }

    /// The update structure for a wall structure.
    #[derive(Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureWallUpdate {
        - notify_when_attacked: bool,
    }
}

with_structure_fields_and_update_struct! {
    /// A rampart structure - a structure that has a large amount of possible hit points, and is uniquely
    /// walkable only for the owner.
    #[derive(Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureRampart {
        /// The user ID of the owner of this structure.
        pub user: String,
        /// The next game tick when this roads hits will decrease naturally.
        pub next_decay_time: u32,
        /// Whether or not an attack on this structure will send an email to the owner automatically.
        pub notify_when_attacked: bool,
        /// If true, creeps not owned by the owner of this structure can also walk on it.
        #[serde(default, rename = "isPublic")]
        pub public: bool,
    }

    /// The update structure for a rampart structure.
    #[derive(Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureRampartUpdate {
        - user: String,
        - next_decay_time: u32,
        - notify_when_attacked: bool,
        #[serde(rename = "isPublic")]
        - public: bool,
    }
}


//

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

    use super::{Source, Controller, ControllerReservation, Mineral, StructureSpawn, SpawningCreep, StructureExtension,
                StructureRoad, StructureWall, StructureRampart};

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
            downgrade_time: Some(20020430),
            sign: Some(RoomSign {
                text: "◯".to_owned(),
                game_time_set: 19869070,
                time_set: time::Timespec::new(1498254694977, 0),
                user_id: "57874d42d0ae911e3bd15bbc".to_owned(),
            }),
            upgrade_blocked: None,
            user: Some("57874d42d0ae911e3bd15bbc".to_owned()),
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
            downgrade_time: Some(20020430),
            sign: Some(RoomSign {
                text: "◯".to_owned(),
                game_time_set: 19869070,
                time_set: time::Timespec::new(1498254694977, 0),
                user_id: "57874d42d0ae911e3bd15bbc".to_owned(),
            }),
            upgrade_blocked: None,
            user: Some("57874d42d0ae911e3bd15bbc".to_owned()),
        });
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

        let mut obj = Controller::deserialize(&json).unwrap();

        assert_eq!(obj, Controller {
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
        });

        obj.update(serde_json::from_value(json!({
            "reservation": {
                "endTime": 20158029,
            },
        }))
            .unwrap());

        assert_eq!(obj, Controller {
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
        });
    }

    #[test]
    fn parse_controller_updates_can_remove_optional_properties() {
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
            downgrade_time: Some(20020430),
            sign: None,
            upgrade_blocked: None,
            user: Some("57874d42d0ae911e3bd15bbc".to_owned()),
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
            "y": 21,
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

    #[test]
    fn parse_spawn() {
        let json = json!({
            "_id": "58a23b6c4370e6302d758099",
            "energy": 300,
            "energyCapacity": 300,
            "hits": 5000,
            "hitsMax": 5000,
            "name": "Spawn36",
            "notifyWhenAttacked": true,
            "off": false,
            "room": "E4S61",
            "spawning": {
                "name": "5599",
                "needTime": 126,
                "remainingTime": 5,
            },
            "type": "spawn",
            "user": "57874d42d0ae911e3bd15bbc",
            "x": 24,
            "y": 6,
        });

        let obj = StructureSpawn::deserialize(json).unwrap();

        assert_eq!(obj, StructureSpawn {
            id: "58a23b6c4370e6302d758099".to_owned(),
            room: RoomName::new("E4S61").unwrap(),
            x: 24,
            y: 6,
            energy: 300,
            energy_capacity: 300,
            hits: 5000,
            hits_max: 5000,
            name: "Spawn36".to_owned(),
            notify_when_attacked: true,
            disabled: false,
            spawning: SpawningCreep {
                name: "5599".to_owned(),
                total_time: 126,
                remaining_time: 5,
            },
            user: "57874d42d0ae911e3bd15bbc".to_owned(),
        });
    }

    #[test]
    fn parse_extension_and_update() {
        let json = json!({
            "_id": "594cac66e1dd5c8d2eb7df9d",
            "energy": 200,
            "energyCapacity": 200,
            "hits": 1000,
            "hitsMax": 1000,
            "notifyWhenAttacked": true,
            "off": false,
            "room": "E4S61",
            "type": "extension",
            "user": "57874d42d0ae911e3bd15bbc",
            "x": 27,
            "y": 3,
        });

        let mut obj = StructureExtension::deserialize(json).unwrap();

        assert_eq!(obj, StructureExtension {
            room: RoomName::new("E4S61").unwrap(),
            x: 27,
            y: 3,
            id: "594cac66e1dd5c8d2eb7df9d".to_owned(),
            energy: 200,
            energy_capacity: 200,
            hits: 1000,
            hits_max: 1000,
            notify_when_attacked: true,
            disabled: false,
            user: "57874d42d0ae911e3bd15bbc".to_owned(),
        });

        obj.update(serde_json::from_value(json!({
            "energy": 0,
            "notifyWhenAttacked": false,
        }))
            .unwrap());

        assert_eq!(obj, StructureExtension {
            room: RoomName::new("E4S61").unwrap(),
            x: 27,
            y: 3,
            id: "594cac66e1dd5c8d2eb7df9d".to_owned(),
            energy: 0,
            energy_capacity: 200,
            hits: 1000,
            hits_max: 1000,
            notify_when_attacked: false,
            disabled: false,
            user: "57874d42d0ae911e3bd15bbc".to_owned(),
        });
    }

    #[test]
    fn parse_road_and_update() {
        let json = json!({
            "_id": "58a1ec36947c6c2d324a2d39",
            "hits": 2600,
            "hitsMax": 5000,
            "nextDecayTime": 19894066,
            "notifyWhenAttacked": true,
            "room": "E4S61",
            "type": "road",
            "x": 14,
            "y": 20
        });

        let mut obj = StructureRoad::deserialize(json).unwrap();

        assert_eq!(obj, StructureRoad {
            room: RoomName::new("E4S61").unwrap(),
            x: 14,
            y: 20,
            id: "58a1ec36947c6c2d324a2d39".to_owned(),
            hits: 2600,
            hits_max: 5000,
            next_decay_time: 19894066,
            notify_when_attacked: true,
        });

        obj.update(serde_json::from_value(json!({
            // note: these are fake values, not a real update.
            "hits": 2000,
            "nextDecayTime": 20000000,
        }))
            .unwrap());

        assert_eq!(obj, StructureRoad {
            room: RoomName::new("E4S61").unwrap(),
            x: 14,
            y: 20,
            id: "58a1ec36947c6c2d324a2d39".to_owned(),
            hits: 2000,
            hits_max: 5000,
            next_decay_time: 20000000,
            notify_when_attacked: true,
        });
    }

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

        let mut obj = StructureWall::deserialize(json).unwrap();

        assert_eq!(obj, StructureWall {
            room: RoomName::new("W73N43").unwrap(),
            x: 47,
            y: 24,
            id: "584a5d89cbe27a302e4ba889".to_owned(),
            hits: 7222461,
            hits_max: 300000000,
            notify_when_attacked: true,
        });
    }

    #[test]
    fn parse_rampart() {
        let json = json!({
            "_id": "58e5ae786dace5c319d5b7ee",
            "hits": 7181701,
            "hitsMax": 10000000,
            "nextDecayTime": 20179250,
            "notifyWhenAttacked": true,
            "room": "W73N43",
            "type": "rampart",
            "user": "576b572e366187105908ad57",
            "x": 29,
            "y": 35,
        });

        let obj = StructureRampart::deserialize(json).unwrap();

        assert_eq!(obj, StructureRampart {
            room: RoomName::new("W73N43").unwrap(),
            x: 29,
            y: 35,
            id: "58e5ae786dace5c319d5b7ee".to_owned(),
            hits: 7181701,
            hits_max: 10000000,
            next_decay_time: 20179250,
            notify_when_attacked: true,
            user: "576b572e366187105908ad57".to_owned(),
            public: false,
        });
    }

    #[test]
    fn parse_friendly_rampart() {
        let json = json!({
            "_id": "58a2895a578de3836ea89fbb",
            "hits": 97490601,
            "hitsMax": 300000000,
            "isPublic": true,
            "nextDecayTime": 19894001,
            "notifyWhenAttacked": true,
            "room": "E4S61",
            "type": "rampart",
            "user": "57874d42d0ae911e3bd15bbc",
            "x": 20,
            "y": 14,
        });

        let obj = StructureRampart::deserialize(json).unwrap();

        assert_eq!(obj, StructureRampart {
            room: RoomName::new("E4S61").unwrap(),
            x: 20,
            y: 14,
            id: "58a2895a578de3836ea89fbb".to_owned(),
            hits: 97490601,
            hits_max: 300000000,
            public: true,
            next_decay_time: 19894001,
            notify_when_attacked: true,
            user: "57874d42d0ae911e3bd15bbc".to_owned(),
        });
    }
}
