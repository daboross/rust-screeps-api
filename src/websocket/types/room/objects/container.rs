//! `StructureContainer` data description.
use data::RoomName;

with_resource_fields_and_update_struct! {
    // Unfortunately, nested macros are not allowed, so we list all resource
    // types manually.
    //
    // This is copy-pasted from `resources.rs`, and any updates here should also be updated
    // there.
    //
    // see: https://github.com/rust-lang/rust/issues/35853
    {
        ::websocket::types::room::resources::ResourceType;

        Energy => energy => "energy"
            => Some(::websocket::types::room::resources::ResourceType::Power);
        Power => power => "power"
            => Some(::websocket::types::room::resources::ResourceType::Hydrogen);
        Hydrogen => hydrogen => "H"
            => Some(::websocket::types::room::resources::ResourceType::Oxygen);
        Oxygen => oxygen => "O"
            => Some(::websocket::types::room::resources::ResourceType::Utrium);
        Utrium => utrium => "U"
            => Some(::websocket::types::room::resources::ResourceType::Lemergium);
        Lemergium => lemergium => "L"
            => Some(::websocket::types::room::resources::ResourceType::Keanium);
        Keanium => keanium => "K"
            => Some(::websocket::types::room::resources::ResourceType::Zynthium);
        Zynthium => zynthium => "Z"
            => Some(::websocket::types::room::resources::ResourceType::Catalyst);
        Catalyst => catalyst => "X"
            => Some(::websocket::types::room::resources::ResourceType::Ghodium);
        Ghodium => ghodium => "G"
            => Some(::websocket::types::room::resources::ResourceType::Hydroxide);
        Hydroxide => hydroxide => "OH"
            => Some(::websocket::types::room::resources::ResourceType::ZynthiumKeanite);
        ZynthiumKeanite => zynthium_keanite => "ZK"
            => Some(::websocket::types::room::resources::ResourceType::UtriumLemergite);
        UtriumLemergite => utrium_lemergite => "UL"
            => Some(::websocket::types::room::resources::ResourceType::UtriumHydride);
        UtriumHydride => utrium_hydride => "UH"
            => Some(::websocket::types::room::resources::ResourceType::UtriumOxide);
        UtriumOxide => utrium_oxide => "UO"
            => Some(::websocket::types::room::resources::ResourceType::KeaniumHydride);
        KeaniumHydride => keanium_hydride => "KH"
            => Some(::websocket::types::room::resources::ResourceType::KeaniumOxide);
        KeaniumOxide => keanium_oxide => "KO"
            => Some(::websocket::types::room::resources::ResourceType::LemergiumHydride);
        LemergiumHydride => lemergium_hydride => "LH"
            => Some(::websocket::types::room::resources::ResourceType::LemergiumOxide);
        LemergiumOxide => lemergium_oxide => "LO"
            => Some(::websocket::types::room::resources::ResourceType::ZynthiumHydride);
        ZynthiumHydride => zynthium_hydride => "ZH"
            => Some(::websocket::types::room::resources::ResourceType::ZynthiumOxide);
        ZynthiumOxide => zynthium_oxide => "ZO"
            => Some(::websocket::types::room::resources::ResourceType::GhodiumHydride);
        GhodiumHydride => ghodium_hydride => "GH"
            => Some(::websocket::types::room::resources::ResourceType::GhodiumOxide);
        GhodiumOxide => ghodium_oxide => "GO"
            => Some(::websocket::types::room::resources::ResourceType::UtriumAcid);
        UtriumAcid => utrium_acid => "UH2O"
            => Some(::websocket::types::room::resources::ResourceType::UtriumAlkalide);
        UtriumAlkalide => utrium_alkalide => "UHO2"
            => Some(::websocket::types::room::resources::ResourceType::KeaniumAcid);
        KeaniumAcid => keanium_acid => "KH2O"
            => Some(::websocket::types::room::resources::ResourceType::KeaniumAlkalide);
        KeaniumAlkalide => keanium_alkalide => "KHO2"
            => Some(::websocket::types::room::resources::ResourceType::LemergiumAcid);
        LemergiumAcid => lemergium_acid => "LH2O"
            => Some(::websocket::types::room::resources::ResourceType::LemergiumAlkalide);
        LemergiumAlkalide => lemergium_alkalide => "LHO2"
            => Some(::websocket::types::room::resources::ResourceType::ZynthiumAcid);
        ZynthiumAcid => zynthium_acid => "ZH2O"
            => Some(::websocket::types::room::resources::ResourceType::ZynthiumAlkalide);
        ZynthiumAlkalide => zynthium_alkalide => "ZHO2"
            => Some(::websocket::types::room::resources::ResourceType::GhodiumAcid);
        GhodiumAcid => ghodium_acid => "GH2O"
            => Some(::websocket::types::room::resources::ResourceType::GhodiumAlkalide);
        GhodiumAlkalide => ghodium_alkalide => "GHO2"
            => Some(::websocket::types::room::resources::ResourceType::CatalyzedUtriumAcid);
        CatalyzedUtriumAcid => catalyzed_utrium_acid => "XUH2O"
            => Some(::websocket::types::room::resources::ResourceType::CatalyzedUtriumAlkalide);
        CatalyzedUtriumAlkalide => catalyzed_utrium_alkalide => "XUHO2"
            => Some(::websocket::types::room::resources::ResourceType::CatalyzedKeaniumAcid);
        CatalyzedKeaniumAcid => catalyzed_keanium_acid => "XKH2O"
            => Some(::websocket::types::room::resources::ResourceType::CatalyzedKeaniumAlkalide);
        CatalyzedKeaniumAlkalide => catalyzed_keanium_alkalide => "XKHO2"
            => Some(::websocket::types::room::resources::ResourceType::CatalyzedLemergiumAcid);
        CatalyzedLemergiumAcid => catalyzed_lemergium_acid => "XLH2O"
            => Some(::websocket::types::room::resources::ResourceType::CatalyzedLemergiumAlkalide);
        CatalyzedLemergiumAlkalide => catalyzed_lemergium_alkalide => "XLHO2"
            => Some(::websocket::types::room::resources::ResourceType::CatalyzedZynthiumAcid);
        CatalyzedZynthiumAcid => catalyzed_zynthium_acid => "XZH2O"
            => Some(::websocket::types::room::resources::ResourceType::CatalyzedZynthiumAlkalide);
        CatalyzedZynthiumAlkalide => catalyzed_zynthium_alkalide => "XZHO2"
            => Some(::websocket::types::room::resources::ResourceType::CatalyzedGhodiumAcid);
        CatalyzedGhodiumAcid => catalyzed_ghodium_acid => "XGH2O"
            => Some(::websocket::types::room::resources::ResourceType::CatalyzedGhodiumAlkalide);
        CatalyzedGhodiumAlkalide => catalyzed_ghodium_alkalide => "XGHO2"
            => None;
    }

    /// A container structure - a structure which can store a small number of any combination of
    /// resources, and can be built in any room, but decays over time.
    #[derive(Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureContainer {
        /// Total capacity for this structure.
        #[serde(rename = "energyCapacity")]
        pub capacity: i32,
        /// The next game tick when this structure's hits will decrease naturally.
        pub next_decay_time: u32,
        /// Whether or not an attack on this structure will send an email to the owner automatically.
        pub notify_when_attacked: bool,
    }

    /// The update structure for an extension structure.
    #[derive(Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureContainerUpdate {
        #[serde(rename = "energyCapacity")]
        - capacity: i32,
        - next_decay_time: u32,
        - notify_when_attacked: bool,
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
        ::websocket::types::room::resources::ResourceType;

        Energy => energy => "energy"
            => Some(::websocket::types::room::resources::ResourceType::Power);
        Power => power => "power"
            => Some(::websocket::types::room::resources::ResourceType::Hydrogen);
        Hydrogen => hydrogen => "H"
            => Some(::websocket::types::room::resources::ResourceType::Oxygen);
        Oxygen => oxygen => "O"
            => Some(::websocket::types::room::resources::ResourceType::Utrium);
        Utrium => utrium => "U"
            => Some(::websocket::types::room::resources::ResourceType::Lemergium);
        Lemergium => lemergium => "L"
            => Some(::websocket::types::room::resources::ResourceType::Keanium);
        Keanium => keanium => "K"
            => Some(::websocket::types::room::resources::ResourceType::Zynthium);
        Zynthium => zynthium => "Z"
            => Some(::websocket::types::room::resources::ResourceType::Catalyst);
        Catalyst => catalyst => "X"
            => Some(::websocket::types::room::resources::ResourceType::Ghodium);
        Ghodium => ghodium => "G"
            => Some(::websocket::types::room::resources::ResourceType::Hydroxide);
        Hydroxide => hydroxide => "OH"
            => Some(::websocket::types::room::resources::ResourceType::ZynthiumKeanite);
        ZynthiumKeanite => zynthium_keanite => "ZK"
            => Some(::websocket::types::room::resources::ResourceType::UtriumLemergite);
        UtriumLemergite => utrium_lemergite => "UL"
            => Some(::websocket::types::room::resources::ResourceType::UtriumHydride);
        UtriumHydride => utrium_hydride => "UH"
            => Some(::websocket::types::room::resources::ResourceType::UtriumOxide);
        UtriumOxide => utrium_oxide => "UO"
            => Some(::websocket::types::room::resources::ResourceType::KeaniumHydride);
        KeaniumHydride => keanium_hydride => "KH"
            => Some(::websocket::types::room::resources::ResourceType::KeaniumOxide);
        KeaniumOxide => keanium_oxide => "KO"
            => Some(::websocket::types::room::resources::ResourceType::LemergiumHydride);
        LemergiumHydride => lemergium_hydride => "LH"
            => Some(::websocket::types::room::resources::ResourceType::LemergiumOxide);
        LemergiumOxide => lemergium_oxide => "LO"
            => Some(::websocket::types::room::resources::ResourceType::ZynthiumHydride);
        ZynthiumHydride => zynthium_hydride => "ZH"
            => Some(::websocket::types::room::resources::ResourceType::ZynthiumOxide);
        ZynthiumOxide => zynthium_oxide => "ZO"
            => Some(::websocket::types::room::resources::ResourceType::GhodiumHydride);
        GhodiumHydride => ghodium_hydride => "GH"
            => Some(::websocket::types::room::resources::ResourceType::GhodiumOxide);
        GhodiumOxide => ghodium_oxide => "GO"
            => Some(::websocket::types::room::resources::ResourceType::UtriumAcid);
        UtriumAcid => utrium_acid => "UH2O"
            => Some(::websocket::types::room::resources::ResourceType::UtriumAlkalide);
        UtriumAlkalide => utrium_alkalide => "UHO2"
            => Some(::websocket::types::room::resources::ResourceType::KeaniumAcid);
        KeaniumAcid => keanium_acid => "KH2O"
            => Some(::websocket::types::room::resources::ResourceType::KeaniumAlkalide);
        KeaniumAlkalide => keanium_alkalide => "KHO2"
            => Some(::websocket::types::room::resources::ResourceType::LemergiumAcid);
        LemergiumAcid => lemergium_acid => "LH2O"
            => Some(::websocket::types::room::resources::ResourceType::LemergiumAlkalide);
        LemergiumAlkalide => lemergium_alkalide => "LHO2"
            => Some(::websocket::types::room::resources::ResourceType::ZynthiumAcid);
        ZynthiumAcid => zynthium_acid => "ZH2O"
            => Some(::websocket::types::room::resources::ResourceType::ZynthiumAlkalide);
        ZynthiumAlkalide => zynthium_alkalide => "ZHO2"
            => Some(::websocket::types::room::resources::ResourceType::GhodiumAcid);
        GhodiumAcid => ghodium_acid => "GH2O"
            => Some(::websocket::types::room::resources::ResourceType::GhodiumAlkalide);
        GhodiumAlkalide => ghodium_alkalide => "GHO2"
            => Some(::websocket::types::room::resources::ResourceType::CatalyzedUtriumAcid);
        CatalyzedUtriumAcid => catalyzed_utrium_acid => "XUH2O"
            => Some(::websocket::types::room::resources::ResourceType::CatalyzedUtriumAlkalide);
        CatalyzedUtriumAlkalide => catalyzed_utrium_alkalide => "XUHO2"
            => Some(::websocket::types::room::resources::ResourceType::CatalyzedKeaniumAcid);
        CatalyzedKeaniumAcid => catalyzed_keanium_acid => "XKH2O"
            => Some(::websocket::types::room::resources::ResourceType::CatalyzedKeaniumAlkalide);
        CatalyzedKeaniumAlkalide => catalyzed_keanium_alkalide => "XKHO2"
            => Some(::websocket::types::room::resources::ResourceType::CatalyzedLemergiumAcid);
        CatalyzedLemergiumAcid => catalyzed_lemergium_acid => "XLH2O"
            => Some(::websocket::types::room::resources::ResourceType::CatalyzedLemergiumAlkalide);
        CatalyzedLemergiumAlkalide => catalyzed_lemergium_alkalide => "XLHO2"
            => Some(::websocket::types::room::resources::ResourceType::CatalyzedZynthiumAcid);
        CatalyzedZynthiumAcid => catalyzed_zynthium_acid => "XZH2O"
            => Some(::websocket::types::room::resources::ResourceType::CatalyzedZynthiumAlkalide);
        CatalyzedZynthiumAlkalide => catalyzed_zynthium_alkalide => "XZHO2"
            => Some(::websocket::types::room::resources::ResourceType::CatalyzedGhodiumAcid);
        CatalyzedGhodiumAcid => catalyzed_ghodium_acid => "XGH2O"
            => Some(::websocket::types::room::resources::ResourceType::CatalyzedGhodiumAlkalide);
        CatalyzedGhodiumAlkalide => catalyzed_ghodium_alkalide => "XGHO2"
            => None;
    }

    /// Resource iterator for a `StructureContainer`.
    #[derive(Debug)]
    pub struct ContainerContents(StructureContainer);
}

