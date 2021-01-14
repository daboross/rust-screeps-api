//! `StructureLink` data description.
use super::super::resources::Store;
use crate::data::RoomName;

use super::ActionLogTarget;

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
        /// The current amount of resources held in this structure.
        pub store: Store,
        /// The maximum amount of resources that can be held in this structure.
        pub store_capacity_resource: Store,
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
        - store: Store,
        - store_capacity_resource: Store,
        - notify_when_attacked: bool,
        - action_log: StructureTowerActions,
    }
}

with_update_struct! {
    /// A struct describing a tower's actions.
    #[derive(serde::Deserialize, Clone, Debug, PartialEq)]
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
    #[derive(serde::Deserialize, Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureTowerActionsUpdate { ... }
}

#[cfg(test)]
mod test {
    use serde::Deserialize;
    use serde_json;

    use crate::data::RoomName;

    use super::{ActionLogTarget, StructureTower, StructureTowerActions};

    #[test]
    fn parse_tower_and_update() {
        let json = json!({
          "_id": "5d334cefdbfe1b628e862a0d",
          "type": "tower",
          "x": 26,
          "y": 25,
          "room": "W44S12",
          "notifyWhenAttacked": true,
          "user": "5a8466038f866773f59fa6c8",
          "hits": 3000,
          "hitsMax": 3000,
          "actionLog": {
            "attack": {
              "x": 46,
              "y": 7
            },
            "heal": null,
            "repair": null
          },
          "store": {
            "energy": 570
          },
          "storeCapacityResource": {
            "energy": 1000
          }
        });

        let mut obj = StructureTower::deserialize(json).unwrap();

        assert_eq!(
            obj,
            StructureTower {
                room: RoomName::new("W44S12").unwrap(),
                x: 26,
                y: 25,
                id: "5d334cefdbfe1b628e862a0d".to_owned(),
                store: store! { Energy: 570 },
                store_capacity_resource: store! { Energy: 1000 },
                hits: 3000,
                hits_max: 3000,
                notify_when_attacked: true,
                disabled: false,
                action_log: StructureTowerActions {
                    attack: Some(ActionLogTarget { x: 46, y: 7 }),
                    heal: None,
                    repair: None,
                },
                user: "5a8466038f866773f59fa6c8".to_owned(),
            }
        );

        obj.update(
            serde_json::from_value(json!({
              "actionLog": {
                "attack": {
                  "x": 45,
                  "y": 6
                }
              },
              "store": {
                "energy": 560
              }
            }))
            .unwrap(),
        );

        assert_eq!(
            obj.action_log,
            StructureTowerActions {
                attack: Some(ActionLogTarget { x: 45, y: 6 }),
                heal: None,
                repair: None,
            }
        );

        assert_eq!(obj.store, store! { Energy: 560 });

        obj.update(
            serde_json::from_value(json!({
              "store": {
                "energy": 550
              }
            }))
            .unwrap(),
        );

        assert_eq!(
            obj.action_log,
            StructureTowerActions {
                attack: Some(ActionLogTarget { x: 45, y: 6 }),
                heal: None,
                repair: None,
            }
        );

        assert_eq!(obj.store, store! { Energy: 550 });

        obj.update(
            serde_json::from_value(json!({
              "actionLog": {
                "attack": null
              }
            }))
            .unwrap(),
        );

        assert_eq!(
            obj.action_log,
            StructureTowerActions {
                attack: None,
                heal: None,
                repair: None,
            },
        );
    }
}
