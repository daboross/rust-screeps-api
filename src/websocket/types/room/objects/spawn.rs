//! `StructureSpawn` data description.
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
        /// The number of game ticks left before this creep is spawned.
        pub remaining_time: u32,
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
        /// The current amount of energy held in this structure.
        pub energy: i32,
        /// The maximum amount of energy that can be held in this structure.
        pub energy_capacity: i32,
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
        - energy: i32,
        - energy_capacity: i32,
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
                room: RoomName::new("W31N48").unwrap(),
                x: 26,
                y: 28,
                energy: 300,
                energy_capacity: 300,
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
            "_id": "58a23b6c4370e6302d758099",
            "energy": 300,
            "energyCapacity": 300,
            "hits": 5000,
            "hitsMax": 5000,
            "name": "Spawn36",
            "notifyWhenAttacked": true,
            "off": false,
            "room": "E4S61",
            "spawning": {
                "name": "5599",
                "needTime": 126,
                "remainingTime": 5,
            },
            "type": "spawn",
            "user": "57874d42d0ae911e3bd15bbc",
            "x": 24,
            "y": 6,
        });

        let mut obj = StructureSpawn::deserialize(json).unwrap();

        assert_eq!(
            obj,
            StructureSpawn {
                id: "58a23b6c4370e6302d758099".to_owned(),
                room: RoomName::new("E4S61").unwrap(),
                x: 24,
                y: 6,
                energy: 300,
                energy_capacity: 300,
                hits: 5000,
                hits_max: 5000,
                name: "Spawn36".to_owned(),
                notify_when_attacked: true,
                disabled: false,
                spawning: Some(SpawningCreep {
                    name: "5599".to_owned(),
                    total_time: 126,
                    remaining_time: 5,
                }),
                user: "57874d42d0ae911e3bd15bbc".to_owned(),
            }
        );

        obj.update(
            serde_json::from_value(json!({
                "spawning": {
                    "remainingTime": 4,
                },
            }))
            .unwrap(),
        );

        obj.update(
            serde_json::from_value(json!({
                "spawning": {
                    "remainingTime": 3,
                },
            }))
            .unwrap(),
        );

        obj.update(
            serde_json::from_value(json!({
                "spawning": {
                    "remainingTime": 2,
                },
            }))
            .unwrap(),
        );

        obj.update(
            serde_json::from_value(json!({
                "spawning": {
                    "remainingTime": 1,
                },
            }))
            .unwrap(),
        );

        assert_eq!(
            obj.spawning,
            Some(SpawningCreep {
                name: "5599".to_owned(),
                total_time: 126,
                remaining_time: 1,
            })
        );

        obj.update(
            serde_json::from_value(json!({
                "spawning": null,
            }))
            .unwrap(),
        );

        assert_eq!(obj.spawning, None);
    }
}