impl StructureContainer {
    /// Iterates over this structure's resources.
    pub fn resources(&self) -> ContainerContents {
        ContainerContents::new(self)
    }
}

#[cfg(test)]
mod test {
    use serde::Deserialize;

    use super::StructureContainer;
    use websocket::types::room::resources::ResourceType;

    #[test]
    fn parse_container() {
        let json = json!({
            "_id": "58cc8143050a8f701678f22e",
            "energy": 2000,
            "energyCapacity": 2000,
            "hits": 250000,
            "hitsMax": 250000,
            "nextDecayTime": 20233841,
            "notifyWhenAttacked": true,
            "room": "E9N23",
            "type": "container",
            "x": 19,
            "y": 22
        });

        let obj = StructureContainer::deserialize(json).unwrap();

        match obj {
            StructureContainer {
                ghodium_oxide: 0,
                keanium_oxide: 0,
                oxygen: 0,
                keanium: 0,
                energy: 2000,
                capacity: 2000,
                hits: 250000,
                hits_max: 250000,
                next_decay_time: 20233841,
                notify_when_attacked: true,
                x: 19,
                y: 22,
                ref id,
                ..
            } if id == "58cc8143050a8f701678f22e" =>
            {
                ()
            }
            other => panic!("expected pre-set StructureContainer to match, found {:#?}", other),
        }

        assert_eq!(
            {
                let mut contents = obj.resources().collect::<Vec<_>>();
                contents.sort();
                contents
            },
            {
                let mut expected = vec![(ResourceType::Energy, 2000)];
                expected.sort();
                expected
            }
        );
    }
}
