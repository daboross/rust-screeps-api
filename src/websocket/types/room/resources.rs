//! Managing and parsing resource

/// All possible resource identifiers in the game.
#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ResourceType {
    /// RESOURCE_ENERGY: "energy",
    #[serde(rename = "energy")]
    Energy,
    /// RESOURCE_POWER: "power",
    #[serde(rename = "power")]
    Power,
    /// RESOURCE_HYDROGEN: "H",
    #[serde(rename = "H")]
    Hydrogen,
    /// RESOURCE_OXYGEN: "O",
    #[serde(rename = "O")]
    Oxygen,
    /// RESOURCE_UTRIUM: "U",
    #[serde(rename = "U")]
    Utrium,
    /// RESOURCE_LEMERGIUM: "L",
    #[serde(rename = "L")]
    Lemergium,
    /// RESOURCE_KEANIUM: "K",
    #[serde(rename = "K")]
    Keanium,
    /// RESOURCE_ZYNTHIUM: "Z",
    #[serde(rename = "Z")]
    Zynthium,
    /// RESOURCE_CATALYST: "X",
    #[serde(rename = "X")]
    Catalyst,
    /// RESOURCE_GHODIUM: "G",
    #[serde(rename = "G")]
    Ghodium,
    /// RESOURCE_HYDROXIDE: "OH",
    #[serde(rename = "OH")]
    Hydroxide,
    /// RESOURCE_ZYNTHIUM_KEANITE: "ZK",
    #[serde(rename = "ZK")]
    ZynthiumKeanite,
    /// RESOURCE_UTRIUM_LEMERGITE: "UL",
    #[serde(rename = "UL")]
    UtriumLemergite,
    /// RESOURCE_UTRIUM_HYDRIDE: "UH",
    #[serde(rename = "UH")]
    UtriumHydride,
    /// RESOURCE_UTRIUM_OXIDE: "UO",
    #[serde(rename = "UO")]
    UtriumOxide,
    /// RESOURCE_KEANIUM_HYDRIDE: "KH",
    #[serde(rename = "KH")]
    KeaniumHydride,
    /// RESOURCE_KEANIUM_OXIDE: "KO",
    #[serde(rename = "KO")]
    KeaniumOxide,
    /// RESOURCE_LEMERGIUM_HYDRIDE: "LH",
    #[serde(rename = "LH")]
    LemergiumHydride,
    /// RESOURCE_LEMERGIUM_OXIDE: "LO",
    #[serde(rename = "LO")]
    LemergiumOxide,
    /// RESOURCE_ZYNTHIUM_HYDRIDE: "ZH",
    #[serde(rename = "ZH")]
    ZynthiumHydride,
    /// RESOURCE_ZYNTHIUM_OXIDE: "ZO",
    #[serde(rename = "ZO")]
    ZynthiumOxide,
    /// RESOURCE_GHODIUM_HYDRIDE: "GH",
    #[serde(rename = "GH")]
    GhodiumHydride,
    /// RESOURCE_GHODIUM_OXIDE: "GO",
    #[serde(rename = "GO")]
    GhodiumOxide,
    /// RESOURCE_UTRIUM_ACID: "UH2O",
    #[serde(rename = "UH2O")]
    UtriumAcid,
    /// RESOURCE_UTRIUM_ALKALIDE: "UHO2",
    #[serde(rename = "UHO2")]
    UtriumAlkalide,
    /// RESOURCE_KEANIUM_ACID: "KH2O",
    #[serde(rename = "KH2O")]
    KeaniumAcid,
    /// RESOURCE_KEANIUM_ALKALIDE: "KHO2",
    #[serde(rename = "KHO2")]
    KeaniumAlkalide,
    /// RESOURCE_LEMERGIUM_ACID: "LH2O",
    #[serde(rename = "LH2O")]
    LemergiumAcid,
    /// RESOURCE_LEMERGIUM_ALKALIDE: "LHO2",
    #[serde(rename = "LHO2")]
    LemergiumAlkalide,
    /// RESOURCE_ZYNTHIUM_ACID: "ZH2O",
    #[serde(rename = "ZH2O")]
    ZynthiumAcid,
    /// RESOURCE_ZYNTHIUM_ALKALIDE: "ZHO2",
    #[serde(rename = "ZHO2")]
    ZynthiumAlkalide,
    /// RESOURCE_GHODIUM_ACID: "GH2O",
    #[serde(rename = "GH2O")]
    GhodiumAcid,
    /// RESOURCE_GHODIUM_ALKALIDE: "GHO2",
    #[serde(rename = "GHO2")]
    GhodiumAlkalide,
    /// RESOURCE_CATALYZED_UTRIUM_ACID: "XUH2O",
    #[serde(rename = "XUH2O")]
    CatalyzedUtriumAcid,
    /// RESOURCE_CATALYZED_UTRIUM_ALKALIDE: "XUHO2",
    #[serde(rename = "XUHO2")]
    CatalyzedUtriumAlkalide,
    /// RESOURCE_CATALYZED_KEANIUM_ACID: "XKH2O",
    #[serde(rename = "XKH2O")]
    CatalyzedKeaniumAcid,
    /// RESOURCE_CATALYZED_KEANIUM_ALKALIDE: "XKHO2",
    #[serde(rename = "XKHO2")]
    CatalyzedKeaniumAlkalide,
    /// RESOURCE_CATALYZED_LEMERGIUM_ACID: "XLH2O",
    #[serde(rename = "XLH2O")]
    CatalyzedLemergiumAcid,
    /// RESOURCE_CATALYZED_LEMERGIUM_ALKALIDE: "XLHO2",
    #[serde(rename = "XLHO2")]
    CatalyzedLemergiumAlkalide,
    /// RESOURCE_CATALYZED_ZYNTHIUM_ACID: "XZH2O",
    #[serde(rename = "XZH2O")]
    CatalyzedZynthiumAcid,
    /// RESOURCE_CATALYZED_ZYNTHIUM_ALKALIDE: "XZHO2",
    #[serde(rename = "XZHO2")]
    CatalyzedZynthiumAlkalide,
    /// RESOURCE_CATALYZED_GHODIUM_ACID: "XGH2O",
    #[serde(rename = "XGH2O")]
    CatalyzedGhodiumAcid,
    /// RESOURCE_CATALYZED_GHODIUM_ALKALIDE: "XGHO2",
    #[serde(rename = "XGHO2")]
    CatalyzedGhodiumAlkalide,
}

