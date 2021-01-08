//! `StructureLink` data description.
use super::super::resources::Store;
use super::ActionLogTarget;
use crate::data::RoomName;

with_structure_fields_and_update_struct! {
    /// A link structure - a structure that can be filled with energy, then instantly send energy to other links
    /// in the same room.
    #[derive(Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureLink {
        /// The user ID of the owner of this structure.
        pub user: String,
        /// Whether or not this structure is non-functional due to a degraded controller.
        #[serde(default, rename = "off")]
        pub disabled: bool,
        /// The resources and amounts of each resource some game object holds.
        pub store: Store,
        /// The maximum amount of each resource that can be held in this structure.
        #[serde(rename = "storeCapacityResource")]
        pub capacity_resource: Store,
        /// The number of ticks till this link can be used to send energy again.
        pub cooldown: i32,
        /// A record of all actions this structure performed last tick.
        pub action_log: StructureLinkActions,
        /// Whether or not an attack on this structure will send an email to the owner automatically.
        pub notify_when_attacked: bool,
    }

    /// The update structure for a `StructureLink`.
    #[derive(Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureLinkUpdate {
        - user: String,
        #[serde(rename = "off")]
        - disabled: bool,
        - store: Store,
        - capacity_resource: Store,
        - cooldown: i32,
        - action_log: StructureLinkActions,
        - notify_when_attacked: bool,
    }
}

with_update_struct! {
    /// A struct describing a link's actions.
    #[derive(serde_derive::Deserialize, Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureLinkActions {
        /// The x,y position the link last transfered energy to.
        pub transfer_energy: Option<ActionLogTarget>,
    }

    /// The update structure for StructureLinkActions.
    #[derive(serde_derive::Deserialize, Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureLinkActionsUpdate { ... }
}

#[cfg(test)]
mod test {
    use serde::Deserialize;
    use serde_json;

    use crate::data::RoomName;

    use super::{ActionLogTarget, StructureLink, StructureLinkActions};

    #[test]
    fn parse_link_and_updates() {
        let json = json!({
            "_id": "57fdb3ea3dad49a17265ecea",
            "actionLog": {
                "transferEnergy": null
            },
            "cooldown": 3,
            "store": {
                "energy": 100
            },
            "storeCapacityResource": {
                "energy": 800,
            },
            "hits": 1000,
            "hitsMax": 1000,
            "notifyWhenAttacked": true,
            "room": "E17N55",
            "type": "link",
            "user": "57874d42d0ae911e3bd15bbc",
            "x": 9,
            "y": 6
        });

        let mut obj = StructureLink::deserialize(json).unwrap();

        assert_eq!(
            obj,
            StructureLink {
                room: RoomName::new("E17N55").unwrap(),
                x: 9,
                y: 6,
                id: "57fdb3ea3dad49a17265ecea".to_owned(),
                store: store! { Energy: 100 },
                capacity_resource: store! { Energy: 800 },
                hits: 1000,
                hits_max: 1000,
                notify_when_attacked: true,
                disabled: false,
                cooldown: 3,
                action_log: StructureLinkActions {
                    transfer_energy: None,
                },
                user: "57874d42d0ae911e3bd15bbc".to_owned(),
            }
        );

        obj.update(
            serde_json::from_value(json!({
                "cooldown": 2
            }))
            .unwrap(),
        );

        obj.update(
            serde_json::from_value(json!({
                "cooldown": 1
            }))
            .unwrap(),
        );

        obj.update(
            serde_json::from_value(json!({
                "cooldown": 0
            }))
            .unwrap(),
        );

        assert_eq!(obj.cooldown, 0);

        obj.update(
            serde_json::from_value(json!({
                "actionLog": {
                    "transferEnergy": {
                        "x": 9,
                        "y": 18
                    }
                },
                "cooldown": 11,
                "store": {
                    "energy": 0
                }
            }))
            .unwrap(),
        );

        assert_eq!(
            obj.action_log,
            StructureLinkActions {
                transfer_energy: Some(ActionLogTarget { x: 9, y: 18 }),
            }
        );

        obj.update(
            serde_json::from_value(json!({
                "actionLog": {
                    "transferEnergy": null
                },
                "cooldown": 10,
                "store": {
                    "energy": 50
                }
            }))
            .unwrap(),
        );

        assert_eq!(
            obj,
            StructureLink {
                room: RoomName::new("E17N55").unwrap(),
                x: 9,
                y: 6,
                id: "57fdb3ea3dad49a17265ecea".to_owned(),
                store: store! { Energy: 50 },
                capacity_resource: store! { Energy: 800 },
                hits: 1000,
                hits_max: 1000,
                notify_when_attacked: true,
                disabled: false,
                cooldown: 10,
                action_log: StructureLinkActions {
                    transfer_energy: None,
                },
                user: "57874d42d0ae911e3bd15bbc".to_owned(),
            }
        );
    }
}
