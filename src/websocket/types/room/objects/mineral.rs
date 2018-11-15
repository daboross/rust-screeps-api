//! `Mineral` data description.
use super::super::resources::ResourceType;
use data::RoomName;

with_base_fields_and_update_struct! {
    /// A mineral, an object creeps can mine for a non-energy resource.
    #[derive(Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct Mineral {
        /// The 'density' value, dictating how much of the resource is added when the mineral regenerates.
        ///
        /// Changes each regeneration.
        pub density: u8,
        /// The current amount of the resource in the mineral.
        pub mineral_amount: f64,
        /// The type of resource this mineral has.
        pub mineral_type: ResourceType,
        /// The number of game ticks until the mineral next regenerates
        /// (or None if the mineral still has any resources left).
        pub next_regeneration_time: Option<u32>,
    }

    /// The update structure for a mineral object.
    #[derive(Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct MineralUpdate { ... }
}

#[cfg(test)]
mod test {
    use serde::Deserialize;

    use data::RoomName;

    use super::{Mineral, ResourceType};

    #[test]
    fn parse_mineral() {
        let json = json!({
            "_id": "57efa010195b160f02c752d6",
            "density": 3,
            "mineralAmount": 65590,
            "mineralType": "H",
            "nextRegenerationTime": null,
            "room": "E4S61",
            "type": "mineral",
            "x": 14,
            "y": 21,
        });

        let obj = Mineral::deserialize(json).unwrap();

        assert_eq!(
            obj,
            Mineral {
                id: "57efa010195b160f02c752d6".to_owned(),
                room: RoomName::new("E4S61").unwrap(),
                x: 14,
                y: 21,
                density: 3,
                mineral_amount: 65590.0,
                mineral_type: ResourceType::Hydrogen,
                next_regeneration_time: None,
            }
        );
    }
}
