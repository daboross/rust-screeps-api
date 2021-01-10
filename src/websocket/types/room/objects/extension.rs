//! `StructureExtension` data description.
use super::super::resources::Store;
use crate::data::RoomName;

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
        pub store: Store,
        /// The maximum amount of energy that can be held in this structure.
        #[serde(rename = "storeCapacityResource")]
        pub capacity_resource: Store,
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
        - store: Store,
        - capacity_resource: Store,
        - notify_when_attacked: bool,
    }
}

#[cfg(test)]
mod test {
    use serde::Deserialize;
    use serde_json;

    use crate::data::RoomName;

    use super::StructureExtension;

    #[test]
    fn parse_extension_and_update() {
        let json = json!({
          "_id": "5bef67919e7eaa3015aadfe2",
          "type": "extension",
          "x": 21,
          "y": 24,
          "room": "E9S32",
          "notifyWhenAttacked": true,
          "user": "589f5265d25357e8253e3ee8",
          "hits": 1000,
          "hitsMax": 1000,
          "off": false,
          "store": {
            "energy": 200
          },
          "storeCapacityResource": {
            "energy": 200
          }
        });

        let mut obj = StructureExtension::deserialize(json).unwrap();

        assert_eq!(
            obj,
            StructureExtension {
                room: RoomName::new("E9S32").unwrap(),
                x: 21,
                y: 24,
                id: "5bef67919e7eaa3015aadfe2".to_owned(),
                store: store! { Energy: 200 },
                capacity_resource: store! { Energy: 200 },
                hits: 1000,
                hits_max: 1000,
                notify_when_attacked: true,
                disabled: false,
                user: "589f5265d25357e8253e3ee8".to_owned(),
            }
        );

        obj.update(
            serde_json::from_value(json!({
                "store": {
                    "energy": 0,
                },
                "notifyWhenAttacked": false,
            }))
            .unwrap(),
        );

        assert_eq!(
            obj,
            StructureExtension {
                room: RoomName::new("E9S32").unwrap(),
                x: 21,
                y: 24,
                id: "5bef67919e7eaa3015aadfe2".to_owned(),
                store: store! {},
                capacity_resource: store! { Energy: 200 },
                hits: 1000,
                hits_max: 1000,
                notify_when_attacked: false,
                disabled: false,
                user: "589f5265d25357e8253e3ee8".to_owned(),
            }
        );
    }
}
