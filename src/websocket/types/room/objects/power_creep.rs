//! `Creep` data description.
use super::super::resources::ResourceType;
use super::creep::CreepMessage;
use super::ActionLogTarget;
use crate::data::RoomName;

with_update_struct! {
    /// A struct describing a creep's actions.
    #[derive(serde_derive::Deserialize, Default, Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct PowerCreepActions {
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
    pub struct PowerCreepActionsUpdate { ... }
}

with_base_fields_and_update_struct! {
    /// A creep object.
    #[derive(Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct PowerCreep {
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
        #[serde(rename = "energyCapacity")]
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
        pub action_log: PowerCreepActions,
    }

    /// The update structure for a `PowerCreep`.
    #[derive(Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct PowerCreepUpdate {
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
        - action_log: PowerCreepActions,
    }
}

#[cfg(test)]
mod test {
    use serde::Deserialize;

    use crate::data::RoomName;

    use super::{CreepPartType, PowerCreep, PowerCreepActions};

    #[test]
    fn parse_creep() {
        let json = json!({
            "_id": "5cb1a1cd5341a50b971acd01",
            "actionLog": {
                "attack": null,
                "attacked": null,
                "healed": null,
                "power": null,
                "say": null,
                "spawned": null,
            },
            "ageTime": 18241147,
            "className": "operator",
            "deleteTime": null,
            "energy": 1922,
            "energyCapacity": 2600,
            "hits": 26000,
            "hitsMax": 26000,
            "level": 25,
            "name": "oprEco_457",
            "ops": 274,
            "powers": {
                "1": {
                    "cooldownTime": 18240561,
                    "level": 5,
                },
                "13": {
                    "cooldownTime": 18240585,
                    "level": 5,
                },
                "14": {
                    "cooldownTime": 18205010,
                    "level": 5,
                },
                "2": {
                    "cooldownTime": 18231062,
                    "level": 4,
                },
                "3": {
                    "level": 2,
                },
                "6": {
                    "cooldownTime": 18239880,
                    "level": 4
                },
            },
            "room": "E8N32",
            "shard": "shard1",
            "spawnCooldownTime": null,
            "type": "powerCreep",
            "user": "58519b0bee6ae29347627228",
            "x": 36,
            "y": 24,
        });

        let obj = PowerCreep::deserialize(json).unwrap();

        match obj {
            PowerCreep {
                ref id,
                ref room,
                x: 27,
                y: 38,
                energy: 0,
                ghodium: 0,
                capacity: 100,
                hits: 2900,
                hits_max: 2900,
                age_time: Some(20236257),
                ref name,
                notify_when_attacked: true,
                spawning: false,
                action_log:
                    PowerCreepActions {
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
                        upgrade_controller: None,
                    },
                ref body,
                ref user,
                ..
            } => {
                if user != "57874d42d0ae911e3bd15bbc"
                    || id != "5969704a55d1b111cbe6b150"
                    || *room != RoomName::new("W65N19").unwrap()
                    || name != "b873"
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
                        != 2
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
                        != 17
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
                        != 10
                    || !body.iter().all(|part| {
                        part.part_type == CreepPartType::Move
                            || part.part_type == CreepPartType::Work
                            || part.part_type == CreepPartType::Carry
                    })
                {
                    panic!("some fields wrong from pre-set PowerCreep: {:#?}", obj);
                }
            }
            other => panic!("expected pre-set PowerCreep to match, found {:#?}", other),
        }

        assert_eq!(obj.carry_contents().collect::<Vec<_>>(), vec![]);
    }
}
