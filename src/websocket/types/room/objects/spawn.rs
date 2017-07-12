//! `StructureSpawn` data description.
use data::RoomName;

with_update_struct! {
    /// A struct describing a creep currently spawning (used as part of the update for a StructureSpawn).
    #[derive(Deserialize, Clone, Debug, PartialEq)]
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
    #[derive(Deserialize, Clone, Debug)]
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
        pub spawning: SpawningCreep,
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
        - spawning: SpawningCreep,
    }
}

#[cfg(test)]
mod test {
    use serde::Deserialize;

    use data::RoomName;

    use super::{StructureSpawn, SpawningCreep};

    #[test]
    fn parse_spawn() {
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

        let obj = StructureSpawn::deserialize(json).unwrap();

        assert_eq!(obj, StructureSpawn {
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
            spawning: SpawningCreep {
                name: "5599".to_owned(),
                total_time: 126,
                remaining_time: 5,
            },
            user: "57874d42d0ae911e3bd15bbc".to_owned(),
        });
    }
}
