//! `StructureLab` data description.
use super::super::resources::Store;
use crate::data::RoomName;

with_structure_fields_and_update_struct! {
    /// A lab structure - a structure that can be filled with energy and minerals, merge minerals with
    /// minerals from other labs, and use minerals and energy to boost creeps.
    #[derive(Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureLab {
        /// The user ID of the owner of this structure.
        pub user: String,
        /// Whether or not this structure is non-functional due to a degraded controller.
        #[serde(default, rename = "off")]
        pub disabled: bool,
        /// The amount of energy and a mineral or a compound that is stored in this structure.
        pub store: Store,
        /// The total amount of each resource that can be stored in this structure.
        #[serde(rename = "storeCapacityResource")]
        pub capacity_resource: Store,
        /// The tick until which this lab can't run any reactions.
        pub cooldown_time: i32,
        /// A record of all actions this structure performed last tick.
        pub action_log: StructureLabActions,
        /// Whether or not an attack on this structure will send an email to the owner automatically.
        pub notify_when_attacked: bool,
    }

    /// The update structure for a `StructureLab`.
    #[derive(Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureLabUpdate {
        - user: String,
        #[serde(rename = "off")]
        - disabled: bool,
        - store: Store,
        - capacity_resource: Store,
        - cooldown_time: i32,
        - action_log: StructureLabActions,
        - notify_when_attacked: bool,
    }
}

with_update_struct! {
    /// A struct describing the source labs for a lab performing a mineral reaction.
    #[derive(serde_derive::Deserialize, Clone, Debug, PartialEq)]
    pub struct LabActionTarget {
        /// The x position of the first source lab.
        pub x1: u32,
        /// The y position of the first source lab.
        pub y1: u32,
        /// The x position of the second source lab.
        pub x2: u32,
        /// The y position of the second source lab.
        pub y2: u32,
    }

    /// The update structure for a `LabActionTarget`.
    #[derive(serde_derive::Deserialize, Clone, Debug)]
    pub struct LabActionTargetUpdate { ... }
}

with_update_struct! {
    /// A struct describing a lab's actions.
    #[derive(serde_derive::Deserialize, Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureLabActions {
        /// The two source labs that provided minerals for the reaction that was run last tick.
        pub run_reaction: Option<LabActionTarget>,
    }

    /// The update structure for a `StructureLabActions`.
    #[derive(serde_derive::Deserialize, Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureLabActionsUpdate { ... }
}

#[cfg(test)]
mod test {
    use serde::Deserialize;
    use serde_json;

    use crate::data::RoomName;

    use super::{LabActionTarget, StructureLab, StructureLabActions};

    #[test]
    fn parse_lab_and_updates() {
        let json = json!({
            "_id": "5aebc6e4ee797138fa7b4a4f",
            "type": "lab",
            "x": 24,
            "y": 17,
            "room": "E9S32",
            "notifyWhenAttacked": false,
            "user": "589f5265d25357e8253e3ee8",
            "hits": 500,
            "hitsMax": 500,
            "cooldown": 10,
            "actionLog": {
              "runReaction": {
                "x1": 24,
                "y1": 16,
                "x2": 23,
                "y2": 15
              },
              "reverseReaction": null
            },
            "store": {
              "energy": 2000,
              "XZHO2": 0,
              "XKHO2": 0,
              "KHO2": 0,
              "KO": 0,
              "XLHO2": 0,
              "XUH2O": 0,
              "ZO": 0,
              "G": 0,
              "UL": 0,
              "ZK": 0,
              "LO": 0,
              "UH2O": 0,
              "UH": 0,
              "OH": 0,
              "LHO2": 0,
              "ZHO2": 0,
              "ZH2O": 0,
              "GHO2": 0,
              "XGHO2": 0,
              "LH2O": 0,
              "LH": 0,
              "ZH": 0,
              "XZH2O": 0,
              "GO": 0,
              "XLH2O": 0,
              "UO": 45,
              "X": 0,
              "H": 0
            },
            "storeCapacityResource": {
              "energy": 2000,
              "XZHO2": null,
              "XKHO2": null,
              "KHO2": null,
              "KO": null,
              "XLHO2": null,
              "XUH2O": null,
              "ZO": null,
              "G": null,
              "UL": null,
              "ZK": null,
              "LO": null,
              "UH2O": null,
              "UH": null,
              "OH": null,
              "LHO2": null,
              "ZHO2": null,
              "ZH2O": null,
              "GHO2": null,
              "XGHO2": null,
              "LH2O": null,
              "LH": null,
              "ZH": null,
              "XZH2O": null,
              "GO": null,
              "XLH2O": null,
              "UO": 3000,
              "X": null,
              "H": null
            },
            "cooldownTime": 30246725,
            "storeCapacity": null,
            "effects": {
              "0": {
                "effect": 5,
                "power": 5,
                "level": 4,
                "endTime": 29752073
              }
            }
        });

        let mut obj = StructureLab::deserialize(json).unwrap();

        assert_eq!(
            obj,
            StructureLab {
                room: RoomName::new("E9S32").unwrap(),
                x: 24,
                y: 17,
                id: "5aebc6e4ee797138fa7b4a4f".to_owned(),
                store: store! { Energy: 2000, UtriumOxide: 45 },
                capacity_resource: store! { Energy: 2000, UtriumOxide: 3000 },
                hits: 500,
                hits_max: 500,
                cooldown_time: 30246725,
                notify_when_attacked: false,
                disabled: false,
                action_log: StructureLabActions {
                    run_reaction: Some(LabActionTarget {
                        x1: 24,
                        y1: 16,
                        x2: 23,
                        y2: 15,
                    })
                },
                user: "589f5265d25357e8253e3ee8".to_owned(),
            }
        );

        obj.update(
            serde_json::from_value(json!({
              "actionLog": {
                "runReaction": null
              }
            }))
            .unwrap(),
        );

        assert_eq!(obj.action_log.run_reaction, None);

        obj.update(
            serde_json::from_value(json!({
              "actionLog": {
                "runReaction": {
                  "x1": 24,
                  "y1": 16,
                  "x2": 23,
                  "y2": 15
                }
              },
              "store": {
                "UO": 50
              },
              "cooldownTime": 30246735
            }))
            .unwrap(),
        );

        assert_eq!(
            obj.action_log,
            StructureLabActions {
                run_reaction: Some(LabActionTarget {
                    x1: 24,
                    y1: 16,
                    x2: 23,
                    y2: 15,
                }),
            }
        );

        obj.update(
            serde_json::from_value(json!({
                "cooldown": 8,
                "actionLog": {
                    "runReaction": null,
                },
            }))
            .unwrap(),
        );

        assert_eq!(
            obj,
            StructureLab {
                room: RoomName::new("E9S32").unwrap(),
                x: 24,
                y: 17,
                id: "5aebc6e4ee797138fa7b4a4f".to_owned(),
                store: store! { Energy: 2000, UtriumOxide: 50 },
                capacity_resource: store! { Energy: 2000, UtriumOxide: 3000 },
                hits: 500,
                hits_max: 500,
                cooldown_time: 30246735,
                notify_when_attacked: false,
                disabled: false,
                action_log: StructureLabActions { run_reaction: None },
                user: "589f5265d25357e8253e3ee8".to_owned(),
            }
        );
    }