basic_updatable!(ResourceType);


impl ResourceType {
    // created by replacing:
    // `s#/// [A-Z_]+: "(\w+)",\n            (\w+),#ResourceType::$2 => "$1",#g`
    // (original is the definition for the enum)

    /// Finds the in-game resource type string for this resource type.
    ///
    /// Example:
    ///
    /// ```
    /// # use screeps_api::websocket::types::room::resources::ResourceType;
    /// assert_eq!(ResourceType::Utrium.to_resource_string(), "U")
    /// ```
    pub fn to_resource_string(&self) -> &'static str {
        match *self {
            ResourceType::Energy => "energy",
            ResourceType::Power => "power",
            ResourceType::Hydrogen => "H",
            ResourceType::Oxygen => "O",
            ResourceType::Utrium => "U",
            ResourceType::Lemergium => "L",
            ResourceType::Keanium => "K",
            ResourceType::Zynthium => "Z",
            ResourceType::Catalyst => "X",
            ResourceType::Ghodium => "G",
            ResourceType::Hydroxide => "OH",
            ResourceType::ZynthiumKeanite => "ZK",
            ResourceType::UtriumLemergite => "UL",
            ResourceType::UtriumHydride => "UH",
            ResourceType::UtriumOxide => "UO",
            ResourceType::KeaniumHydride => "KH",
            ResourceType::KeaniumOxide => "KO",
            ResourceType::LemergiumHydride => "LH",
            ResourceType::LemergiumOxide => "LO",
            ResourceType::ZynthiumHydride => "ZH",
            ResourceType::ZynthiumOxide => "ZO",
            ResourceType::GhodiumHydride => "GH",
            ResourceType::GhodiumOxide => "GO",
            ResourceType::UtriumAcid => "UH2O",
            ResourceType::UtriumAlkalide => "UHO2",
            ResourceType::KeaniumAcid => "KH2O",
            ResourceType::KeaniumAlkalide => "KHO2",
            ResourceType::LemergiumAcid => "LH2O",
            ResourceType::LemergiumAlkalide => "LHO2",
            ResourceType::ZynthiumAcid => "ZH2O",
            ResourceType::ZynthiumAlkalide => "ZHO2",
            ResourceType::GhodiumAcid => "GH2O",
            ResourceType::GhodiumAlkalide => "GHO2",
            ResourceType::CatalyzedUtriumAcid => "XUH2O",
            ResourceType::CatalyzedUtriumAlkalide => "XUHO2",
            ResourceType::CatalyzedKeaniumAcid => "XKH2O",
            ResourceType::CatalyzedKeaniumAlkalide => "XKHO2",
            ResourceType::CatalyzedLemergiumAcid => "XLH2O",
            ResourceType::CatalyzedLemergiumAlkalide => "XLHO2",
            ResourceType::CatalyzedZynthiumAcid => "XZH2O",
            ResourceType::CatalyzedZynthiumAlkalide => "XZHO2",
            ResourceType::CatalyzedGhodiumAcid => "XGH2O",
            ResourceType::CatalyzedGhodiumAlkalide => "XGHO2",
        }
    }
}


