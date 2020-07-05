//! `Creep` data description.
use super::super::resources::ResourceType;
use super::super::resources::Store;
use super::ActionLogTarget;
use crate::data::RoomName;

with_update_struct! {
    /// A struct describing a creep part.
    ///
    /// TODO: parse creep part boosts.
    #[derive(serde_derive::Deserialize, Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct CreepPart {
        /// Part health, out of 100.
        pub hits: i32,
        /// Part type.
        #[serde(rename = "type")]
        pub part_type: CreepPartType,
        /// Part boost, if any.
        pub boost: Option<ResourceType>,
    }

    /// The update structure for a `CreepPart`.
    #[derive(serde_derive::Deserialize, Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct CreepPartUpdate { ... }
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "snake_case")]
/// A type of creep part.
pub enum CreepPartType {
    /// A move part - allows creeps to move, or move faster (max speed at 1 move part per other part).
    Move,
    /// A work part - allows creeps to spend energy doing things, mine sources and minerals, and damage
    /// structures.
    Work,
    /// A carry part - allows creeps to carry energy and other resources.
    Carry,
    /// An attack part - allows creeps to damage both other creeps, and structures.
    Attack,
    /// A heal part - allows creeps to heal creeps (other and self).
    Heal,
    /// A ranged attack part - allows creeps to attack other creeps and structures from a small distance.
    RangedAttack,
    /// A tough part - a cheap part which has no additional functionality.
    Tough,
    /// A claim part - a part which allows a creep to reserve or claim a room controller.
    Claim,
}

basic_updatable!(CreepPartType);

with_update_struct! {
    /// A struct describing a creep's message conveyed with `say`.
    #[derive(serde_derive::Deserialize, Default, Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct CreepMessage {
        /// If true, the message is visible to all players.
        pub is_public: bool,
        /// The message.
        pub message: String,
    }

    /// The update structure for a `CreepMessage`.
    #[derive(serde_derive::Deserialize, Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct CreepMessageUpdate { ... }
}

with_update_struct! {
    /// A struct describing a creep's actions.
    #[derive(serde_derive::Deserialize, Default, Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct CreepActions {
        /// The location this creep harvested last tick.
        pub harvest: Option<ActionLogTarget>,
        /// The location this creep built to last tick.
        pub build: Option<ActionLogTarget>,
        /// The location this creep repaired last tick.
        pub repair: Option<ActionLogTarget>,
        /// The location this creep attacked last tick.
        pub attack: Option<ActionLogTarget>,
        /// The location this creep healed last tick.
        pub heal: Option<ActionLogTarget>,
        /// The location this creep range-attacked last tick.
        pub ranged_attack: Option<ActionLogTarget>,
        /// The location this creep range-healed last tick.
        pub ranged_heal: Option<ActionLogTarget>,
        /// If present, this creep made a ranged mass attack last tick.
        pub ranged_mass_attack: Option<ActionLogTarget>,
        /// The location of the controller this creep reserved last tick.
        pub reserve_controller: Option<ActionLogTarget>,
        /// The location of the controller this creep upgraded last tick.
        pub upgrade_controller: Option<ActionLogTarget>,
        /// The location this creep was attacked from last tick.
        pub attacked: Option<ActionLogTarget>,
        /// The location this creep was healed from last tick.
        pub healed: Option<ActionLogTarget>,
        /// The message this creep said last tick. TODO: confirm this is correct.
        pub say: Option<CreepMessage>,
    }

    /// The update structure for a `CreepActions`.
    #[derive(serde_derive::Deserialize, Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct CreepActionsUpdate { ... }
}

with_base_fields_and_update_struct! {
    /// A creep object.
    #[derive(Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct Creep {
        /// The current number of hit-points this structure has.
        #[serde(default)]
        pub hits: i32,
        /// The maximum number of hit-points this structure has.
        #[serde(default)]
        pub hits_max: i32,
        /// The user ID of the owner of this creep.
        pub user: String,
        /// Whether or not this creep is currently being constructed 'inside' a spawner. If this is the case,
        /// it cannot perform any actions.
        #[serde(default)]
        pub spawning: bool,
        /// The total amount of resources this creep can carry.
        #[serde(rename = "storeCapacity")]
        pub capacity: i32,
        /// Whether or not an attack on this creep will send an email to the owner automatically..
        #[serde(default)]
        pub notify_when_attacked: bool,
        /// The name of this creep, unique per player.
        pub name: String,
        /// The current 'fatigue' of this creep. If higher than 0, this creep cannot execute a move action this tick.
        pub fatigue: i32,
        /// The game time at which this creep will die if not renewed further before then.
        ///
        /// None if the creep is still spawning.
        pub age_time: Option<u32>,
        /// A list of parts on this creep, and their current states.
        pub body: Vec<CreepPart>,
        /// A record of all actions this creep performed and some actions performed onto this creep last tick.
        #[serde(default)] // won't exist for spawning creeps, but we can just do this.
        pub action_log: CreepActions,
        /// The resources and amounts of each resource some game object holds.
        pub store: Store,
    }

    /// The update structure for a `Creep`.
    #[derive(Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct CreepUpdate {
        - hits: i32,
        - hits_max: i32,
        - user: String,
        - spawning: bool,
        #[serde(rename = "energyCapacity")]
        - capacity: i32,
        - notify_when_attacked: bool,
        - name: String,
        - fatigue: i32,
        - age_time: Option<u32>,
        - body: Vec<CreepPart>,
        - action_log: CreepActions,
        - store: Store,
    }
}

