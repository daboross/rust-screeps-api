//! Managing and parsing resource
use std::{cmp, collections::HashMap, fmt};

use serde::{
    de::{Deserializer, MapAccess, Visitor},
    Deserialize, Serialize,
};

use crate::websocket::room_object_macros::Updatable;

macro_rules! resource_types {
    ({
        $(
            $(#[$attrs:meta])*
            $name:ident: $repl:tt,
        )*
    }) => {
        /// All possible resource identifiers in the game.
        #[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[serde(rename_all = "camelCase")]
        pub enum ResourceType {
            $(
                $(#[$attrs])*
                #[serde(rename = $repl)]
                $name,
            )*
        }
        basic_updatable!(ResourceType);

        impl ResourceType {
            /// Finds the in-game resource type string for this resource type.
            ///
            /// Example:
            ///
            /// ```
            /// # use screeps_api::websocket::resources::ResourceType;
            /// assert_eq!(ResourceType::Utrium.to_resource_string(), "U")
            /// ```
            pub fn to_resource_string(&self) -> &'static str {
                match *self {
                    $(
                        ResourceType::$name => $repl,
                    )*
                }
            }
        }
    };
}

resource_types!({
    /// RESOURCE_ENERGY: "energy"
    Energy: "energy",
    /// RESOURCE_POWER: "power"
    Power: "power",

    /// RESOURCE_HYDROGEN: "H"
    Hydrogen: "H",
    /// RESOURCE_OXYGEN: "O"
    Oxygen: "O",
    /// RESOURCE_UTRIUM: "U"
    Utrium: "U",
    /// RESOURCE_LEMERGIUM: "L"
    Lemergium: "L",
    /// RESOURCE_KEANIUM: "K"
    Keanium: "K",
    /// RESOURCE_ZYNTHIUM: "Z"
    Zynthium: "Z",
    /// RESOURCE_CATALYST: "X"
    Catalyst: "X",
    /// RESOURCE_GHODIUM: "G"
    Ghodium: "G",

    /// RESOURCE_SILICON: "silicon"
    Silicon: "silicon",
    /// RESOURCE_METAL: "metal"
    Metal: "metal",
    /// RESOURCE_BIOMASS: "biomass"
    Biomass: "biomass",
    /// RESOURCE_MIST: "mist"
    Mist: "mist",

    /// RESOURCE_HYDROXIDE: "OH"
    Hydroxide: "OH",
    /// RESOURCE_ZYNTHIUM_KEANITE: "ZK"
    ZynthiumKeanite: "ZK",
    /// RESOURCE_UTRIUM_LEMERGITE: "UL"
    UtriumLemergite: "UL",

    /// RESOURCE_UTRIUM_HYDRIDE: "UH"
    UtriumHydride: "UH",
    /// RESOURCE_UTRIUM_OXIDE: "UO"
    UtriumOxide: "UO",
    /// RESOURCE_KEANIUM_HYDRIDE: "KH"
    KeaniumHydride: "KH",
    /// RESOURCE_KEANIUM_OXIDE: "KO"
    KeaniumOxide: "KO",
    /// RESOURCE_LEMERGIUM_HYDRIDE: "LH"
    LemergiumHydride: "LH",
    /// RESOURCE_LEMERGIUM_OXIDE: "LO"
    LemergiumOxide: "LO",
    /// RESOURCE_ZYNTHIUM_HYDRIDE: "ZH"
    ZynthiumHydride: "ZH",
    /// RESOURCE_ZYNTHIUM_OXIDE: "ZO"
    ZynthiumOxide: "ZO",
    /// RESOURCE_GHODIUM_HYDRIDE: "GH"
    GhodiumHydride: "GH",
    /// RESOURCE_GHODIUM_OXIDE: "GO"
    GhodiumOxide: "GO",

    /// RESOURCE_UTRIUM_ACID: "UH2O"
    UtriumAcid: "UH2O",
    /// RESOURCE_UTRIUM_ALKALIDE: "UHO2"
    UtriumAlkalide: "UHO2",
    /// RESOURCE_KEANIUM_ACID: "KH2O"
    KeaniumAcid: "KH2O",
    /// RESOURCE_KEANIUM_ALKALIDE: "KHO2"
    KeaniumAlkalide: "KHO2",
    /// RESOURCE_LEMERGIUM_ACID: "LH2O"
    LemergiumAcid: "LH2O",
    /// RESOURCE_LEMERGIUM_ALKALIDE: "LHO2"
    LemergiumAlkalide: "LHO2",
    /// RESOURCE_ZYNTHIUM_ACID: "ZH2O"
    ZynthiumAcid: "ZH2O",
    /// RESOURCE_ZYNTHIUM_ALKALIDE: "ZHO2"
    ZynthiumAlkalide: "ZHO2",
    /// RESOURCE_GHODIUM_ACID: "GH2O"
    GhodiumAcid: "GH2O",
    /// RESOURCE_GHODIUM_ALKALIDE: "GHO2"
    GhodiumAlkalide: "GHO2",

    /// RESOURCE_CATALYZED_UTRIUM_ACID: "XUH2O"
    CatalyzedUtriumAcid: "XUH2O",
    /// RESOURCE_CATALYZED_UTRIUM_ALKALIDE: "XUHO2"
    CatalyzedUtriumAlkalide: "XUHO2",
    /// RESOURCE_CATALYZED_KEANIUM_ACID: "XKH2O"
    CatalyzedKeaniumAcid: "XKH2O",
    /// RESOURCE_CATALYZED_KEANIUM_ALKALIDE: "XKHO2"
    CatalyzedKeaniumAlkalide: "XKHO2",
    /// RESOURCE_CATALYZED_LEMERGIUM_ACID: "XLH2O"
    CatalyzedLemergiumAcid: "XLH2O",
    /// RESOURCE_CATALYZED_LEMERGIUM_ALKALIDE: "XLHO2"
    CatalyzedLemergiumAlkalide: "XLHO2",
    /// RESOURCE_CATALYZED_ZYNTHIUM_ACID: "XZH2O"
    CatalyzedZynthiumAcid: "XZH2O",
    /// RESOURCE_CATALYZED_ZYNTHIUM_ALKALIDE: "XZHO2"
    CatalyzedZynthiumAlkalide: "XZHO2",
    /// RESOURCE_CATALYZED_GHODIUM_ACID: "XGH2O"
    CatalyzedGhodiumAcid: "XGH2O",
    /// RESOURCE_CATALYZED_GHODIUM_ALKALIDE: "XGHO2"
    CatalyzedGhodiumAlkalide: "XGHO2",

    /// RESOURCE_OPS: "ops"
    Ops: "ops",

    /// RESOURCE_UTRIUM_BAR: "utrium_bar"
    UtriumBar: "utrium_bar",
    /// RESOURCE_LEMERGIUM_BAR: "lemergium_bar"
    LemergiumBar: "lemergium_bar",
    /// RESOURCE_ZYNTHIUM_BAR: "zynthium_bar"
    ZynthiumBar: "zynthium_bar",
    /// RESOURCE_KEANIUM_BAR: "keanium_bar"
    KeaniumBar: "keanium_bar",
    /// RESOURCE_GHODIUM_MELT: "ghodium_melt"
    GhodiumMelt: "ghodium_melt",
    /// RESOURCE_OXIDANT: "oxidant"
    Oxidant: "oxidant",
    /// RESOURCE_REDUCTANT: "reductant"
    Reductant: "reductant",
    /// RESOURCE_PURIFIER: "purifier"
    Purifier: "purifier",
    /// RESOURCE_BATTERY: "battery"
    Battery: "battery",

    /// RESOURCE_COMPOSITE: "composite"
    Composite: "composite",
    /// RESOURCE_CRYSTAL: "crystal"
    Crystal: "crystal",
    /// RESOURCE_LIQUID: "liquid"
    Liquid: "liquid",

    /// RESOURCE_WIRE: "wire"
    Wire: "wire",
    /// RESOURCE_SWITCH: "switch"
    Switch: "switch",
    /// RESOURCE_TRANSISTOR: "transistor"
    Transistor: "transistor",
    /// RESOURCE_MICROCHIP: "microchip"
    Microchip: "microchip",
    /// RESOURCE_CIRCUIT: "circuit"
    Circuit: "circuit",
    /// RESOURCE_DEVICE: "device"
    Device: "device",

    /// RESOURCE_CELL: "cell"
    Cell: "cell",
    /// RESOURCE_PHLEGM: "phlegm"
    Phlegm: "phlegm",
    /// RESOURCE_TISSUE: "tissue"
    Tissue: "tissue",
    /// RESOURCE_MUSCLE: "muscle"
    Muscle: "muscle",
    /// RESOURCE_ORGANOID: "organoid"
    Organoid: "organoid",
    /// RESOURCE_ORGANISM: "organism"
    Organism: "organism",

    /// RESOURCE_ALLOY: "alloy"
    Alloy: "alloy",
    /// RESOURCE_TUBE: "tube"
    Tube: "tube",
    /// RESOURCE_FIXTURES: "fixtures"
    Fixtures: "fixtures",
    /// RESOURCE_FRAME: "frame"
    Frame: "frame",
    /// RESOURCE_HYDRAULICS: "hydraulics"
    Hydraulics: "hydraulics",
    /// RESOURCE_MACHINE: "machine"
    Machine: "machine",

    /// RESOURCE_CONDENSATE: "condensate"
    Condensate: "condensate",
    /// RESOURCE_CONCENTRATE: "concentrate"
    Concentrate: "concentrate",
    /// RESOURCE_EXTRACT: "extract"
    Extract: "extract",
    /// RESOURCE_SPIRIT: "spirit"
    Spirit: "spirit",
    /// RESOURCE_EMANATION: "emanation"
    Emanation: "emanation",
    /// RESOURCE_ESSENCE: "essence"
    Essence: "essence",
});

/// The resources and amounts of each resource some game object holds.
#[derive(Serialize, Clone, Debug, Default, PartialEq, Eq)]
#[serde(transparent)]
pub struct Store(pub HashMap<ResourceType, i32>);

impl Store {
    /// Iterate over the contents of this store.
    pub fn iter(&self) -> impl Iterator<Item = (ResourceType, i32)> + '_ {
        self.0.iter().map(|(k, v)| (*k, *v))
    }

    /// Get the amount of a specific resource in this store.
    pub fn get(&self, resource: ResourceType) -> i32 {
        self.0.get(&resource).copied().unwrap_or(0)
    }
}

