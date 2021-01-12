//! `StructureLab` data description.
use super::super::resources::ResourceType;
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
        /// The current amount of energy held in this structure.
        pub energy: i32,
        /// The maximum amount of energy that can be held in this structure.
        pub energy_capacity: i32,
        /// The type of mineral stored in this lab.
        pub mineral_type: Option<ResourceType>,
        /// The amount of whatever mineral is stored in this lab.
        pub mineral_amount: i32,
        /// The maximum amount of any mineral that can be held in this structure.
        pub mineral_capacity: i32,
        /// The number of ticks till this lab can run a reaction again.
        pub cooldown: i32,
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
        - energy: i32,
        - energy_capacity: i32,
        - mineral_type: Option<ResourceType>,
        - mineral_amount: i32,
        - mineral_capacity: i32,
        - cooldown: i32,
        - action_log: StructureLabActions,
        - notify_when_attacked: bool,
    }
}

with_update_struct! {
    /// A struct describing the source labs for a lab performing a mineral reaction.
    #[derive(serde::Deserialize, Clone, Debug, PartialEq)]
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
    #[derive(serde::Deserialize, Clone, Debug)]
    pub struct LabActionTargetUpdate { ... }
}

with_update_struct! {
    /// A struct describing a lab's actions.
    #[derive(serde::Deserialize, Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureLabActions {
        /// The two source labs that provided minerals for the reaction that was run last tick.
        pub run_reaction: Option<LabActionTarget>,
    }

    /// The update structure for a `StructureLabActions`.
    #[derive(serde::Deserialize, Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureLabActionsUpdate { ... }
}

#[cfg(test)]
mod test {
    use serde::Deserialize;
    use serde_json;

    use crate::data::RoomName;

    use super::{LabActionTarget, ResourceType, StructureLab, StructureLabActions};

    #[test]
    fn parse_lab_and_updates() {
        let json = json!({
            "_id": "58228250580b9e752863fd95",
            "actionLog": {
                "runReaction": null
            },
            "cooldown": 6,
            "energy": 2000,
            "energyCapacity": 2000,
            "hits": 500,
            "hitsMax": 500,
            "mineralAmount": 155,
            "mineralCapacity": 3000,
            "mineralType": "KHO2",
            "notifyWhenAttacked": true,
            "room": "E9N23",
            "type": "lab",
            "user": "561e4d4645f3f7244a7622e8",
            "x": 17,
            "y": 8
        });

        let mut obj = StructureLab::deserialize(json).unwrap();

        assert_eq!(
            obj,
            StructureLab {
                room: RoomName::new("E9N23").unwrap(),
                x: 17,
                y: 8,
                id: "58228250580b9e752863fd95".to_owned(),
                energy: 2000,
                energy_capacity: 2000,
                mineral_amount: 155,
                mineral_capacity: 3000,
                mineral_type: Some(ResourceType::KeaniumAlkalide),
                hits: 500,
                hits_max: 500,
                notify_when_attacked: true,
                disabled: false,
                cooldown: 6,
                action_log: StructureLabActions { run_reaction: None },
                user: "561e4d4645f3f7244a7622e8".to_owned(),
            }
        );

        obj.update(
            serde_json::from_value(json!({
                "cooldown": 5
            }))
            .unwrap(),
        );

        obj.update(
            serde_json::from_value(json!({
                "cooldown": 4
            }))
            .unwrap(),
        );

        obj.update(
            serde_json::from_value(json!({
                "cooldown": 3
            }))
            .unwrap(),
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
                    "runReaction": {
                        "x1": 18,
                        "x2": 17,
                        "y1": 9,
                        "y2": 10
                    }
                },
                "cooldown": 9,
                "mineralAmount": 160
            }))
            .unwrap(),
        );

        assert_eq!(
            obj.action_log,
            StructureLabActions {
                run_reaction: Some(LabActionTarget {
                    x1: 18,
                    y1: 9,
                    x2: 17,
                    y2: 10,
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
                room: RoomName::new("E9N23").unwrap(),
                x: 17,
                y: 8,
                id: "58228250580b9e752863fd95".to_owned(),
                energy: 2000,
                energy_capacity: 2000,
                mineral_amount: 160,
                mineral_capacity: 3000,
                mineral_type: Some(ResourceType::KeaniumAlkalide),
                hits: 500,
                hits_max: 500,
                notify_when_attacked: true,
                disabled: false,
                cooldown: 8,
                action_log: StructureLabActions { run_reaction: None },
                user: "561e4d4645f3f7244a7622e8".to_owned(),
            }
        );
    }

    #[test]
    fn parse_empty_lab() {
        let json = json!({
            "_id": "5968055177adbb592b9c2e4e",
            "type": "lab",
            "x": 32,
            "y": 34,
            "room": "W65N19",
            "notifyWhenAttacked": true,
            "user": "57874d42d0ae911e3bd15bbc",
            "hits": 500,
            "hitsMax": 500,
            "mineralAmount": 0,
            "cooldown": 0,
            "mineralType": null,
            "mineralCapacity": 3000,
            "energy": 2000,
            "energyCapacity": 2000,
            "actionLog": {
                "runReaction": null
            }
        });

        let obj = StructureLab::deserialize(json).unwrap();

        assert_eq!(
            obj,
            StructureLab {
                room: RoomName::new("W65N19").unwrap(),
                x: 32,
                y: 34,
                id: "5968055177adbb592b9c2e4e".to_owned(),
                energy: 2000,
                energy_capacity: 2000,
                mineral_amount: 0,
                mineral_capacity: 3000,
                mineral_type: None,
                hits: 500,
                hits_max: 500,
                notify_when_attacked: true,
                disabled: false,
                cooldown: 0,
                action_log: StructureLabActions { run_reaction: None },
                user: "57874d42d0ae911e3bd15bbc".to_owned(),
            }
        );
    }
}
