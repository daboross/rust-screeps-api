//! `StructureTerminal` data description.
use crate::data::RoomName;

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

    /// A terminal structure - a structure that has a medium capacity for storing multiple resources,
    /// and can spend energy to send any resources to another room with a terminal instantly.
    // Note: there's a field "send" which seems to always be null. It will be non-existent on a
    // terminal which has never sent any resources, and then will be updated to "null" on the first send.
    #[derive(Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureTerminal {
        /// The user ID of the owner of this structure.
        pub user: String,
        /// Whether or not this structure is non-functional due to a degraded controller.
        #[serde(default, rename = "off")]
        pub disabled: bool,
        /// Total capacity for this structure.
        #[serde(rename = "energyCapacity")]
        pub capacity: i32,
        /// The game time at which this terminal will next be able to send minerals.
        pub cooldown_time: u32,
        /// Whether or not an attack on this structure will send an email to the owner automatically.
        pub notify_when_attacked: bool,
    }

    /// The update structure for an extension structure.
    #[derive(Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct StructureTerminalUpdate {
        - user: String,
        #[serde(rename = "off")]
        - disabled: bool,
        #[serde(rename = "energyCapacity")]
        - capacity: i32,
        - cooldown_time: u32,
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

    /// Resource iterator for a `StructureTerminal`.
    #[derive(Debug)]
    pub struct TerminalContents(StructureTerminal);
}

impl StructureTerminal {
    /// Iterates over this structure's resources.
    pub fn resources(&self) -> TerminalContents {
        TerminalContents::new(self)
    }
}

#[cfg(test)]
mod test {
    use serde::Deserialize;

    use super::StructureTerminal;
    use crate::websocket::types::room::resources::ResourceType;

    #[test]
    fn parse_terminal() {
        let json = json!({
            "XLHO2": 0,
            "XZH2O": 0,
            "_id": "59675fbfb554456b195342e7",
            "cooldownTime": 20236024,
            "energy": 1321,
            "energyCapacity": 300000,
            "hits": 3000,
            "hitsMax": 3000,
            "notifyWhenAttacked": true,
            "room": "W65N19",
            "type": "terminal",
            "user": "57874d42d0ae911e3bd15bbc",
            "x": 30,
            "y": 35
        });

        let obj = StructureTerminal::deserialize(json).unwrap();

        match obj {
            StructureTerminal {
                ghodium_oxide: 0,
                keanium_oxide: 0,
                oxygen: 0,
                keanium: 0,
                energy: 1321,
                capacity: 300000,
                hits: 3000,
                hits_max: 3000,
                notify_when_attacked: true,
                cooldown_time: 20236024,
                disabled: false,
                x: 30,
                y: 35,
                ref user,
                ref id,
                ..
            } if user == "57874d42d0ae911e3bd15bbc" && id == "59675fbfb554456b195342e7" => (),
            other => panic!(
                "expected pre-set StructureTerminal to match, found {:#?}",
                other
            ),
        }

        assert_eq!(
            {
                let mut contents = obj.resources().collect::<Vec<_>>();
                contents.sort();
                contents
            },
            {
                let mut expected = vec![(ResourceType::Energy, 1321)];
                expected.sort();
                expected
            }
        );
    }
}