struct StoreVisitor;

impl<'de> Visitor<'de> for StoreVisitor {
    type Value = Store;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a map")
    }

    #[inline]
    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        // Adopted from `HashMap` `Deserialize` impl from
        // https://github.com/serde-rs/serde/blob/master/serde/src/de/impls.rs
        let mut values = HashMap::with_capacity(cmp::min(map.size_hint().unwrap_or(0), 4096));

        while let Some((key, value)) = map.next_entry::<_, Option<i32>>()? {
            let value = value.unwrap_or(0);
            if value != 0 {
                values.insert(key, value);
            }
        }

        Ok(Store(values))
    }
}

impl<'de> Deserialize<'de> for Store {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(StoreVisitor)
    }
}

/// Update structure for Store. The difference is that StoreUpdate allows 0 values.
#[derive(Serialize, Clone, Debug, Default, PartialEq, Eq)]
#[serde(transparent)]
pub struct StoreUpdate(pub HashMap<ResourceType, i32>);

/// Like `StoreVisitor`, but keeps 0s and nulls.
struct StoreUpdateVisitor;

impl<'de> Visitor<'de> for StoreUpdateVisitor {
    type Value = StoreUpdate;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a map")
    }

    #[inline]
    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        // Adopted from `HashMap` `Deserialize` impl from
        // https://github.com/serde-rs/serde/blob/master/serde/src/de/impls.rs
        let mut values = HashMap::with_capacity(cmp::min(map.size_hint().unwrap_or(0), 4096));

        while let Some((key, value)) = map.next_entry::<_, Option<i32>>()? {
            let value = value.unwrap_or(0);
            values.insert(key, value);
        }

        Ok(StoreUpdate(values))
    }
}

impl<'de> Deserialize<'de> for StoreUpdate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(StoreUpdateVisitor)
    }
}

impl Updatable for Store {
    type Update = StoreUpdate;
    fn apply_update(&mut self, update: Self::Update) {
        for (key, value) in update.0 {
            if value == 0 {
                self.0.remove(&key);
            } else {
                self.0.insert(key, value);
            }
        }
    }

    fn create_from_update(update: Self::Update) -> Option<Self> {
        let mut values = update.0;
        values.retain(|_k, v| *v != 0);
        Some(Store(values))
    }
}

#[cfg(test)]
macro_rules! store {
    ($($name:ident: $val:expr),*$(,)?) => (
        {
            #[allow(unused_mut)]
            let mut store = crate::websocket::types::room::resources::Store::default();

            $(
                store.0.insert(crate::websocket::types::room::resources::ResourceType::$name, $val);
            )*

            store
        }
    );
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
//         print(last_line.strip() + '|Some(::websocket::resources::ResourceType::' + new_split[0] + ')')
//     last_line = line
// print(last_line.strip() + '|None')
// ```
