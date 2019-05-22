//! `ConstructionSite` data description.
use crate::data::RoomName;

/// Type of structure (not general room object).
///
/// Currently only used when decoding ConstructionSites.
#[derive(Clone, Debug, PartialEq, Eq, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum StructureType {
    /// StructureSpawn structure type
    Spawn,
    /// StructureExtension structure type
    Extension,
    /// Road structure type
    Road,
    /// StructureWall structure type
    ConstructedWall,
    /// StructureRampart structure type
    Rampart,
    /// StructureKeeperLair structure type
    KeeperLair,
    /// StructurePortal structure type
    Portal,
    /// StructureController structure type
    Controller,
    /// StructureLink structure type
    Link,
    /// StructureStorage structure type
    Storage,
    /// StructureTower structure type
    Tower,
    /// StructureObserver structure type
    Observer,
    /// StructurePowerBank structure type
    PowerBank,
    /// StructurePowerSpawn structure type
    PowerSpawn,
    /// StructureExtractor structure type
    Extractor,
    /// StructureLab structure type
    Lab,
    /// StructureTerminal structure type
    Terminal,
    /// StructureContainer structure type
    Container,
    /// StructureNuker structure type
    Nuker,
}

basic_updatable!(StructureType);

with_base_fields_and_update_struct! {
    /// A construction site - a planned structure
    #[derive(Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct ConstructionSite {
        /// The user ID of the owner of the creep
        pub user: String,
        /// A name the structure will have once built (only for spawns)
        pub name: Option<String>,
        /// Progress on the construction site
        pub progress: u32,
        /// Total progress needed
        pub progress_total: u32,
        /// Structure type
        pub structure_type: StructureType,
    }

    /// The update structure for a `ConstructionSite`.
    #[derive(Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct ConstructionSiteUpdate { ... }
}

#[cfg(test)]
mod test {
    use serde::Deserialize;

    use crate::data::RoomName;

    use super::{ConstructionSite, StructureType};

    #[test]
    fn parse_simple_site() {
        let json = json!({
            "_id": "5ce54eadab500847fda00973",
            "name": null,
            "progress": 211,
            "progressTotal": 300,
            "room": "E9S32",
            "structureType": "road",
            "type": "constructionSite",
            "user": "59cec9e20dd629146b767d96",
            "x": 4,
            "y": 25,
        });

        let obj = ConstructionSite::deserialize(json).unwrap();

        assert_eq!(
            obj,
            ConstructionSite {
                id: "5ce54eadab500847fda00973".to_owned(),
                room: RoomName::new("E9S32").unwrap(),
                x: 4,
                y: 25,
                user: "59cec9e20dd629146b767d96".to_owned(),
                name: None,
                progress: 211,
                progress_total: 300,
                structure_type: StructureType::Road,
            }
        );
    }
}
