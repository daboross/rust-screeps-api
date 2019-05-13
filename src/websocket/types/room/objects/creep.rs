//! `Creep` data description.
use super::super::resources::ResourceType;
use super::ActionLogTarget;
use data::RoomName;

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

with_resource_fields_and_update_struct! {
    // Unfortunately, nested macros are not allowed, so we list all resource
    // types manually.
    //
    // This is copy-pasted from `resources.rs`, and any updates here should also be updated
    // there.
    //
    // see: https://github.com/rust-lang/rust/issues/35853
    {
        crate::websocket::types::room::resources::ResourceType;

        Energy => energy => "energy"
            => Some(crate::websocket::types::room::resources::ResourceType::Power);
        Power => power => "power"
            => Some(crate::websocket::types::room::resources::ResourceType::Hydrogen);
        Hydrogen => hydrogen => "H"
            => Some(crate::websocket::types::room::resources::ResourceType::Oxygen);
        Oxygen => oxygen => "O"
            => Some(crate::websocket::types::room::resources::ResourceType::Utrium);
        Utrium => utrium => "U"
            => Some(crate::websocket::types::room::resources::ResourceType::Lemergium);
        Lemergium => lemergium => "L"
            => Some(crate::websocket::types::room::resources::ResourceType::Keanium);
        Keanium => keanium => "K"
            => Some(crate::websocket::types::room::resources::ResourceType::Zynthium);
        Zynthium => zynthium => "Z"
            => Some(crate::websocket::types::room::resources::ResourceType::Catalyst);
        Catalyst => catalyst => "X"
            => Some(crate::websocket::types::room::resources::ResourceType::Ghodium);
        Ghodium => ghodium => "G"
            => Some(crate::websocket::types::room::resources::ResourceType::Hydroxide);
        Hydroxide => hydroxide => "OH"
            => Some(crate::websocket::types::room::resources::ResourceType::ZynthiumKeanite);
        ZynthiumKeanite => zynthium_keanite => "ZK"
            => Some(crate::websocket::types::room::resources::ResourceType::UtriumLemergite);
        UtriumLemergite => utrium_lemergite => "UL"
            => Some(crate::websocket::types::room::resources::ResourceType::UtriumHydride);
        UtriumHydride => utrium_hydride => "UH"
            => Some(crate::websocket::types::room::resources::ResourceType::UtriumOxide);
        UtriumOxide => utrium_oxide => "UO"
            => Some(crate::websocket::types::room::resources::ResourceType::KeaniumHydride);
        KeaniumHydride => keanium_hydride => "KH"
            => Some(crate::websocket::types::room::resources::ResourceType::KeaniumOxide);
        KeaniumOxide => keanium_oxide => "KO"
            => Some(crate::websocket::types::room::resources::ResourceType::LemergiumHydride);
        LemergiumHydride => lemergium_hydride => "LH"
            => Some(crate::websocket::types::room::resources::ResourceType::LemergiumOxide);
        LemergiumOxide => lemergium_oxide => "LO"
            => Some(crate::websocket::types::room::resources::ResourceType::ZynthiumHydride);
        ZynthiumHydride => zynthium_hydride => "ZH"
            => Some(crate::websocket::types::room::resources::ResourceType::ZynthiumOxide);
        ZynthiumOxide => zynthium_oxide => "ZO"
            => Some(crate::websocket::types::room::resources::ResourceType::GhodiumHydride);
        GhodiumHydride => ghodium_hydride => "GH"
            => Some(crate::websocket::types::room::resources::ResourceType::GhodiumOxide);
        GhodiumOxide => ghodium_oxide => "GO"
            => Some(crate::websocket::types::room::resources::ResourceType::UtriumAcid);
        UtriumAcid => utrium_acid => "UH2O"
            => Some(crate::websocket::types::room::resources::ResourceType::UtriumAlkalide);
        UtriumAlkalide => utrium_alkalide => "UHO2"
            => Some(crate::websocket::types::room::resources::ResourceType::KeaniumAcid);
        KeaniumAcid => keanium_acid => "KH2O"
            => Some(crate::websocket::types::room::resources::ResourceType::KeaniumAlkalide);
        KeaniumAlkalide => keanium_alkalide => "KHO2"
            => Some(crate::websocket::types::room::resources::ResourceType::LemergiumAcid);
        LemergiumAcid => lemergium_acid => "LH2O"
            => Some(crate::websocket::types::room::resources::ResourceType::LemergiumAlkalide);
        LemergiumAlkalide => lemergium_alkalide => "LHO2"
            => Some(crate::websocket::types::room::resources::ResourceType::ZynthiumAcid);
        ZynthiumAcid => zynthium_acid => "ZH2O"
            => Some(crate::websocket::types::room::resources::ResourceType::ZynthiumAlkalide);
        ZynthiumAlkalide => zynthium_alkalide => "ZHO2"
            => Some(crate::websocket::types::room::resources::ResourceType::GhodiumAcid);
        GhodiumAcid => ghodium_acid => "GH2O"
            => Some(crate::websocket::types::room::resources::ResourceType::GhodiumAlkalide);
        GhodiumAlkalide => ghodium_alkalide => "GHO2"
            => Some(crate::websocket::types::room::resources::ResourceType::CatalyzedUtriumAcid);
        CatalyzedUtriumAcid => catalyzed_utrium_acid => "XUH2O"
            => Some(crate::websocket::types::room::resources::ResourceType::CatalyzedUtriumAlkalide);
        CatalyzedUtriumAlkalide => catalyzed_utrium_alkalide => "XUHO2"
            => Some(crate::websocket::types::room::resources::ResourceType::CatalyzedKeaniumAcid);
        CatalyzedKeaniumAcid => catalyzed_keanium_acid => "XKH2O"
            => Some(crate::websocket::types::room::resources::ResourceType::CatalyzedKeaniumAlkalide);
        CatalyzedKeaniumAlkalide => catalyzed_keanium_alkalide => "XKHO2"
            => Some(crate::websocket::types::room::resources::ResourceType::CatalyzedLemergiumAcid);
        CatalyzedLemergiumAcid => catalyzed_lemergium_acid => "XLH2O"
            => Some(crate::websocket::types::room::resources::ResourceType::CatalyzedLemergiumAlkalide);
        CatalyzedLemergiumAlkalide => catalyzed_lemergium_alkalide => "XLHO2"
            => Some(crate::websocket::types::room::resources::ResourceType::CatalyzedZynthiumAcid);
        CatalyzedZynthiumAcid => catalyzed_zynthium_acid => "XZH2O"
            => Some(crate::websocket::types::room::resources::ResourceType::CatalyzedZynthiumAlkalide);
        CatalyzedZynthiumAlkalide => catalyzed_zynthium_alkalide => "XZHO2"
            => Some(crate::websocket::types::room::resources::ResourceType::CatalyzedGhodiumAcid);
        CatalyzedGhodiumAcid => catalyzed_ghodium_acid => "XGH2O"
            => Some(crate::websocket::types::room::resources::ResourceType::CatalyzedGhodiumAlkalide);
        CatalyzedGhodiumAlkalide => catalyzed_ghodium_alkalide => "XGHO2"
            => None;
    }

    /// A creep object.
    #[derive(Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct Creep {
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
        pub action_log: CreepActions,
    }

    /// The update structure for a `Creep`.
    #[derive(Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct CreepUpdate {
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
    }
}

