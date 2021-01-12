//! `Tombstone` data description.
use super::super::resources::ResourceType;
use super::super::resources::Store;
use crate::data::RoomName;

use super::creep::CreepPartType;

with_base_fields_and_update_struct! {
    /// A tomstone - remnants of a dead creep
    #[derive(Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct Tombstone {
        /// The user ID of the owner of the creep
        pub user: String,
        /// The body of the creep who died
        pub creep_body: Vec<CreepPartType>,
        /// The ID of the creep who died
        pub creep_id: String,
        /// The name of the creep who died
        pub creep_name: String,
        /// What the creep was saying when they died
        pub creep_saying: Option<String>,
        /// How many ticks-to-live did the creeps have when it died
        pub creep_ticks_to_live: i32,
        /// The time of death of the creep
        pub death_time: u32,
        /// The time at which this structure will decay
        pub decay_time: u32,
        /// The resources and amounts of each resource some game object holds.
        pub store: Store,
        // TODO: what does the tombstone of a power creep look like?
    }

    /// The update structure for a `Tombstone`.
    #[derive(Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct TombstoneUpdate { ... }
}

impl Tombstone {
    /// Iterates over this structure's resources.
    pub fn resources(&self) -> impl Iterator<Item = (ResourceType, i32)> + '_ {
        self.store.iter()
    }
}

#[cfg(test)]
mod test {
    use serde::Deserialize;

    use super::Tombstone;

    #[test]
    fn parse_simple_tombstone() {
        let json = json!({
            "_id": "5f025c6070eb1e39990f1622",
            "creepBody": [
                "move",
                "move",
                "carry",
                "work"
            ],
            "creepId": "5f025b3706ec7d5f6e3cd7a3",
            "creepName": "38445073-0",
            "creepSaying": null,
            "creepTicksToLive": 1457,
            "deathTime": 38445127,
            "decayTime": 38445147,
            "room": "W41N48",
            "store": {
                "energy": 48
            },
            "type": "tombstone",
            "user": "57874d42d0ae911e3bd15bbc",
            "x": 12,
            "y": 24
        });

        let obj = Tombstone::deserialize(json).unwrap();

        assert_eq!(obj.creep_id, "5f025b3706ec7d5f6e3cd7a3");
        assert_eq!(obj.creep_name, "38445073-0");
    }

    #[test]
    fn parse_tombstone_with_energy() {
        let json = json!({
            "_id": "5f023c25557c806c615bf76d",
            "creepBody": [
                "carry",
                "carry",
                "carry",
                "carry",
                "carry",
                "carry",
                "carry",
                "carry",
                "carry",
                "carry",
                "carry",
                "carry",
                "carry",
                "carry",
                "carry",
                "carry",
                "carry",
                "carry",
                "carry",
                "carry",
                "move",
                "move",
                "move",
                "move",
                "move",
                "move",
                "move",
                "move",
                "move",
                "move"
            ],
            "creepId": "5f021a8c0a816c111ff81006",
            "creepName": "W31N48_transport_20200",
            "creepSaying": null,
            "creepTicksToLive": 1,
            "deathTime": 38443608,
            "decayTime": 38443758,
            "room": "W31N48",
            "store": {
                "energy": 1000
            },
            "type": "tombstone",
            "user": "57cd3be0559868c84d297d87",
            "x": 20,
            "y": 33
        });

        let obj = Tombstone::deserialize(json).unwrap();

        assert_eq!(obj.creep_id, "5f021a8c0a816c111ff81006");
        assert_eq!(obj.creep_name, "W31N48_transport_20200");
        assert_eq!(obj.store, store! { Energy: 1000 });
    }
}
