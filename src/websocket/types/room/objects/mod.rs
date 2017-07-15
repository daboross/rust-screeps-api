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