resource_iterator_for! {
    // Unfortunately, nested macros are not allowed, so we list all resource
    // types manually.
    //
    // This is copy-pasted from `resources.rs`, and any updates here should also be updated
    // there.
    //
    // see: https://github.com/rust-lang/rust/issues/35853
    {
        crate::websocket::types::room::resources::ResourceType;

        Energy => energy => "energy"
            => Some(crate::websocket::types::room::resources::ResourceType::Power);
        Power => power => "power"
            => Some(crate::websocket::types::room::resources::ResourceType::Hydrogen);
        Hydrogen => hydrogen => "H"
            => Some(crate::websocket::types::room::resources::ResourceType::Oxygen);
        Oxygen => oxygen => "O"
            => Some(crate::websocket::types::room::resources::ResourceType::Utrium);
        Utrium => utrium => "U"
            => Some(crate::websocket::types::room::resources::ResourceType::Lemergium);
        Lemergium => lemergium => "L"
            => Some(crate::websocket::types::room::resources::ResourceType::Keanium);
        Keanium => keanium => "K"
            => Some(crate::websocket::types::room::resources::ResourceType::Zynthium);
        Zynthium => zynthium => "Z"
            => Some(crate::websocket::types::room::resources::ResourceType::Catalyst);
        Catalyst => catalyst => "X"
            => Some(crate::websocket::types::room::resources::ResourceType::Ghodium);
        Ghodium => ghodium => "G"
            => Some(crate::websocket::types::room::resources::ResourceType::Hydroxide);
        Hydroxide => hydroxide => "OH"
            => Some(crate::websocket::types::room::resources::ResourceType::ZynthiumKeanite);
        ZynthiumKeanite => zynthium_keanite => "ZK"
            => Some(crate::websocket::types::room::resources::ResourceType::UtriumLemergite);
        UtriumLemergite => utrium_lemergite => "UL"
            => Some(crate::websocket::types::room::resources::ResourceType::UtriumHydride);
        UtriumHydride => utrium_hydride => "UH"
            => Some(crate::websocket::types::room::resources::ResourceType::UtriumOxide);
        UtriumOxide => utrium_oxide => "UO"
            => Some(crate::websocket::types::room::resources::ResourceType::KeaniumHydride);
        KeaniumHydride => keanium_hydride => "KH"
            => Some(crate::websocket::types::room::resources::ResourceType::KeaniumOxide);
        KeaniumOxide => keanium_oxide => "KO"
            => Some(crate::websocket::types::room::resources::ResourceType::LemergiumHydride);
        LemergiumHydride => lemergium_hydride => "LH"
            => Some(crate::websocket::types::room::resources::ResourceType::LemergiumOxide);
        LemergiumOxide => lemergium_oxide => "LO"
            => Some(crate::websocket::types::room::resources::ResourceType::ZynthiumHydride);
        ZynthiumHydride => zynthium_hydride => "ZH"
            => Some(crate::websocket::types::room::resources::ResourceType::ZynthiumOxide);
        ZynthiumOxide => zynthium_oxide => "ZO"
            => Some(crate::websocket::types::room::resources::ResourceType::GhodiumHydride);
        GhodiumHydride => ghodium_hydride => "GH"
            => Some(crate::websocket::types::room::resources::ResourceType::GhodiumOxide);
        GhodiumOxide => ghodium_oxide => "GO"
            => Some(crate::websocket::types::room::resources::ResourceType::UtriumAcid);
        UtriumAcid => utrium_acid => "UH2O"
            => Some(crate::websocket::types::room::resources::ResourceType::UtriumAlkalide);
        UtriumAlkalide => utrium_alkalide => "UHO2"
            => Some(crate::websocket::types::room::resources::ResourceType::KeaniumAcid);
        KeaniumAcid => keanium_acid => "KH2O"
            => Some(crate::websocket::types::room::resources::ResourceType::KeaniumAlkalide);
        KeaniumAlkalide => keanium_alkalide => "KHO2"
            => Some(crate::websocket::types::room::resources::ResourceType::LemergiumAcid);
        LemergiumAcid => lemergium_acid => "LH2O"
            => Some(crate::websocket::types::room::resources::ResourceType::LemergiumAlkalide);
        LemergiumAlkalide => lemergium_alkalide => "LHO2"
            => Some(crate::websocket::types::room::resources::ResourceType::ZynthiumAcid);
        ZynthiumAcid => zynthium_acid => "ZH2O"
            => Some(crate::websocket::types::room::resources::ResourceType::ZynthiumAlkalide);
        ZynthiumAlkalide => zynthium_alkalide => "ZHO2"
            => Some(crate::websocket::types::room::resources::ResourceType::GhodiumAcid);
        GhodiumAcid => ghodium_acid => "GH2O"
            => Some(crate::websocket::types::room::resources::ResourceType::GhodiumAlkalide);
        GhodiumAlkalide => ghodium_alkalide => "GHO2"
            => Some(crate::websocket::types::room::resources::ResourceType::CatalyzedUtriumAcid);
        CatalyzedUtriumAcid => catalyzed_utrium_acid => "XUH2O"
            => Some(crate::websocket::types::room::resources::ResourceType::CatalyzedUtriumAlkalide);
        CatalyzedUtriumAlkalide => catalyzed_utrium_alkalide => "XUHO2"
            => Some(crate::websocket::types::room::resources::ResourceType::CatalyzedKeaniumAcid);
        CatalyzedKeaniumAcid => catalyzed_keanium_acid => "XKH2O"
            => Some(crate::websocket::types::room::resources::ResourceType::CatalyzedKeaniumAlkalide);
        CatalyzedKeaniumAlkalide => catalyzed_keanium_alkalide => "XKHO2"
            => Some(crate::websocket::types::room::resources::ResourceType::CatalyzedLemergiumAcid);
        CatalyzedLemergiumAcid => catalyzed_lemergium_acid => "XLH2O"
            => Some(crate::websocket::types::room::resources::ResourceType::CatalyzedLemergiumAlkalide);
        CatalyzedLemergiumAlkalide => catalyzed_lemergium_alkalide => "XLHO2"
            => Some(crate::websocket::types::room::resources::ResourceType::CatalyzedZynthiumAcid);
        CatalyzedZynthiumAcid => catalyzed_zynthium_acid => "XZH2O"
            => Some(crate::websocket::types::room::resources::ResourceType::CatalyzedZynthiumAlkalide);
        CatalyzedZynthiumAlkalide => catalyzed_zynthium_alkalide => "XZHO2"
            => Some(crate::websocket::types::room::resources::ResourceType::CatalyzedGhodiumAcid);
        CatalyzedGhodiumAcid => catalyzed_ghodium_acid => "XGH2O"
            => Some(crate::websocket::types::room::resources::ResourceType::CatalyzedGhodiumAlkalide);
        CatalyzedGhodiumAlkalide => catalyzed_ghodium_alkalide => "XGHO2"
            => None;
    }

    /// Resource iterator for a `Creep`.
    #[derive(Debug)]
    pub struct CreepContents(Creep);
}

