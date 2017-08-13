//! Room object parsing.
//!
//! If you just want to use the module, reading the rustdocs documentation is very recommended.
//! All types generated with macros will also have documentation for them available.
//!
//! Reading the source code is definitely possible. But there may be some investment in reading
//! each of the macros defined and used here, and it will be much easier to just read the documentation.

pub mod shared;
pub mod source;
pub mod mineral;
pub mod spawn;
pub mod extension;
pub mod wall;
pub mod road;
pub mod rampart;
pub mod keeper_lair;
pub mod controller;
pub mod portal;
pub mod link;
pub mod storage;
pub mod tower;
pub mod observer;
pub mod power_bank;
pub mod power_spawn;
pub mod lab;
pub mod terminal;
pub mod container;
pub mod nuker;
pub mod creep;
pub mod resource;

use self::shared::ActionLogTarget;
pub use self::source::{Source, SourceUpdate};
pub use self::mineral::{Mineral, MineralUpdate};
pub use self::spawn::{StructureSpawn, StructureSpawnUpdate};
pub use self::extension::{StructureExtension, StructureExtensionUpdate};
pub use self::wall::{StructureWall, StructureWallUpdate};
pub use self::road::{StructureRoad, StructureRoadUpdate};
pub use self::rampart::{StructureRampart, StructureRampartUpdate};
pub use self::keeper_lair::{StructureKeeperLair, StructureKeeperLairUpdate};
pub use self::controller::{StructureController, StructureControllerUpdate};
pub use self::portal::{StructurePortal, StructurePortalUpdate};
pub use self::link::{StructureLink, StructureLinkUpdate};
pub use self::storage::{StructureStorage, StructureStorageUpdate};
pub use self::tower::{StructureTower, StructureTowerUpdate};
pub use self::observer::{StructureObserver, StructureObserverUpdate};
pub use self::power_bank::{StructurePowerBank, StructurePowerBankUpdate};
pub use self::power_spawn::{StructurePowerSpawn, StructurePowerSpawnUpdate};
pub use self::lab::{StructureLab, StructureLabUpdate};
pub use self::terminal::{StructureTerminal, StructureTerminalUpdate};
pub use self::container::{StructureContainer, StructureContainerUpdate};
pub use self::nuker::{StructureNuker, StructureNukerUpdate};
pub use self::creep::{Creep, CreepUpdate};
pub use self::resource::{Resource, ResourceUpdate};

use serde_json;

/// Enum describing all known room objects.
#[derive(Deserialize, Clone, Debug, PartialEq)]
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
    /// Lab owned structure.
    Lab(StructureLab),
    /// Terminal owned structure.
    Terminal(StructureTerminal),
    /// Container unowned structure.
    Container(StructureContainer),
    /// Nuker owned structure.
    Nuker(StructureNuker),
    /// Creep owned creature.
    Creep(Creep),
    /// Resource dropped object.
    #[serde(rename = "energy")]
    Resource(Resource),
}

impl KnownRoomObject {
    /// Update this room object with a JSON update string.
    pub fn update(&mut self, input: serde_json::Value) -> Result<(), serde_json::Error> {
        use self::KnownRoomObject::*;

        macro_rules! large_match {
            (
                $(
                    $enum_name:ident
                ),*
            ) => (
                match *self {
                    $(
                        $enum_name(ref mut value) => value.update(serde_json::from_value(input)?),
                    )*
                }
            )
        }

        large_match!(
            Source,
            Mineral,
            Spawn,
            Extension,
            Wall,
            Road,
            Rampart,
            KeeperLair,
            Controller,
            Portal,
            Link,
            Storage,
            Tower,
            Observer,
            PowerBank,
            PowerSpawn,
            Lab,
            Terminal,
            Container,
            Nuker,
            Creep,
            Resource
        );

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use std::collections::hash_map::Entry::*;

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
                            entry
                                .into_mut()
                                .update(value)
                                .expect(&format!("expected {} in update #{} to succeed", id, update_index));
                        }
                        Vacant(entry) => {
                            entry.insert(
                                serde_json::from_value(value)
                                    .expect(&format!("expected {} in update #{} to succeed", id, update_index)),
                            );
                        }
                    }
                }
            }
        }
    }
}
