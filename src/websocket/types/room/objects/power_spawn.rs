//! `StructurePowerSpawn` data description.
use crate::data::RoomName;

with_structure_fields_and_update_struct! {
    /// A power spawn structure - a structure which can consume power, and in the future
    /// will be able to spawn power creeps.
    #[derive(Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct StructurePowerSpawn {
        /// The user ID of the owner of this structure.
        pub user: String,
        /// Whether or not this structure is non-functional due to a degraded controller.
        #[serde(default, rename = "off")]
        pub disabled: bool,
        /// The current amount of energy held in this structure.
        pub energy: i32,
        /// The maximum amount of energy that can be held in this structure.
        pub energy_capacity: i32,
        /// The current amount of power held in this structure.
        pub power: i32,
        /// The maximum amount of power that can be held in this structure.
        pub power_capacity: i32,
        /// Whether or not an attack on this structure will send an email to the owner automatically.
        pub notify_when_attacked: bool,
    }

    /// The update structure for a mineral object.
    #[derive(Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct StructurePowerSpawnUpdate {
        - user: String,
        #[serde(rename = "off")]
        - disabled: bool,
        - energy: i32,
        - energy_capacity: i32,
        - power: i32,
        - power_capacity: i32,
        - notify_when_attacked: bool,
    }
}

#[cfg(test)]
mod test {
    use serde::Deserialize;

    use crate::data::RoomName;

    use super::StructurePowerSpawn;

    #[test]
    fn parse_power_spawn() {
        let json = json!({
            "_id": "5825874e440f3fbd2caf30b5",
            "energy": 4944,
            "energyCapacity": 5000,
            "hits": 5000,
            "hitsMax": 5000,
            "notifyWhenAttacked": true,
            "power": 94,
            "powerCapacity": 100,
            "room": "E9N23",
            "type": "powerSpawn",
            "user": "561e4d4645f3f7244a7622e8",
            "x": 19,
            "y": 14
        });

        let obj = StructurePowerSpawn::deserialize(json).unwrap();

        assert_eq!(
            obj,
            StructurePowerSpawn {
                id: "5825874e440f3fbd2caf30b5".to_owned(),
                room: RoomName::new("E9N23").unwrap(),
                x: 19,
                y: 14,
                energy: 4944,
                energy_capacity: 5000,
                power: 94,
                power_capacity: 100,
                hits: 5000,
                hits_max: 5000,
                notify_when_attacked: true,
                disabled: false,
                user: "561e4d4645f3f7244a7622e8".to_owned(),
            }
        );
    }
}