impl Creep {
    /// Iterates over this creep's carried resources.
    pub fn carry_contents(&self) -> CreepContents {
        CreepContents::new(self)
    }
}

#[cfg(test)]
mod test {
    use serde::Deserialize;

    use data::RoomName;

    use super::{Creep, CreepActions, CreepPartType};

    #[test]
    fn parse_creep() {
        let json = json!({
            "_id": "5969704a55d1b111cbe6b150",
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
                "upgradeController": null
            },
            "ageTime": 20236257,
            "body": [
                {
                    "hits": 100,
                    "type": "carry"
                },
                {
                    "hits": 100,
                    "type": "work"
                },
                {
                    "hits": 100,
                    "type": "work"
                },
                {
                    "hits": 100,
                    "type": "work"
                },
                {
                    "hits": 100,
                    "type": "work"
                },
                {
                    "hits": 100,
                    "type": "work"
                },
                {
                    "hits": 100,
                    "type": "work"
                },
                {
                    "hits": 100,
                    "type": "work"
                },
                {
                    "hits": 100,
                    "type": "work"
                },
                {
                    "hits": 100,
                    "type": "work"
                },
                {
                    "hits": 100,
                    "type": "work"
                },
                {
                    "hits": 100,
                    "type": "work"
                },
                {
                    "hits": 100,
                    "type": "work"
                },
                {
                    "hits": 100,
                    "type": "work"
                },
                {
                    "hits": 100,
                    "type": "work"
                },
                {
                    "hits": 100,
                    "type": "work"
                },
                {
                    "hits": 100,
                    "type": "work"
                },
                {
                    "hits": 100,
                    "type": "work"
                },
                {
                    "hits": 100,
                    "type": "carry"
                },
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
                    "type": "move"
                },
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
                    "type": "move"
                },
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
                    "type": "move"
                },
                {
                    "hits": 100,
                    "type": "move"
                }
            ],
            "energy": 0,
            "energyCapacity": 100,
            "fatigue": 0,
            "hits": 2900,
            "hitsMax": 2900,
            "name": "b873",
            "notifyWhenAttacked": true,
            "room": "W65N19",
            "spawning": false,
            "type": "creep",
            "user": "57874d42d0ae911e3bd15bbc",
            "x": 27,
            "y": 38
        });

        let obj = Creep::deserialize(json).unwrap();

        match obj {
            Creep {
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
                    panic!("some fields wrong from pre-set Creep: {:#?}", obj);
                }
            }
            other => panic!("expected pre-set Creep to match, found {:#?}", other),
        }

        assert_eq!(obj.carry_contents().collect::<Vec<_>>(), vec![]);
    }
}
