//! Update parsing for spare "map view" room updates
use std::{
    marker::PhantomData,
    {cmp, fmt},
};

use serde::{
    de::{DeserializeSeed, Error, MapAccess, SeqAccess, Visitor},
    {Deserialize, Deserializer},
};

/// "Map view" room status update. This contains all entities in a given room,
/// organized by what type of thing they are, or who owns them.
#[derive(Default, Clone, Hash, Debug)]
pub struct RoomMapViewUpdate {
    /// Constructed walls in the room. Does not include terrain.
    pub walls: Vec<(u32, u32)>,
    /// All roads in the room.
    pub roads: Vec<(u32, u32)>,
    /// All power banks and dropped power in the room.
    pub power_or_power_bank: Vec<(u32, u32)>,
    /// All portal squares in the room.
    pub portals: Vec<(u32, u32)>,
    /// All energy sources in the room.
    pub sources: Vec<(u32, u32)>,
    /// All mineral deposits in the room.
    pub minerals: Vec<(u32, u32)>,
    /// All controllers in the room.
    pub controllers: Vec<(u32, u32)>,
    /// All keeper's lairs in the room.
    pub keeper_lairs: Vec<(u32, u32)>,
    /// All users who own objects in the room along with all owned objects for each user.
    ///
    /// This is a Vec of (user_id, owned_objects_of_that_user). The game does not provide
    /// more information on what type of object (creep or building, or type of building).
    pub users_objects: Vec<(String, Vec<(u32, u32)>)>,
    /// Phantom data in order to allow adding any additional fields in the future.
    _phantom: PhantomData<()>,
}

struct RoomMapViewUpdateVisitor {
    marker: PhantomData<RoomMapViewUpdate>,
}

impl RoomMapViewUpdateVisitor {
    pub fn new() -> Self {
        RoomMapViewUpdateVisitor {
            marker: PhantomData,
        }
    }
}

struct StrOrU32Seed;

impl<'de> DeserializeSeed<'de> for StrOrU32Seed {
    type Value = u32;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        crate::decoders::u32_or_str_containing::deserialize(deserializer)
    }
}

struct StrOrU32TupleVisitor;

impl<'de> Visitor<'de> for StrOrU32TupleVisitor {
    type Value = (u32, u32);

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a tuple of size 2")
    }

    #[inline]
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let first = match seq.next_element_seed(StrOrU32Seed)? {
            Some(v) => v,
            None => return Err(A::Error::invalid_length(0, &self)),
        };
        let second = match seq.next_element_seed(StrOrU32Seed)? {
            Some(v) => v,
            None => return Err(A::Error::invalid_length(1, &self)),
        };
        Ok((first, second))
    }
}

struct StrOrU32TupleSeed;

impl<'de> DeserializeSeed<'de> for StrOrU32TupleSeed {
    type Value = (u32, u32);

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_tuple(2, StrOrU32TupleVisitor)
    }
}

struct StrOrU32TupleVecVisitor;

impl<'de> Visitor<'de> for StrOrU32TupleVecVisitor {
    type Value = Vec<(u32, u32)>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a sequence")
    }

    #[inline]
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut values = Vec::with_capacity(::std::cmp::min(seq.size_hint().unwrap_or(0), 4069));

        while let Some(value) = seq.next_element_seed(StrOrU32TupleSeed)? {
            values.push(value);
        }
        Ok(values)
    }
}

struct StrOrU32TupleVecSeed;

impl<'de> DeserializeSeed<'de> for StrOrU32TupleVecSeed {
    type Value = Vec<(u32, u32)>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(StrOrU32TupleVecVisitor)
    }
}

