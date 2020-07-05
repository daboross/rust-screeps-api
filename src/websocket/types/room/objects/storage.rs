//! `StructureStorage` data description.
use crate::data::RoomName;
use crate::websocket::types::room::resources::ResourceType;
use crate::websocket::types::room::resources::Store;

with_base_fields_and_update_struct! {
    /// A storage structure - a structure that has a large capacity for storing multiple resources,
    /// and does not decay over time.
    #[derive(Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureStorage {
        /// The current number of hit-points this structure has.
        #[serde(default)]
        pub hits: i32,
        /// The maximum number of hit-points this structure has.
        #[serde(default)]
        pub hits_max: i32,
        /// The user ID of the owner of this structure.
        pub user: String,
        /// Whether or not this structure is non-functional due to a degraded controller.
        #[serde(default, rename = "off")]
        pub disabled: bool,
        /// Total capacity for this structure.
        #[serde(rename = "energyCapacity")]
        pub capacity: i32,
        /// Whether or not an attack on this structure will send an email to the owner automatically.
        pub notify_when_attacked: bool,
        /// The resources and amounts of each resource some game object holds.
        pub store: Store,
    }

    /// The update structure for an extension structure.
    #[derive(Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureStorageUpdate {
        - hits: i32,
        - hits_max: i32,
        - user: String,
        #[serde(rename = "off")]
        - disabled: bool,
        #[serde(rename = "storeCapacity")]
        - capacity: i32,
        - notify_when_attacked: bool,
        - store: Store,
    }
}

impl StructureStorage {
    /// Iterates over this structure's resources.
    pub fn resources(&self) -> impl Iterator<Item = (ResourceType, i32)> + '_ {
        self.store.iter()
    }
}

#[cfg(test)]
mod test {
    use serde::Deserialize;

    use super::StructureStorage;
    use crate::websocket::types::room::resources::ResourceType;

    #[test]
    fn parse_storage() {
        let json = json!({
            "GO": 0,
            "KO": 0,
            "O": 0,
            "U": 33581,
            "XGHO2": 5400,
            "XKHO2": 4249,
            "XLHO2": 2554,
            "XZH2O": 2660,
            "ZH": 0,
            "_id": "57f376603bbada4a68f0135c",
            "energy": 631112,
            "energyCapacity": 1000000,
            "hits": 10000,
            "hitsMax": 10000,
            "notifyWhenAttacked": true,
            "room": "E17N55",
            "type": "storage",
            "user": "57874d42d0ae911e3bd15bbc",
            "x": 7,
            "y": 16
        });

        let obj = StructureStorage::deserialize(json).unwrap();

        match obj {
            StructureStorage {
                ghodium_oxide: 0,
                keanium_oxide: 0,
                oxygen: 0,
                utrium: 33581,
                catalyzed_ghodium_alkalide: 5400,
                keanium: 0,
                energy: 631112,
                capacity: 1000000,
                hits: 10000,
                hits_max: 10000,
                notify_when_attacked: true,
                x: 7,
                y: 16,
                ..
            } => (),
            other => panic!(
                "expected pre-set StructureStorage to match, found {:#?}",
                other
            ),
        }

        assert_eq!(
            {
                let mut contents = obj.resources().collect::<Vec<_>>();
                contents.sort();
                contents
            },
            {
                let mut expected = vec![
                    (ResourceType::Utrium, 33581),
                    (ResourceType::CatalyzedGhodiumAlkalide, 5400),
                    (ResourceType::CatalyzedKeaniumAlkalide, 4249),
                    (ResourceType::CatalyzedLemergiumAlkalide, 2554),
                    (ResourceType::CatalyzedZynthiumAcid, 2660),
                    (ResourceType::Energy, 631112),
                ];
                expected.sort();
                expected
            }
        );
    }
}
