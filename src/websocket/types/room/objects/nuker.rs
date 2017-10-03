//! `StructureNuker` data description.
use data::RoomName;

with_structure_fields_and_update_struct! {
    /// An nuker structure - a structure which can be loaded with energy and ghodium, and then
    /// fired to launch a large impact missile into another nearby room.
    #[derive(Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureNuker {
        /// The user ID of the owner of this structure.
        pub user: String,
        /// Whether or not this structure is non-functional due to a degraded controller.
        #[serde(default, rename = "off")]
        pub disabled: bool,
        /// The current amount of energy held in this structure.
        pub energy: i32,
        /// The maximum amount of energy that can be held in this structure.
        pub energy_capacity: i32,
        /// The current amount of ghodium held in this structure.
        #[serde(rename = "G")]
        pub ghodium: i32,
        /// The maximum amount of ghodium that can be held in this structure.
        #[serde(rename = "GCapacity")]
        pub ghodium_capacity: i32,
        /// The game time at which this nuker will next be able to launch a missile.
        pub cooldown_time: u32,
        /// Whether or not an attack on this structure will send an email to the owner automatically.
        pub notify_when_attacked: bool,
    }

    /// The update structure for a `StructureNuker`.
    #[derive(Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureNukerUpdate {
        - user: String,
        #[serde(rename = "off")]
        - disabled: bool,
        - energy: i32,
        - energy_capacity: i32,
        #[serde(rename = "G")]
        - ghodium: i32,
        #[serde(rename = "GCapacity")]
        - ghodium_capacity: i32,
        - cooldown_time: u32,
        - notify_when_attacked: bool,
    }
}

#[cfg(test)]
mod test {
    use serde::Deserialize;

    use data::RoomName;

    use super::StructureNuker;

    #[test]
    fn parse_nuker() {
        let json = json!({
            "G": 5000,
            "GCapacity": 5000,
            "_id": "582582913105cae9690e9cb6",
            "cooldownTime": 19516631,
            "energy": 300000,
            "energyCapacity": 300000,
            "hits": 1000,
            "hitsMax": 1000,
            "notifyWhenAttacked": true,
            "room": "E9N23",
            "type": "nuker",
            "user": "561e4d4645f3f7244a7622e8",
            "x": 19,
            "y": 13
        });

        let obj = StructureNuker::deserialize(json).unwrap();

        assert_eq!(
            obj,
            StructureNuker {
                room: RoomName::new("E9N23").unwrap(),
                x: 19,
                y: 13,
                id: "582582913105cae9690e9cb6".to_owned(),
                hits: 1000,
                hits_max: 1000,
                energy: 300000,
                energy_capacity: 300000,
                ghodium: 5000,
                ghodium_capacity: 5000,
                cooldown_time: 19516631,
                notify_when_attacked: true,
                disabled: false,
                user: "561e4d4645f3f7244a7622e8".to_owned(),
            }
        );
    }
}
