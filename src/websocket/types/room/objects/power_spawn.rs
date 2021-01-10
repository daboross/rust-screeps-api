//! `StructurePowerSpawn` data description.
use super::super::resources::Store;
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
        /// The current amount of energy and power held in this structure.
        pub store: Store,
        /// The maximum amount of energy and power that can be held in this structure.
        #[serde(rename = "storeCapacityResource")]
        pub capacity_resource: Store,
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
        - store: Store,
        #[serde(rename = "storeCapacityResource")]
        - capacity_resource: Store,
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
          "_id": "5c4f9dfdad370f697474ed5b",
          "type": "powerSpawn",
          "x": 33,
          "y": 24,
          "room": "W15S43",
          "notifyWhenAttacked": true,
          "user": "589f5265d25357e8253e3ee8",
          "hits": 5000,
          "hitsMax": 5000,
          "store": {
            "energy": 369,
            "power": 27
          },
          "storeCapacityResource": {
            "energy": 5000,
            "power": 100
          }
        });

        let obj = StructurePowerSpawn::deserialize(json).unwrap();

        assert_eq!(
            obj,
            StructurePowerSpawn {
                id: "5c4f9dfdad370f697474ed5b".to_owned(),
                room: RoomName::new("W15S43").unwrap(),
                x: 33,
                y: 24,
                store: store! {
                    Energy: 369,
                    Power: 27
                },
                capacity_resource: store! {
                    Energy: 5000,
                    Power: 100
                },
                hits: 5000,
                hits_max: 5000,
                notify_when_attacked: true,
                disabled: false,
                user: "589f5265d25357e8253e3ee8".to_owned(),
            }
        );
    }
}
