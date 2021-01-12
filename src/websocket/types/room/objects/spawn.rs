//! `StructureSpawn` data description.
use super::super::resources::Store;
use crate::data::RoomName;

with_update_struct! {
    /// A struct describing a creep currently spawning (used as part of the update for a StructureSpawn).
    #[derive(serde_derive::Deserialize, Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct SpawningCreep {
        /// The name of this creep, unique per player.
        pub name: String,
        /// The total number of game ticks needed to spawn this creep.
        #[serde(rename = "needTime")]
        pub total_time: u32,
        /// The game tick on which the creep will be spawned.
        pub spawn_time: u32,
    }

    /// The update structure for a spawning creep.
    #[derive(serde_derive::Deserialize, Clone, Debug)]
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
        /// The current amount of resources held in this structure.
        pub store: Store,
        /// The maximum amount of resources that can be held in this structure.
        #[serde(rename = "storeCapacityResource")]
        pub capacity_resource: Store,
        /// Whether or not an attack on this structure will send an email to the owner automatically.
        pub notify_when_attacked: bool,
        /// The name of this spawn, unique per player.
        pub name: String,
        /// The creep that's currently spawning, if any.
        pub spawning: Option<SpawningCreep>,
    }

    /// The update structure for a mineral object.
    #[derive(Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureSpawnUpdate {
        - user: String,
        #[serde(rename = "off")]
        - disabled: bool,
        - store: Store,
        #[serde(rename = "storeCapacityResource")]
        - capacity_resource: Store,
        - notify_when_attacked: bool,
        - name: String,
        - spawning: Option<SpawningCreep>,
    }
}

#[cfg(test)]
mod test {
    use serde_json;

    use serde::Deserialize;

    use crate::data::RoomName;

    use super::{SpawningCreep, StructureSpawn};

    #[test]
    fn parse_empty_spawn() {
        let json = json!({
            "_id": "5f0236153187fd5e3dfa814a",
            "hits": 5000,
            "hitsMax": 5000,
            "name": "Spawn1",
            "notifyWhenAttacked": true,
            "off": false,
            "room": "W41N48",
            "spawning": null,
            "store": {
                "energy": 300
            },
            "storeCapacityResource": {
                "energy": 300
            },
            "type": "spawn",
            "user": "57874d42d0ae911e3bd15bbc",
            "x": 26,
            "y": 28
        });

        let obj = StructureSpawn::deserialize(json).unwrap();

        assert_eq!(
            obj,
            StructureSpawn {
                id: "5f0236153187fd5e3dfa814a".to_owned(),
                room: RoomName::new("W41N48").unwrap(),
                x: 26,
                y: 28,
                store: store! { Energy: 300 },
                capacity_resource: store! { Energy: 300 },
                hits: 5000,
                hits_max: 5000,
                name: "Spawn1".to_owned(),
                notify_when_attacked: true,
                disabled: false,
                spawning: None,
                user: "57874d42d0ae911e3bd15bbc".to_owned(),
            }
        );
    }

    #[test]
    fn parse_spawn_and_update() {
        let json = json!({
          "_id": "5d25a0b8e52ab5700a3747ab",
          "type": "spawn",
          "room": "W44S12",
          "x": 28,
          "y": 26,
          "name": "Spawn1",
          "user": "5a8466038f866773f59fa6c8",
          "hits": 5000,
          "hitsMax": 5000,
          "spawning": {
            "name": "902640",
            "needTime": 144,
            "spawnTime": 24577555
          },
          "notifyWhenAttacked": true,
          "off": false,
          "store": {
            "energy": 300
          },
          "storeCapacityResource": {
            "energy": 300
          }
        });

        let mut obj = StructureSpawn::deserialize(json).unwrap();

        assert_eq!(
            obj,
            StructureSpawn {
                id: "5d25a0b8e52ab5700a3747ab".to_owned(),
                room: RoomName::new("W44S12").unwrap(),
                x: 28,
                y: 26,
                store: store! { Energy: 300 },
                capacity_resource: store! { Energy: 300 },
                hits: 5000,
                hits_max: 5000,
                name: "Spawn1".to_owned(),
                notify_when_attacked: true,
                disabled: false,
                spawning: Some(SpawningCreep {
                    name: "902640".to_owned(),
                    total_time: 144,
                    spawn_time: 24577555,
                }),
                user: "5a8466038f866773f59fa6c8".to_owned(),
            }
        );

        obj.update(serde_json::from_value(json!({ "spawning": null })).unwrap());

        assert_eq!(obj.spawning, None);

        obj.update(
            serde_json::from_value(json!({
              "spawning": {
                "name": "8449040",
                "needTime": 144,
                "spawnTime": 24577699
              },
              "store": {
                "energy": 0
              }
            }))
            .unwrap(),
        );

        assert_eq!(
            obj.spawning,
            Some(SpawningCreep {
                name: "8449040".to_owned(),
                total_time: 144,
                spawn_time: 24577699,
            })
        );

        assert_eq!(obj.store, store! {});

        obj.update(
            serde_json::from_value(json!({
              "store": {
                "energy": 300
              }
            }))
            .unwrap(),
        );

        assert_eq!(obj.store, store! { Energy: 300 });
    }
}
