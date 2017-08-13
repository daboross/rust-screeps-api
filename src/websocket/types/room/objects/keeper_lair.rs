//! `StructureKeeperLair` data description.
use data::RoomName;

with_base_fields_and_update_struct! {
    /// A keeper lair - a structure which spawns npc hostiles to protect minerals and resources nearby.
    #[derive(Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureKeeperLair {
        /// The next game time at which this structure will spawn an NPC.
        ///
        /// If an NPC this structure spawned is currently alive, the structure waits until that NPC dies
        /// before starting the timer. This is what a null/None value indicates.
        pub next_spawn_time: Option<u32>,
    }

    /// The update structure for a keeper lair structure.
    #[derive(Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureKeeperLairUpdate { ... }
}

#[cfg(test)]
mod test {
    use serde_json;
    use serde::Deserialize;

    use data::RoomName;

    use super::StructureKeeperLair;

    #[test]
    fn parse_keeper_lair_spawned_and_update() {
        let json = json!({
            "_id": "55c34a6b5be41a0a6e80c325",
            // Guessing this is a legacy value that's in the database from when this was the game tick time.
            //
            // This only appears on objects in rooms near the center of the map (the oldest part).
            "_updated": 13052933,
            "nextSpawnTime": null,
            "room": "E4N4",
            "type": "keeperLair",
            "x": 16,
            "y": 1
        });

        let mut obj = StructureKeeperLair::deserialize(json).unwrap();

        assert_eq!(obj, StructureKeeperLair {
            room: RoomName::new("E4N4").unwrap(),
            x: 16,
            y: 1,
            id: "55c34a6b5be41a0a6e80c325".to_owned(),
            next_spawn_time: None,
        });

        obj.update(
            serde_json::from_value(json!({
            "nextSpawnTime": 20000000,
        })).unwrap(),
        );

        assert_eq!(obj, StructureKeeperLair {
            room: RoomName::new("E4N4").unwrap(),
            x: 16,
            y: 1,
            id: "55c34a6b5be41a0a6e80c325".to_owned(),
            next_spawn_time: Some(20000000),
        });

    }

    #[test]
    fn parse_keeper_lair_spawning_and_update() {
        let json = json!({
            "_id": "55c34a6b5be41a0a6e80c6ce",
            "_updated": 13052933,
            "nextSpawnTime": 20184048,
            "room": "W14S5",
            "type": "keeperLair",
            "x": 22,
            "y": 13
        });

        let mut obj = StructureKeeperLair::deserialize(json).unwrap();

        assert_eq!(obj, StructureKeeperLair {
            room: RoomName::new("W14S5").unwrap(),
            x: 22,
            y: 13,
            id: "55c34a6b5be41a0a6e80c6ce".to_owned(),
            next_spawn_time: Some(20184048),
        });

        obj.update(
            serde_json::from_value(json!({
            "nextSpawnTime": null,
        })).unwrap(),
        );

        assert_eq!(obj, StructureKeeperLair {
            room: RoomName::new("W14S5").unwrap(),
            x: 22,
            y: 13,
            id: "55c34a6b5be41a0a6e80c6ce".to_owned(),
            next_spawn_time: None,
        });
    }
}
