//! `Tombstone` data description.
use crate::data::RoomName;
use super::super::resources::ResourceType;
use super::super::resources::Store;

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
    pub fn resources(&self) -> impl Iterator<Item=(ResourceType, i32)> + '_ {
        self.store.iter()
    }
}

#[cfg(test)]
mod test {
    use serde::Deserialize;

    use super::{ResourceType, Tombstone};

    #[test]
    fn parse_simple_tombstone() {
        let json = json!({
            "_id": "5ce0a11dcd8deb5a978cb216",
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
                "move",
                "move",
                "move",
                "move",
                "move"
            ],
            "creepId": "5ce08e9f184f823e92616faf",
            "creepName": "HeavyConsolidator6921549",
            "creepSaying": null,
            "creepTicksToLive": 1,
            "deathTime": 6923092,
            "decayTime": 6923167,
            "room": "E8S29",
            "type": "tombstone",
            "user": "5ca80c8f3c33e30c8e85555d",
            "x": 30,
            "y": 31
        });

        let obj = Tombstone::deserialize(json).unwrap();

        assert_eq!(obj.creep_id, "5ce08e9f184f823e92616faf");
        assert_eq!(obj.creep_name, "HeavyConsolidator6921549");
    }

    #[test]
    fn parse_tombstone_with_energy() {
        let json = json!({
            "_id": "5ce0a4839c5bd560bebb03e9",
            "creepBody": [
                "move",
                "move",
                "carry",
                "work"
            ],
            "creepId": "5ce098c5c4e055067fbf182e",
            "creepName": "6922398-0",
            "creepSaying": null,
            "creepTicksToLive": 532,
            "deathTime": 6923377,
            "decayTime": 6923397,
            "energy": 44,
            "room": "E9S31",
            "type": "tombstone",
            "user": "57874d42d0ae911e3bd15bbc",
            "x": 33,
            "y": 23
        });

        let obj = Tombstone::deserialize(json).unwrap();

        assert_eq!(obj.creep_id, "5ce098c5c4e055067fbf182e");
        assert_eq!(obj.creep_name, "6922398-0");
        assert_eq!(obj.store.get(ResourceType::Energy), 44);
    }
}