/// This macro creates the struct described within the invocation, but with an additional 2 fields common to all
/// Structures, with everything provided by `with_base_fields_and_update_struct!`, and with one field per in-game
/// resource type.
///
/// Since defining nested macros is not allowed (https://github.com/rust-lang/rust/issues/35853), it's best,
/// unfortunately, to copy-paste all resource names into the macro.`
macro_rules! with_resource_fields_and_update_struct {
    (
        {
            $full_resource_type:path;

            $(
                $resource_type_ident:ident => $field_ident:ident => $serde_ident:tt => $next_iterator_type:expr;
            )*
        }

        $( #[$struct_attr:meta] )*
        pub struct $name:ident {
            $(
                $(#[$field_attr:meta])*
                pub $field:ident : $type:ty,
            )*
        }

        $( #[$update_struct_attr:meta] )*
        $( ($update_extra:tt) )*
        pub struct $update_name:ident { ... }
    ) => (
        with_resource_fields_and_update_struct! {
            {
                $full_resource_type;

                $(
                    $resource_type_ident => $field_ident => $serde_ident => $next_iterator_type;
                )*
            }

            $( #[$struct_attr] )*
            pub struct $name {
                $(
                    $( #[$field_attr] )*
                    pub $field : $type,
                )*
            }

            $( #[$update_struct_attr] )*
            $( ($update_extra) )*
            pub struct $update_name {
                $(
                    $( #[$field_attr] )*
                    - $field : $type,
                )*
            }
        }
    );
    (
        {
            $full_resource_type: path;

            $(
                $resource_type_ident:ident => $field_ident:ident => $serde_ident:tt => $next_iterator_type:expr;
            )*
        }

        $( #[$struct_attr:meta] )*
        pub struct $name:ident {
            $( $struct_field:tt )*
        }

        $( #[$update_struct_attr:meta] )*
        $( ($update_extra:tt) )*
        pub struct $update_name:ident {
            $( $update_field:tt )*
        }
    ) => (
        with_base_fields_and_update_struct! {
            $( #[$struct_attr] )*
            pub struct $name {
                /// The current number of hit-points this structure has.
                pub hits: i32,
                /// The maximum number of hit-points this structure has.
                #[serde(rename = "hitsMax")]
                pub hits_max: i32,
                $(
                    /// The current amount of this resource held in this structure.
                    #[serde(default, rename = $serde_ident)]
                    pub $field_ident: i32,
                )*
                $( $struct_field )*
            }

            $( #[$update_struct_attr] )*
            $( ($update_extra) )*
            pub struct $update_name {
                - hits: i32,
                #[serde(rename = "hitsMax")]
                - hits_max: i32,
                $(
                    #[serde(rename = $serde_ident)]
                    - $field_ident: i32,
                )*
                $( $update_field )*
            }
        }
    )
}


macro_rules! resource_iterator_for {
    (
        {
            $full_resource_type: path;

            $(
                $resource_type_ident:ident => $field_ident:ident => $serde_ident:tt => $next_iterator_type:expr;
            )*
        }


        $( #[$struct_attr:meta] )*
        pub struct $name:ident ( $from:ident );
    ) => (
        $( #[$struct_attr] )*
        pub struct $name<'a> {
            source: &'a $from,
            current_resource: Option<$full_resource_type>,
        }

        impl<'a> $name<'a> {
            fn new(input: &'a $from) -> Self {
                use $full_resource_type::*;
                $name {
                    source: input,
                    current_resource: Some(Energy)
                }
            }
        }

        impl<'a> ::std::iter::Iterator for $name<'a> {
            type Item = ($full_resource_type, i32);

            fn next(&mut self) -> Option<Self::Item> {
                use $full_resource_type::*;
                while let Some(to_check) = self.current_resource {
                    let (amount, next) = match to_check {
                        $(
                            $resource_type_ident => {
                                (self.source.$field_ident, $next_iterator_type)
                            }
                        )*
                    };

                    self.current_resource = next;
                    if amount > 0 {
                        return Some((to_check, amount));
                    }
                }

                None
            }
        }
    )
}

// creating this:
// ```python
// import fileinput
// last_line = None
//
// for line in fileinput.input():
//     if last_line is not None:
//         new_split = line.split('|')
//
//         print(last_line.strip() + '|Some(::websocket::types::room::resources::ResourceType::' + new_split[0] + ')')
//     last_line = line
// print(last_line.strip() + '|None')
// ```

#[allow(unused_macros)]
macro_rules! resource_list {
    () => (
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
    )
}