impl<'de> Visitor<'de> for RoomMapViewUpdateVisitor {
    type Value = RoomMapViewUpdate;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a map")
    }

    #[inline]
    fn visit_unit<E>(self) -> Result<Self::Value, E> {
        Ok(RoomMapViewUpdate::default())
    }

    #[inline]
    fn visit_map<T>(self, mut access: T) -> Result<Self::Value, T::Error>
    where
        T: MapAccess<'de>,
    {
        let mut walls = None;
        let mut roads = None;
        let mut power_or_power_bank = None;
        let mut portals = None;
        let mut sources = None;
        let mut minerals = None;
        let mut controllers = None;
        let mut keeper_lairs = None;
        // there are 8 expected keys, any extra are user ids
        let mut users_objects =
            Vec::with_capacity(cmp::max(cmp::min(access.size_hint().unwrap_or(0), 4069), 8) - 8);

        while let Some((key, value)) = access.next_entry_seed(PhantomData, StrOrU32TupleVecSeed)? {
            match key {
                "w" => walls = Some(value),
                "r" => roads = Some(value),
                "pb" => power_or_power_bank = Some(value),
                "p" => portals = Some(value),
                "s" => sources = Some(value),
                "m" => minerals = Some(value),
                "c" => controllers = Some(value),
                "k" => keeper_lairs = Some(value),
                user_id => users_objects.push((user_id.to_owned(), value)),
            }
        }

        let finished = RoomMapViewUpdate {
            walls: walls.unwrap_or_default(),
            roads: roads.unwrap_or_default(),
            power_or_power_bank: power_or_power_bank.unwrap_or_default(),
            portals: portals.unwrap_or_default(),
            sources: sources.unwrap_or_default(),
            minerals: minerals.unwrap_or_default(),
            controllers: controllers.unwrap_or_default(),
            keeper_lairs: keeper_lairs.unwrap_or_default(),
            users_objects: users_objects,
            _phantom: PhantomData,
        };

        Ok(finished)
    }
}

impl<'de> Deserialize<'de> for RoomMapViewUpdate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(RoomMapViewUpdateVisitor::new())
    }
}

#[cfg(test)]
mod test {
    use super::RoomMapViewUpdate;
    use serde_json;

    // this is an edge case discovered on the official server.
    //
    // Sometimes it will give strings as coordinates rather than integers.
    #[test]
    fn string_coords_edge_case() {
        let _: RoomMapViewUpdate = serde_json::from_str(
            r#"{
            "w":[[31,18],[9,8],[13,8],[26,2],[27,2],[28,2],[31,2],[32,2],[35,2],[38,2],[39,2],[40,2],[42,2],[43,2],
            [44,2],[45,2],[47,8],[47,9],[47,14],[47,15],[47,17],[47,19],[47,20],[47,21],[47,26],[47,27],[47,28],
            [47,29],[47,30],[47,31],[47,32],[47,33],[47,37],[47,38],[47,39],[47,40],[47,43],[2,28],[2,29],[2,31],
            [2,32],[2,35],[2,36],[2,37],[2,38],[2,39],[2,40],[2,44],[2,45],[32,16],[2,41],[32,17],[31,15],[8,8],[11,8],
            [12,8],[15,4],[15,6],[25,2],[33,2],[34,2],[36,2],[37,2],[41,2],[47,10],[47,11],[47,16],[47,18],[47,24],
            [47,25],[47,34],[47,35],[47,36],[47,44],[47,45],[36,47],[37,47],[39,47],[41,47],[42,47],[45,47],[38,47],
            [40,47],[43,47],[44,47],[28,14],[17,47],[14,47],[13,47],[12,47],[11,47],[10,47],[9,47],[8,47],[7,47],
            [6,47],[5,47],[33,15],[33,16],[47,22],[47,23],[47,46],[33,1]],
            "r":[],"pb":[],"p":[],"s":[[26,38],[31,16]],"c":[[30,15]],"m":[[15,7]],"k":[],
            "54d8d6bf9facf3600349ba3d":[[28,26],[31,13],[29,28],[27,27],[30,21],[27,22],[33,22],[27,23],[24,23],
            [30,25],[31,26],[31,11],[31,27],[30,28],[28,27],[27,28],[25,24],[26,29],[26,26],[36,12],[34,11],[35,12],
            [35,10],[36,11],[36,9],[30,22],[29,22],[24,22],[26,24],[25,26],[24,27],[25,29],[24,28],[26,21],[32,23],
            [15,7],[30,13],[29,13],[37,10],[36,18],[34,10],[35,9],[37,11],[27,37],[30,11],[31,23],[33,21],[32,14],
            [37,17],[29,11],[28,24],[33,20],[23,20],[32,27],[33,29],[33,28],[32,28],[30,29],[31,29],[32,30],[31,30],
            [25,30],[23,28],[24,30],[31,20],[25,21],[29,23],[30,24],[33,30],[23,21],[26,25],[25,20],[23,30],[24,20],
            [23,22],[31,24],[26,28],[25,27],[30,26],[29,27],[31,21],[32,22],[32,20],[25,23],[26,22],[27,25],[29,25],
            [29,12],[28,11],[35,17],["23","29"]]
        }"#,
        ).expect("expected edge case parsing to succeed");
    }
}
