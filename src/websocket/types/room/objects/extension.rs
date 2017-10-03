//! `StructureExtension` data description.
use data::RoomName;

with_structure_fields_and_update_struct! {
    /// An extension structure - a structure that can be filled with extra energy spawns can use.
    #[derive(Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureExtension {
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
    }

    /// The update structure for an extension structure.
    #[derive(Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureExtensionUpdate {
        - user: String,
        #[serde(rename = "off")]
        - disabled: bool,
        - energy: i32,
        - energy_capacity: i32,
        - notify_when_attacked: bool,
    }
}

#[cfg(test)]
mod test {
    use serde_json;
    use serde::Deserialize;

    use data::RoomName;

    use super::StructureExtension;

    #[test]
    fn parse_extension_and_update() {
        let json = json!({
            "_id": "594cac66e1dd5c8d2eb7df9d",
            "energy": 200,
            "energyCapacity": 200,
            "hits": 1000,
            "hitsMax": 1000,
            "notifyWhenAttacked": true,
            "off": false,
            "room": "E4S61",
            "type": "extension",
            "user": "57874d42d0ae911e3bd15bbc",
            "x": 27,
            "y": 3,
        });

        let mut obj = StructureExtension::deserialize(json).unwrap();

        assert_eq!(
            obj,
            StructureExtension {
                room: RoomName::new("E4S61").unwrap(),
                x: 27,
                y: 3,
                id: "594cac66e1dd5c8d2eb7df9d".to_owned(),
                energy: 200,
                energy_capacity: 200,
                hits: 1000,
                hits_max: 1000,
                notify_when_attacked: true,
                disabled: false,
                user: "57874d42d0ae911e3bd15bbc".to_owned(),
            }
        );

        obj.update(
            serde_json::from_value(json!({
            "energy": 0,
            "notifyWhenAttacked": false,
        })).unwrap(),
        );

        assert_eq!(
            obj,
            StructureExtension {
                room: RoomName::new("E4S61").unwrap(),
                x: 27,
                y: 3,
                id: "594cac66e1dd5c8d2eb7df9d".to_owned(),
                energy: 0,
                energy_capacity: 200,
                hits: 1000,
                hits_max: 1000,
                notify_when_attacked: false,
                disabled: false,
                user: "57874d42d0ae911e3bd15bbc".to_owned(),
            }
        );
    }
}
