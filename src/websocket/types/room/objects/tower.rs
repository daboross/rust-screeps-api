//! `StructureLink` data description.
use super::ActionLogTarget;
use data::RoomName;

with_structure_fields_and_update_struct! {
    /// A tower structure - a structure that can be filled with energy, and then attack, heal and
    /// repair things within the same room using that energy.
    #[derive(Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureTower {
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
        /// A record of all actions this structure performed last tick.
        pub action_log: StructureTowerActions,
    }

    /// The update structure for a `StructureTower`.
    #[derive(Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureTowerUpdate {
        - user: String,
        #[serde(rename = "off")]
        - disabled: bool,
        - energy: i32,
        - energy_capacity: i32,
        - notify_when_attacked: bool,
        - action_log: StructureTowerActions,
    }
}

with_update_struct! {
    /// A struct describing a tower's actions.
    #[derive(serde_derive::Deserialize, Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureTowerActions {
        /// Where this tower attacked last tick.
        pub attack: Option<ActionLogTarget>,
        /// Where this tower healed last tick.
        pub heal: Option<ActionLogTarget>,
        /// Where this tower repaired last tick.
        pub repair: Option<ActionLogTarget>,
    }

    /// The update structure for a `StructureTowerActions`.
    #[derive(serde_derive::Deserialize, Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureTowerActionsUpdate { ... }
}

#[cfg(test)]
mod test {
    use serde::Deserialize;
    use serde_json;

    use data::RoomName;

    use super::{ActionLogTarget, StructureTower, StructureTowerActions};

    #[test]
    fn parse_tower_and_update() {
        let json = json!({
            "_id": "57f1cc9d27e2c0520e93ba95",
            "actionLog": {
                "attack": null,
                "heal": null,
                "repair": null
            },
            "energy": 920,
            "energyCapacity": 1000,
            "hits": 3000,
            "hitsMax": 3000,
            "notifyWhenAttacked": true,
            "room": "E17N55",
            "type": "tower",
            "user": "57874d42d0ae911e3bd15bbc",
            "x": 9,
            "y": 19
        });

        let mut obj = StructureTower::deserialize(json).unwrap();

        assert_eq!(
            obj,
            StructureTower {
                room: RoomName::new("E17N55").unwrap(),
                x: 9,
                y: 19,
                id: "57f1cc9d27e2c0520e93ba95".to_owned(),
                energy: 920,
                energy_capacity: 1000,
                hits: 3000,
                hits_max: 3000,
                notify_when_attacked: true,
                disabled: false,
                action_log: StructureTowerActions {
                    attack: None,
                    heal: None,
                    repair: None,
                },
                user: "57874d42d0ae911e3bd15bbc".to_owned(),
            }
        );

        obj.update(
            serde_json::from_value(json!({
                "actionLog": {
                    "attack": {
                        "x": 10,
                        "y": 10
                    }
                },
                "energy": 820
            }))
            .unwrap(),
        );

        assert_eq!(
            obj.action_log,
            StructureTowerActions {
                attack: Some(ActionLogTarget { x: 10, y: 10 }),
                heal: None,
                repair: None,
            }
        );
    }
}
