//! `Tombstone` data description.
use crate::data::RoomName;

use super::creep::CreepPartType;

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

    /// A tomstone - remnants of a dead creep
    #[derive(Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct Tombstone {
        /// The user ID of the owner of the creep
        pub user: String,
        /// The body of the creep who died
        pub creep_body: Vec<CreepPartType>,
        /// The ID of the creep who died
        pub creep_id: String,
        /// The name of the creep who died
        pub creep_name: String,
        /// What the creep was saying when they died
        pub creep_saying: Option<String>,
        /// How many ticks-to-live did the creeps have when it died
        pub creep_ticks_to_live: i32,
        /// The time of death of the creep
        pub death_time: u32,
        /// The time at which this structure will decay
        pub decay_time: u32,
        // TODO: what does the tombstone of a power creep look like?
    }

    /// The update structure for a `Tombstone`.
    #[derive(Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct TombstoneUpdate { ... }
}

#[cfg(test)]
mod test {
    use serde::Deserialize;

    use super::Tombstone;

    #[test]
    fn parse_simple_tombstone() {
        let json = json!({
            "_id": "5ce0a11dcd8deb5a978cb216",
            "creepBody": [
                "carry",
                "carry",
                "carry",
                "carry",
                "carry",
                "carry",
                "carry",
                "carry",
                "carry",
                "carry",
                "move",
                "move",
                "move",
                "move",
                "move"
            ],
            "creepId": "5ce08e9f184f823e92616faf",
            "creepName": "HeavyConsolidator6921549",
            "creepSaying": null,
            "creepTicksToLive": 1,
            "deathTime": 6923092,
            "decayTime": 6923167,
            "room": "E8S29",
            "type": "tombstone",
            "user": "5ca80c8f3c33e30c8e85555d",
            "x": 30,
            "y": 31
        });

        let obj = Tombstone::deserialize(json).unwrap();

        assert_eq!(obj.creep_id, "5ce08e9f184f823e92616faf");
        assert_eq!(obj.creep_name, "HeavyConsolidator6921549");
    }

    #[test]
    fn parse_tombstone_with_energy() {
        let json = json!({
            "_id": "5ce0a4839c5bd560bebb03e9",
            "creepBody": [
                "move",
                "move",
                "carry",
                "work"
            ],
            "creepId": "5ce098c5c4e055067fbf182e",
            "creepName": "6922398-0",
            "creepSaying": null,
            "creepTicksToLive": 532,
            "deathTime": 6923377,
            "decayTime": 6923397,
            "energy": 44,
            "room": "E9S31",
            "type": "tombstone",
            "user": "57874d42d0ae911e3bd15bbc",
            "x": 33,
            "y": 23
        });

        let obj = Tombstone::deserialize(json).unwrap();

        assert_eq!(obj.creep_id, "5ce098c5c4e055067fbf182e");
        assert_eq!(obj.creep_name, "6922398-0");
        assert_eq!(obj.energy, 44);
    }
}
