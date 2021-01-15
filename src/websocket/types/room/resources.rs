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
    RESOURCE_ENERGY: "energy",
    /// RESOURCE_POWER: "power"
    RESOURCE_POWER: "power",

    /// RESOURCE_HYDROGEN: "H"
    RESOURCE_HYDROGEN: "H",
    /// RESOURCE_OXYGEN: "O"
    RESOURCE_OXYGEN: "O",
    /// RESOURCE_UTRIUM: "U"
    RESOURCE_UTRIUM: "U",
    /// RESOURCE_LEMERGIUM: "L"
    RESOURCE_LEMERGIUM: "L",
    /// RESOURCE_KEANIUM: "K"
    RESOURCE_KEANIUM: "K",
    /// RESOURCE_ZYNTHIUM: "Z"
    RESOURCE_ZYNTHIUM: "Z",
    /// RESOURCE_CATALYST: "X"
    RESOURCE_CATALYST: "X",
    /// RESOURCE_GHODIUM: "G"
    RESOURCE_GHODIUM: "G",

    /// RESOURCE_SILICON: "silicon"
    RESOURCE_SILICON: "silicon",
    /// RESOURCE_METAL: "metal"
    RESOURCE_METAL: "metal",
    /// RESOURCE_BIOMASS: "biomass"
    RESOURCE_BIOMASS: "biomass",
    /// RESOURCE_MIST: "mist"
    RESOURCE_MIST: "mist",

    /// RESOURCE_HYDROXIDE: "OH"
    RESOURCE_HYDROXIDE: "OH",
    /// RESOURCE_ZYNTHIUM_KEANITE: "ZK"
    RESOURCE_ZYNTHIUM_KEANITE: "ZK",
    /// RESOURCE_UTRIUM_LEMERGITE: "UL"
    RESOURCE_UTRIUM_LEMERGITE: "UL",

    /// RESOURCE_UTRIUM_HYDRIDE: "UH"
    RESOURCE_UTRIUM_HYDRIDE: "UH",
    /// RESOURCE_UTRIUM_OXIDE: "UO"
    RESOURCE_UTRIUM_OXIDE: "UO",
    /// RESOURCE_KEANIUM_HYDRIDE: "KH"
    RESOURCE_KEANIUM_HYDRIDE: "KH",
    /// RESOURCE_KEANIUM_OXIDE: "KO"
    RESOURCE_KEANIUM_OXIDE: "KO",
    /// RESOURCE_LEMERGIUM_HYDRIDE: "LH"
    RESOURCE_LEMERGIUM_HYDRIDE: "LH",
    /// RESOURCE_LEMERGIUM_OXIDE: "LO"
    RESOURCE_LEMERGIUM_OXIDE: "LO",
    /// RESOURCE_ZYNTHIUM_HYDRIDE: "ZH"
    RESOURCE_ZYNTHIUM_HYDRIDE: "ZH",
    /// RESOURCE_ZYNTHIUM_OXIDE: "ZO"
    RESOURCE_ZYNTHIUM_OXIDE: "ZO",
    /// RESOURCE_GHODIUM_HYDRIDE: "GH"
    RESOURCE_GHODIUM_HYDRIDE: "GH",
    /// RESOURCE_GHODIUM_OXIDE: "GO"
    RESOURCE_GHODIUM_OXIDE: "GO",

    /// RESOURCE_UTRIUM_ACID: "UH2O"
    RESOURCE_UTRIUM_ACID: "UH2O",
    /// RESOURCE_UTRIUM_ALKALIDE: "UHO2"
    RESOURCE_UTRIUM_ALKALIDE: "UHO2",
    /// RESOURCE_KEANIUM_ACID: "KH2O"
    RESOURCE_KEANIUM_ACID: "KH2O",
    /// RESOURCE_KEANIUM_ALKALIDE: "KHO2"
    RESOURCE_KEANIUM_ALKALIDE: "KHO2",
    /// RESOURCE_LEMERGIUM_ACID: "LH2O"
    RESOURCE_LEMERGIUM_ACID: "LH2O",
    /// RESOURCE_LEMERGIUM_ALKALIDE: "LHO2"
    RESOURCE_LEMERGIUM_ALKALIDE: "LHO2",
    /// RESOURCE_ZYNTHIUM_ACID: "ZH2O"
    RESOURCE_ZYNTHIUM_ACID: "ZH2O",
    /// RESOURCE_ZYNTHIUM_ALKALIDE: "ZHO2"
    RESOURCE_ZYNTHIUM_ALKALIDE: "ZHO2",
    /// RESOURCE_GHODIUM_ACID: "GH2O"
    RESOURCE_GHODIUM_ACID: "GH2O",
    /// RESOURCE_GHODIUM_ALKALIDE: "GHO2"
    RESOURCE_GHODIUM_ALKALIDE: "GHO2",

    /// RESOURCE_CATALYZED_UTRIUM_ACID: "XUH2O"
    RESOURCE_CATALYZED_UTRIUM_ACID: "XUH2O",
    /// RESOURCE_CATALYZED_UTRIUM_ALKALIDE: "XUHO2"
    RESOURCE_CATALYZED_UTRIUM_ALKALIDE: "XUHO2",
    /// RESOURCE_CATALYZED_KEANIUM_ACID: "XKH2O"
    RESOURCE_CATALYZED_KEANIUM_ACID: "XKH2O",
    /// RESOURCE_CATALYZED_KEANIUM_ALKALIDE: "XKHO2"
    RESOURCE_CATALYZED_KEANIUM_ALKALIDE: "XKHO2",
    /// RESOURCE_CATALYZED_LEMERGIUM_ACID: "XLH2O"
    RESOURCE_CATALYZED_LEMERGIUM_ACID: "XLH2O",
    /// RESOURCE_CATALYZED_LEMERGIUM_ALKALIDE: "XLHO2"
    RESOURCE_CATALYZED_LEMERGIUM_ALKALIDE: "XLHO2",
    /// RESOURCE_CATALYZED_ZYNTHIUM_ACID: "XZH2O"
    RESOURCE_CATALYZED_ZYNTHIUM_ACID: "XZH2O",
    /// RESOURCE_CATALYZED_ZYNTHIUM_ALKALIDE: "XZHO2"
    RESOURCE_CATALYZED_ZYNTHIUM_ALKALIDE: "XZHO2",
    /// RESOURCE_CATALYZED_GHODIUM_ACID: "XGH2O"
    RESOURCE_CATALYZED_GHODIUM_ACID: "XGH2O",
    /// RESOURCE_CATALYZED_GHODIUM_ALKALIDE: "XGHO2"
    RESOURCE_CATALYZED_GHODIUM_ALKALIDE: "XGHO2",

    /// RESOURCE_OPS: "ops"
    RESOURCE_OPS: "ops",

    /// RESOURCE_UTRIUM_BAR: "utrium_bar"
    RESOURCE_UTRIUM_BAR: "utrium_bar",
    /// RESOURCE_LEMERGIUM_BAR: "lemergium_bar"
    RESOURCE_LEMERGIUM_BAR: "lemergium_bar",
    /// RESOURCE_ZYNTHIUM_BAR: "zynthium_bar"
    RESOURCE_ZYNTHIUM_BAR: "zynthium_bar",
    /// RESOURCE_KEANIUM_BAR: "keanium_bar"
    RESOURCE_KEANIUM_BAR: "keanium_bar",
    /// RESOURCE_GHODIUM_MELT: "ghodium_melt"
    RESOURCE_GHODIUM_MELT: "ghodium_melt",
    /// RESOURCE_OXIDANT: "oxidant"
    RESOURCE_OXIDANT: "oxidant",
    /// RESOURCE_REDUCTANT: "reductant"
    RESOURCE_REDUCTANT: "reductant",
    /// RESOURCE_PURIFIER: "purifier"
    RESOURCE_PURIFIER: "purifier",
    /// RESOURCE_BATTERY: "battery"
    RESOURCE_BATTERY: "battery",

    /// RESOURCE_COMPOSITE: "composite"
    RESOURCE_COMPOSITE: "composite",
    /// RESOURCE_CRYSTAL: "crystal"
    RESOURCE_CRYSTAL: "crystal",
    /// RESOURCE_LIQUID: "liquid"
    RESOURCE_LIQUID: "liquid",

    /// RESOURCE_WIRE: "wire"
    RESOURCE_WIRE: "wire",
    /// RESOURCE_SWITCH: "switch"
    RESOURCE_SWITCH: "switch",
    /// RESOURCE_TRANSISTOR: "transistor"
    RESOURCE_TRANSISTOR: "transistor",
    /// RESOURCE_MICROCHIP: "microchip"
    RESOURCE_MICROCHIP: "microchip",
    /// RESOURCE_CIRCUIT: "circuit"
    RESOURCE_CIRCUIT: "circuit",
    /// RESOURCE_DEVICE: "device"
    RESOURCE_DEVICE: "device",

    /// RESOURCE_CELL: "cell"
    RESOURCE_CELL: "cell",
    /// RESOURCE_PHLEGM: "phlegm"
    RESOURCE_PHLEGM: "phlegm",
    /// RESOURCE_TISSUE: "tissue"
    RESOURCE_TISSUE: "tissue",
    /// RESOURCE_MUSCLE: "muscle"
    RESOURCE_MUSCLE: "muscle",
    /// RESOURCE_ORGANOID: "organoid"
    RESOURCE_ORGANOID: "organoid",
    /// RESOURCE_ORGANISM: "organism"
    RESOURCE_ORGANISM: "organism",

    /// RESOURCE_ALLOY: "alloy"
    RESOURCE_ALLOY: "alloy",
    /// RESOURCE_TUBE: "tube"
    RESOURCE_TUBE: "tube",
    /// RESOURCE_FIXTURES: "fixtures"
    RESOURCE_FIXTURES: "fixtures",
    /// RESOURCE_FRAME: "frame"
    RESOURCE_FRAME: "frame",
    /// RESOURCE_HYDRAULICS: "hydraulics"
    RESOURCE_HYDRAULICS: "hydraulics",
    /// RESOURCE_MACHINE: "machine"
    RESOURCE_MACHINE: "machine",

    /// RESOURCE_CONDENSATE: "condensate"
    RESOURCE_CONDENSATE: "condensate",
    /// RESOURCE_CONCENTRATE: "concentrate"
    RESOURCE_CONCENTRATE: "concentrate",
    /// RESOURCE_EXTRACT: "extract"
    RESOURCE_EXTRACT: "extract",
    /// RESOURCE_SPIRIT: "spirit"
    RESOURCE_SPIRIT: "spirit",
    /// RESOURCE_EMANATION: "emanation"
    RESOURCE_EMANATION: "emanation",
    /// RESOURCE_ESSENCE: "essence"
    RESOURCE_ESSENCE: "essence",
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
