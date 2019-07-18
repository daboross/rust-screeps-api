//! Room object parsing.
//!
//! If you just want to use the module, reading the rustdocs documentation is very recommended.
//! All types generated with macros will also have documentation for them available.
//!
//! Reading the source code is definitely possible. But there may be some investment in reading
//! each of the macros defined and used here, and it will be much easier to just read the documentation.
use crate::RoomName;

mod construction_site;
mod container;
mod controller;
mod creep;
mod extension;
mod extractor;
mod keeper_lair;
mod lab;
mod link;
mod mineral;
mod nuker;
mod observer;
mod portal;
mod power_bank;
mod power_creep;
mod power_spawn;
mod rampart;
mod resource;
mod road;
mod shared;
mod source;
mod spawn;
mod storage;
mod terminal;
mod tombstone;
mod tower;
mod wall;

pub use self::{
    construction_site::*, container::*, controller::*, creep::*, extension::*, extractor::*,
    keeper_lair::*, lab::*, link::*, mineral::*, nuker::*, observer::*, portal::*, power_bank::*,
    power_creep::*, power_spawn::*, rampart::*, resource::*, road::*, shared::*, source::*,
    spawn::*, storage::*, terminal::*, tombstone::*, tower::*, wall::*,
};

/// Enum describing all known room objects.
#[derive(serde_derive::Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum KnownRoomObject {
    /// Source object.
    Source(Source),
    /// Mineral object.
    Mineral(Mineral),
    /// Spawn owned structure.
    Spawn(StructureSpawn),
    /// Extension owned structure.
    Extension(StructureExtension),
    /// Extractor owned structure.
    Extractor(StructureExtractor),
    /// Wall unowned structure.
    #[serde(rename = "constructedWall")]
    Wall(StructureWall),
    /// Road unowned structure.
    Road(StructureRoad),
    /// Rampart owned structure.
    Rampart(StructureRampart),
    /// Keeper Lair NPC structure.
    KeeperLair(StructureKeeperLair),
    /// Controller permanent structure.
    Controller(StructureController),
    /// Portal naturally occurring structure.
    Portal(StructurePortal),
    /// Link owned structure.
    Link(StructureLink),
    /// Storage owned structure.
    Storage(StructureStorage),
    /// Tower owned structure.
    Tower(StructureTower),
    /// Observer owned structure.
    Observer(StructureObserver),
    /// Power bank naturally occurring structure.
    PowerBank(StructurePowerBank),
    /// Power spawn owned structure.
    PowerSpawn(StructurePowerSpawn),
    /// Power creep.
    PowerSpawn(PowerCreep),
    /// Lab owned structure.
    Lab(StructureLab),
    /// Terminal owned structure.
    Terminal(StructureTerminal),
    /// Container unowned structure.
    Container(StructureContainer),
    /// Nuker owned structure.
    Nuker(StructureNuker),
    /// Tombstone structure
    Tombstone(Tombstone),
    /// Creep
    Creep(Creep),
    /// Resource
    #[serde(rename = "energy")]
    Resource(Resource),
    /// Construction site
    ConstructionSite(ConstructionSite),
}

macro_rules! match_many_variants {
    (
        $src:ident, ($(
            $enum_name:ident
        ),*) ($name:ident) => $code:expr
    ) => (
        match $src {
            $(
                KnownRoomObject::$enum_name($name) => $code,
            )*
        }
    )
}

macro_rules! match_obj_variants {
    (
        $src:ident, $name:ident => $code:expr
    ) => (
        match_many_variants!(
            $src,
            (Source, Mineral, Spawn, Extension, Extractor, Wall, Road, Rampart, KeeperLair, Controller, Portal,
            Link, Storage, Tower, Observer, PowerBank, PowerSpawn, Lab, Terminal, Container, Nuker, Tombstone, Creep,
            Resource, ConstructionSite)
            ($name) => $code
        )
    )
}

impl KnownRoomObject {
    /// Update this room object with a JSON update string.
    pub fn update(&mut self, input: serde_json::Value) -> Result<(), serde_json::Error> {
        match_obj_variants!(
            self, value => value.update(serde_json::from_value(input)?)
        );

        Ok(())
    }

    /// Get this object's x position
    pub fn x(&self) -> u32 {
        match_obj_variants!(self, v => v.x)
    }

    /// Get this object's y position
    pub fn y(&self) -> u32 {
        match_obj_variants!(self, v => v.y)
    }

    /// Get this object's id
    pub fn id(&self) -> &str {
        match_obj_variants!(self, v => &v.id)
    }

    /// Get this object's room name
    pub fn room(&self) -> RoomName {
        match_obj_variants!(self, v => v.room)
    }
}

#[cfg(test)]
mod test {
    use std::collections::hash_map::Entry::*;
    use std::collections::HashMap;

    use serde_json;

    use super::KnownRoomObject;

    #[test]
    fn parse_a_room_update_chain() {
        // This is a full bunch of messages from a real websocket stream.
        let json: serde_json::Value = serde_json::from_slice(include_bytes!("test-stream.json"))
            .expect("expected saved json file to parse successfully.");

        let array = match json {
            serde_json::Value::Array(v) => v,
            other => panic!("expected Array(), found {:?}", other),
        };

        let mut iter = array.into_iter().enumerate();

        let first_value = match iter.next().unwrap().1 {
            serde_json::Value::Object(m) => m,
            other => panic!("expected Map, found {:?}", other),
        };

        let mut id_to_thing = first_value
            .into_iter()
            .map(|(id, value)| Ok((id, serde_json::from_value(value)?)))
            .collect::<Result<HashMap<String, KnownRoomObject>, serde_json::Error>>()
            .expect("expected initial json map to parse.");

        for (update_index, update) in iter {
            let update = match update {
                serde_json::Value::Object(m) => m,
                other => panic!("expected Map, found {:?}", other),
            };

            for (id, value) in update.into_iter() {
                if value.is_null() {
                    id_to_thing.remove(&id);
                } else {
                    match id_to_thing.entry(id.clone()) {
                        Occupied(entry) => {
                            entry.into_mut().update(value).expect(&format!(
                                "expected {} in update #{} to succeed",
                                id, update_index
                            ));
                        }
                        Vacant(entry) => {
                            entry.insert(serde_json::from_value(value).expect(&format!(
                                "expected {} in update #{} to succeed",
                                id, update_index
                            )));
                        }
                    }
                }
            }
        }
    }
}
