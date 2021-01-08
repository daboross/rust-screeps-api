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
        #[serde(rename = "storeCapacity")]
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
        #[serde(rename = "storeCapacity")]
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
          "_id": "5c229f613f9ca9206752338c",
          "type": "container",
          "x": 20,
          "y": 24,
          "room": "E9S32",
          "notifyWhenAttacked": true,
          "hits": 230000,
          "hitsMax": 250000,
          "nextDecayTime": 30246717,
          "store": {
            "energy": 447,
            "XLHO2": 0,
            "Z": 5,
            "XGHO2": 0,
            "XUH2O": 0,
            "XZHO2": 0,
            "GO": 0,
            "KO": 0,
            "ZH": 0,
            "UH": 0,
            "UO": 0
          },
          "storeCapacity": 2000
        });

        let obj = StructureContainer::deserialize(json).unwrap();

        match obj {
            StructureContainer {
                capacity: 2000,
                hits: 230000,
                hits_max: 250000,
                next_decay_time: 30246717,
                notify_when_attacked: true,
                x: 20,
                y: 24,
                ref id,
                ref store,
                ..
            } if id == "5c229f613f9ca9206752338c" && *store == store! {Energy: 447, Zynthium: 5} => (),
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
                let mut expected = vec![(ResourceType::Energy, 447), (ResourceType::Zynthium, 5)];
                expected.sort();
                expected
            }
        );
    }
}
