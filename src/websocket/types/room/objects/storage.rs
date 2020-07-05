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
        #[serde(rename = "storeCapacity")]
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

    #[test]
    fn parse_storage() {
        let json = json!({
            "_id": "599ca7f2e48c09254b443791",
            "hits": 10000,
            "hitsMax": 10000,
            "notifyWhenAttacked": true,
            "room": "W39N49",
            "store": {
                "energy": 913026
            },
            "storeCapacity": 1000000,
            "type": "storage",
            "user": "5788389e3fd9069e6b546e2d",
            "x": 6,
            "y": 13
        });

        let obj = StructureStorage::deserialize(json).unwrap();

        match obj {
            StructureStorage {
                capacity: 1000000,
                hits: 10000,
                hits_max: 10000,
                notify_when_attacked: true,
                x: 6,
                y: 13,
                ref store,
                ..
            } if *store
                == store! { Energy: 913026 } =>
            {
                ()
            }
            other => panic!(
                "expected pre-set StructureStorage to match, found {:#?}",
                other
            ),
        }
    }
}
