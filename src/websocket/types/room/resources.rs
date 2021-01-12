//! Managing and parsing resource
use std::{cmp, collections::HashMap, fmt};

use serde::{
    de::{Deserializer, MapAccess, Visitor},
    Deserialize, Serialize,
};

use crate::websocket::room_object_macros::Updatable;

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
    /// # use screeps_api::websocket::resources::ResourceType;
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
    type Update = Store;
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
