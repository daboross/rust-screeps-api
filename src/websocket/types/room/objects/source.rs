//! `Source` data description.
use crate::data::RoomName;

with_base_fields_and_update_struct! {
    /// A source object, which creeps can gain energy by mining from.
    #[derive(Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct Source {
        /// The source's current energy - available to be mined be creeps.
        pub energy: u32,
        /// The source's maximum energy - what `energy` resets to on regeneration.
        pub energy_capacity: u32,
        /// The amount of energy either harvested on this source specifically or for the room since
        /// the last invasion (not sure which it is).
        #[serde(default)]
        pub invader_harvested: u32,
        /// The game time at which the source will next regenerate. `None` when waiting on a creep to
        /// first harvest it.
        pub next_regeneration_time: Option<u32>,
        /// The number of ticks between when a source is first harvested after regeneration and when it next
        /// regenerates.
        pub ticks_to_regeneration: u32,
    }

    /// The update structure for a source object.
    #[derive(Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct SourceUpdate {
        - energy: u32,
        - energy_capacity: u32,
        - invader_harvested: u32,
        - next_regeneration_time: Option<u32>,
        - ticks_to_regeneration: u32,
    }
}

#[cfg(test)]
mod test {
    use serde::Deserialize;
    use serde_json;

    use crate::data::RoomName;

    use super::Source;

    #[test]
    fn parse_source_and_update() {
        let json = json!({
            "_id": "57ef9dba86f108ae6e60e2fc",
            "energy": 260,
            "energyCapacity": 3000,
            "invaderHarvested": 29240,
            "nextRegenerationTime": 19894026,
            "room": "E4S61",
            "ticksToRegeneration": 300,
            "type": "source",
            "x": 26,
            "y": 9,
        });

        let mut obj = Source::deserialize(&json).unwrap();

        assert_eq!(
            obj,
            Source {
                id: "57ef9dba86f108ae6e60e2fc".to_owned(),
                room: RoomName::new("E4S61").unwrap(),
                x: 26,
                y: 9,
                energy: 260,
                energy_capacity: 3000,
                invader_harvested: 29240,
                next_regeneration_time: Some(19894026),
                ticks_to_regeneration: 300,
            }
        );

        obj.update(
            serde_json::from_value(json!({
                "x": 40,
                "y": 50,
                "energy": 0,
            }))
            .unwrap(),
        );

        assert_eq!(
            obj,
            Source {
                id: "57ef9dba86f108ae6e60e2fc".to_owned(),
                room: RoomName::new("E4S61").unwrap(),
                x: 40,
                y: 50,
                energy: 0,
                energy_capacity: 3000,
                invader_harvested: 29240,
                next_regeneration_time: Some(19894026),
                ticks_to_regeneration: 300,
            }
        );
    }

    #[test]
    fn handle_no_invader_harvested() {
        let json = json!({
            "_id": "5bbcad499099fc012e6370ab",
            "energy": 4000,
            "energyCapacity": 4000,
            "nextRegenerationTime": null,
            "room": "E6S26",
            "ticksToRegeneration": 300,
            "type": "source",
            "x": 40,
            "y": 34,
        });

        let obj = Source::deserialize(&json).unwrap();

        assert_eq!(
            obj,
            Source {
                id: "5bbcad499099fc012e6370ab".to_owned(),
                room: RoomName::new("E6S26").unwrap(),
                x: 40,
                y: 34,
                energy: 4000,
                energy_capacity: 4000,
                invader_harvested: 0,
                next_regeneration_time: None,
                ticks_to_regeneration: 300,
            }
        );
    }
}
