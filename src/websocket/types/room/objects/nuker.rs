//! `StructureNuker` data description.
use super::super::resources::Store;
use crate::data::RoomName;

with_structure_fields_and_update_struct! {
    /// An nuker structure - a structure which can be loaded with energy and ghodium, and then
    /// fired to launch a large impact missile into another nearby room.
    #[derive(Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureNuker {
        /// The user ID of the owner of this structure.
        pub user: String,
        /// Whether or not this structure is non-functional due to a degraded controller.
        #[serde(default, rename = "off")]
        pub disabled: bool,
        /// The current amount of energy and ghodium held in this structure.
        pub store: Store,
        /// The maximum amount of energy and ghodium that can be held in this structure.
        pub store_capacity_resource: Store,
        /// The game time at which this nuker will next be able to launch a missile.
        pub cooldown_time: u32,
        /// Whether or not an attack on this structure will send an email to the owner automatically.
        pub notify_when_attacked: bool,
    }

    /// The update structure for a `StructureNuker`.
    #[derive(Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureNukerUpdate {
        - user: String,
        #[serde(rename = "off")]
        - disabled: bool,
        - store: Store,
        - store_capacity_resource: Store,
        - cooldown_time: u32,
        - notify_when_attacked: bool,
    }
}

#[cfg(test)]
mod test {
    use serde::Deserialize;

    use crate::data::RoomName;

    use super::StructureNuker;

    #[test]
    fn parse_nuker() {
        let json = json!({
          "_id": "5df45e27d1ba60a873688d18",
          "type": "nuker",
          "x": 25,
          "y": 19,
          "room": "E17S38",
          "notifyWhenAttacked": false,
          "user": "589f5265d25357e8253e3ee8",
          "store": {
            "energy": 300000,
            "G": 5000
          },
          "storeCapacityResource": {
            "energy": 300000,
            "G": 5000
          },
          "hits": 1000,
          "hitsMax": 1000,
          "cooldownTime": 30260100
        });

        let obj = StructureNuker::deserialize(json).unwrap();

        assert_eq!(
            obj,
            StructureNuker {
                room: RoomName::new("E17S38").unwrap(),
                x: 25,
                y: 19,
                id: "5df45e27d1ba60a873688d18".to_owned(),
                hits: 1000,
                hits_max: 1000,
                store: store! {
                    Energy: 300000,
                    Ghodium: 5000
                },
                store_capacity_resource: store! {
                    Energy: 300000,
                    Ghodium: 5000
                },
                cooldown_time: 30260100,
                notify_when_attacked: false,
                disabled: false,
                user: "589f5265d25357e8253e3ee8".to_owned(),
            }
        );
    }
}