impl Creep {
    /// Iterates over this creep's carried resources.
    pub fn carry_contents(&self) -> impl Iterator<Item = (ResourceType, i32)> + '_ {
        self.store.iter()
    }
}

#[cfg(test)]
mod test {
    use serde::Deserialize;

    use crate::data::RoomName;

    use super::{Creep, CreepActions, CreepPartType};

    #[test]
    fn parse_creep() {
        let json = json!({
            "_id": "5e117142fadd09a383ffdc99",
            "actionLog": {
                "attack": null,
                "attacked": null,
                "build": null,
                "harvest": null,
                "heal": null,
                "healed": null,
                "rangedAttack": null,
                "rangedHeal": null,
                "rangedMassAttack": null,
                "repair": null,
                "reserveController": null,
                "say": null,
                "upgradeController": {
                    "x": 27,
                    "y": 8,
                }
            },
            "ageTime": 23469491,
            "body": [
                {
                    "hits": 100,
                    "type": "move"
                },
                {
                    "hits": 100,
                    "type": "move"
                },
                {
                    "hits": 100,
                    "type": "carry"
                },
                {
                    "hits": 100,
                    "type": "work"
                }
            ],
            "fatigue": 0,
            "hits": 400,
            "hitsMax": 400,
            "name": "w9g1gpnN",
            "notifyWhenAttacked": true,
            "room": "E44S19",
            "spawning": false,
            "store": {
                "energy": 13
            },
            "storeCapacity": 50,
            "type": "creep",
            "user": "57874d42d0ae911e3bd15bbc",
            "x": 26,
            "y": 7
        });

        let obj = Creep::deserialize(json).unwrap();

        match obj {
            Creep {
                ref id,
                ref room,
                x: 26,
                y: 7,
                capacity: 50,
                hits: 400,
                hits_max: 400,
                age_time: Some(23469491),
                ref name,
                notify_when_attacked: true,
                spawning: false,
                action_log:
                    CreepActions {
                        attack: None,
                        attacked: None,
                        build: None,
                        harvest: None,
                        heal: None,
                        healed: None,
                        ranged_attack: None,
                        ranged_heal: None,
                        ranged_mass_attack: None,
                        repair: None,
                        reserve_controller: None,
                        say: None,
                        upgrade_controller: ActionLogTarget { x: 27, y: 8 },
                    },
                ref body,
                ref user,
                ..
            } => {
                if user != "57874d42d0ae911e3bd15bbc"
                    || id != "5e117142fadd09a383ffdc99"
                    || *room != RoomName::new("E33S19").unwrap()
                    || name != "w9g1gpnN"
                    || body
                        .iter()
                        .map(|part| {
                            if part.part_type == CreepPartType::Carry {
                                1
                            } else {
                                0
                            }
                        })
                        .sum::<i32>()
                        != 1
                    || body
                        .iter()
                        .map(|part| {
                            if part.part_type == CreepPartType::Work {
                                1
                            } else {
                                0
                            }
                        })
                        .sum::<i32>()
                        != 1
                    || body
                        .iter()
                        .map(|part| {
                            if part.part_type == CreepPartType::Move {
                                1
                            } else {
                                0
                            }
                        })
                        .sum::<i32>()
                        != 2
                    || !body.iter().all(|part| {
                        part.part_type == CreepPartType::Move
                            || part.part_type == CreepPartType::Work
                            || part.part_type == CreepPartType::Carry
                    })
                {
                    panic!("some fields wrong from pre-set Creep: {:#?}", obj);
                }
            }
            other => panic!("expected pre-set Creep to match, found {:#?}", other),
        }

        assert_eq!(obj.carry_contents().collect::<Vec<_>>(), vec![]);
    }
}