    #[test]
    fn parse_empty_lab() {
        let json = json!({
            "_id": "5d2aca5c5e41f216fb099492",
            "type": "lab",
            "x": 24,
            "y": 28,
            "room": "W44S12",
            "notifyWhenAttacked": true,
            "user": "5a8466038f866773f59fa6c8",
            "hits": 500,
            "hitsMax": 500,
            "cooldown": 0,
            "actionLog": {
              "runReaction": null,
              "reverseReaction": null
            },
            "store": {
              "energy": 2000,
              "ZH": 0,
              "UH": 0,
              "KO": 0,
              "LO": 0,
              "LH": 0,
              "ZK": 0,
              "UL": 0,
              "G": 0,
              "ZO": 0,
              "GO": 0,
              "UO": 0,
              "OH": 0,
              "UH2O": 0,
              "XUH2O": 0,
              "KHO2": 0,
              "LHO2": 0,
              "XKHO2": 0,
              "GH": 0,
              "XLHO2": 0,
              "ZHO2": 0,
              "KH": 0,
              "GHO2": 0,
              "XZHO2": 0,
              "LH2O": 0,
              "XGHO2": 0,
              "ZH2O": 0,
              "XLH2O": 0,
              "XZH2O": 0,
              "KH2O": 0,
              "GH2O": 0,
              "XKH2O": 0,
              "UHO2": 0,
              "XGH2O": 0,
              "XUHO2": 0
            },
            "storeCapacity": 5000,
            "storeCapacityResource": {
              "energy": 2000,
              "ZH": null,
              "UH": null,
              "KO": null,
              "LO": null,
              "LH": null,
              "ZK": null,
              "UL": null,
              "G": null,
              "ZO": null,
              "GO": null,
              "UO": null,
              "OH": null,
              "UH2O": null,
              "XUH2O": null,
              "KHO2": null,
              "LHO2": null,
              "XKHO2": null,
              "GH": null,
              "XLHO2": null,
              "ZHO2": null,
              "KH": null,
              "GHO2": null,
              "XZHO2": null,
              "LH2O": null,
              "XGHO2": null,
              "ZH2O": null,
              "XLH2O": null,
              "XZH2O": null,
              "KH2O": null,
              "GH2O": null,
              "XKH2O": null,
              "UHO2": null,
              "XGH2O": null,
              "XUHO2": null
            },
            "cooldownTime": 23464205
        });

        let obj = StructureLab::deserialize(json).unwrap();

        assert_eq!(
            obj,
            StructureLab {
                room: RoomName::new("W44S12").unwrap(),
                x: 24,
                y: 28,
                id: "5d2aca5c5e41f216fb099492".to_owned(),
                store: store! { Energy: 2000 },
                capacity_resource: store! { Energy: 2000 },
                hits: 500,
                hits_max: 500,
                notify_when_attacked: true,
                disabled: false,
                cooldown_time: 23464205,
                action_log: StructureLabActions { run_reaction: None },
                user: "5a8466038f866773f59fa6c8".to_owned(),
            }
        );
    }
}
