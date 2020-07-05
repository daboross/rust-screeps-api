//! `StructureContainer` data description.
use super::super::resources::ResourceType;
use super::super::resources::Store;
use crate::data::RoomName;

with_base_fields_and_update_struct! {
    /// A container structure - a structure which can store a small number of any combination of
    /// resources, and can be built in any room, but decays over time.
    #[derive(Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureContainer {
        /// The current number of hit-points this structure has.
        #[serde(default)]
        pub hits: i32,
        /// The maximum number of hit-points this structure has.
        #[serde(default)]
        pub hits_max: i32,
        /// Total capacity for this structure.
        #[serde(rename = "energyCapacity")]
        pub capacity: i32,
        /// The next game tick when this structure's hits will decrease naturally.
        pub next_decay_time: u32,
        /// Whether or not an attack on this structure will send an email to the owner automatically.
        pub notify_when_attacked: bool,
        /// The resources and amounts of each resource some game object holds.
        pub store: Store,
    }

    /// The update structure for an extension structure.
    #[derive(Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureContainerUpdate {
        - hits: i32,
        - hits_max: i32,
        #[serde(rename = "energyCapacity")]
        - capacity: i32,
        - next_decay_time: u32,
        - notify_when_attacked: bool,
        - store: Store,
    }
}

impl StructureContainer {
    /// Iterates over this structure's resources.
    pub fn resources(&self) -> impl Iterator<Item = (ResourceType, i32)> + '_ {
        self.store.iter()
    }
}

#[cfg(test)]
mod test {
    use serde::Deserialize;

    use super::StructureContainer;
    use crate::websocket::types::room::resources::ResourceType;

    #[test]
    fn parse_container() {
        let json = json!({
            "_id": "58cc8143050a8f701678f22e",
            "store": {
                "energy": 2000,
            },
            "energyCapacity": 2000,
            "hits": 250000,
            "hitsMax": 250000,
            "nextDecayTime": 20233841,
            "notifyWhenAttacked": true,
            "room": "E9N23",
            "type": "container",
            "x": 19,
            "y": 22
        });

        let obj = StructureContainer::deserialize(json).unwrap();

        match obj {
            StructureContainer {
                capacity: 2000,
                hits: 250000,
                hits_max: 250000,
                next_decay_time: 20233841,
                notify_when_attacked: true,
                x: 19,
                y: 22,
                ref id,
                ref store,
                ..
            } if id == "58cc8143050a8f701678f22e" && *store == store! {Energy: 2000} => (),
            other => panic!(
                "expected pre-set StructureContainer to match, found {:#?}",
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
                let mut expected = vec![(ResourceType::Energy, 2000)];
                expected.sort();
                expected
            }
        );
    }
}
