use std::marker::PhantomData;
use std::borrow::Cow;
use std::{cmp, fmt};

use serde::{Deserializer, Deserialize};
use serde::de::{MapAccess, Visitor};

/// "Map view" room status update. This contains all entities in a given room,
/// organized by what type of thing they are, or who owns them.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RoomMapViewUpdate<'a> {
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
    pub users_objects: Vec<(Cow<'a, str>, Vec<(u32, u32)>)>,
    /// Phantom data in order to allow adding any additional fields in the future.
    _phantom: PhantomData<()>,
}

impl<'a> RoomMapViewUpdate<'a> {
    pub fn to_owned(self) -> RoomMapViewUpdate<'static> {
        RoomMapViewUpdate {
            walls: self.walls,
            roads: self.roads,
            power_or_power_bank: self.power_or_power_bank,
            portals: self.portals,
            sources: self.sources,
            minerals: self.minerals,
            controllers: self.controllers,
            keeper_lairs: self.keeper_lairs,
            users_objects: self.users_objects.into_iter().map(|(k, v)| (k.into_owned().into(), v)).collect(),
            _phantom: PhantomData,
        }
    }
}

struct RoomMapViewUpdateVisitor<'a> {
    marker: PhantomData<RoomMapViewUpdate<'a>>,
}

impl<'a> RoomMapViewUpdateVisitor<'a> {
    pub fn new() -> Self {
        RoomMapViewUpdateVisitor { marker: PhantomData }
    }
}

impl<'de> Visitor<'de> for RoomMapViewUpdateVisitor<'de> {
    type Value = RoomMapViewUpdate<'static>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a map")
    }

    #[inline]
    fn visit_unit<E>(self) -> Result<Self::Value, E> {
        Ok(RoomMapViewUpdate::default())
    }

    #[inline]
    fn visit_map<T>(self, mut access: T) -> Result<Self::Value, T::Error>
        where T: MapAccess<'de>
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
        let mut users_objects = Vec::with_capacity(cmp::max(cmp::min(access.size_hint().unwrap_or(0), 4069), 8) - 8);

        while let Some((key, value)) = access.next_entry()? {
            match key {
                "w" => walls = Some(value),
                "r" => roads = Some(value),
                "pb" => power_or_power_bank = Some(value),
                "p" => portals = Some(value),
                "s" => sources = Some(value),
                "m" => minerals = Some(value),
                "c" => controllers = Some(value),
                "k" => keeper_lairs = Some(value),
                user_id => users_objects.push((user_id.to_owned().into(), value)),
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

impl<'de> Deserialize<'de> for RoomMapViewUpdate<'static> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        deserializer.deserialize_map(RoomMapViewUpdateVisitor::new())
    }
}
