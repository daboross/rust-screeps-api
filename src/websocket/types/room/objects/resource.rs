//! `Resource` data description.
use std::fmt;

use serde::de::{
    value::Error as ValueError, Deserialize, Deserializer, Error, IgnoredAny, IntoDeserializer,
    MapAccess, Visitor,
};

use super::super::resources::ResourceType;
use crate::data::RoomName;

with_update_struct! {
    /// A resource, a bit of some resource which has dropped on the ground, and is decaying each tick.
    #[derive(Clone, Debug, PartialEq)]
    pub struct Resource {
        /// Unique 'id' identifier for all game objects on a server.
        pub id: String,
        /// Room object is in.
        pub room: RoomName,
        /// X position within the room (0-50).
        pub x: u32,
        /// Y position within the room (0-50).
        pub y: u32,
        /// Resource type that this resource is.
        pub resource_type: ResourceType,
        /// Resource amount that this resource is.
        pub amount: i32,
    }

    /// Update structure for a `Resource`.
    #[derive(Clone, Debug)]
    (no_extra_meta)
    pub struct ResourceUpdate {
        - id: String,
        - room: RoomName,
        - x: u32,
        - y: u32,
        - resource_type: ResourceType,
        - amount: i32,
    }
}

/// deserialize helper, shared between `Resource` and `ResourceUpdate`.
enum FieldName {
    Id,
    Room,
    X,
    Y,
    ResourceType,
    Other(ResourceType),
    Ignored,
}

impl<'de> Deserialize<'de> for FieldName {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FieldVisitor;
        impl<'de> Visitor<'de> for FieldVisitor {
            type Value = FieldName;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "field identifier")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                match value {
                    "_id" => Ok(FieldName::Id),
                    "room" => Ok(FieldName::Room),
                    "x" => Ok(FieldName::X),
                    "y" => Ok(FieldName::Y),
                    "resourceType" => Ok(FieldName::ResourceType),
                    other => {
                        match ResourceType::deserialize(
                            IntoDeserializer::<ValueError>::into_deserializer(other),
                        ) {
                            Ok(resource_type) => Ok(FieldName::Other(resource_type)),
                            Err(_) => Ok(FieldName::Ignored),
                        }
                    }
                }
            }

            fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
            where
                E: Error,
            {
                match value {
                    b"_id" => Ok(FieldName::Id),
                    b"room" => Ok(FieldName::Room),
                    b"x" => Ok(FieldName::X),
                    b"y" => Ok(FieldName::Y),
                    b"resourceType" => Ok(FieldName::ResourceType),
                    other => match ::std::str::from_utf8(other) {
                        Ok(other_str) => {
                            match ResourceType::deserialize(
                                IntoDeserializer::<ValueError>::into_deserializer(other_str),
                            ) {
                                Ok(resource_type) => Ok(FieldName::Other(resource_type)),
                                Err(_) => Ok(FieldName::Ignored),
                            }
                        }
                        Err(_) => Ok(FieldName::Ignored),
                    },
                }
            }
        }
        deserializer.deserialize_identifier(FieldVisitor)
    }
}

impl<'de> Deserialize<'de> for Resource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ResourceVisitor;
        impl<'de> Visitor<'de> for ResourceVisitor {
            type Value = Resource;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                fmt::Formatter::write_str(formatter, "struct Resource")
            }

            #[inline]
            fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut id: Option<String> = None;
                let mut room: Option<RoomName> = None;
                let mut x: Option<u32> = None;
                let mut y: Option<u32> = None;
                let mut resource_type: Option<ResourceType> = None;
                let mut resource_amount: Option<(ResourceType, i32)> = None;
                while let Some(key) = access.next_key::<FieldName>()? {
                    match key {
                        FieldName::Id => {
                            if Option::is_some(&id) {
                                return Err(A::Error::duplicate_field("_id"));
                            }
                            id = Some(access.next_value::<String>()?);
                        }
                        FieldName::Room => {
                            if Option::is_some(&room) {
                                return Err(A::Error::duplicate_field("room"));
                            }
                            room = Some(access.next_value::<RoomName>()?);
                        }
                        FieldName::X => {
                            if Option::is_some(&x) {
                                return Err(A::Error::duplicate_field("x"));
                            }
                            x = Some(access.next_value::<u32>()?);
                        }
                        FieldName::Y => {
                            if Option::is_some(&y) {
                                return Err(A::Error::duplicate_field("y"));
                            }
                            y = Some(access.next_value::<u32>()?);
                        }
                        FieldName::ResourceType => {
                            if Option::is_some(&resource_type) {
                                return Err(A::Error::duplicate_field("resourceType"));
                            }
                            resource_type = Some(access.next_value::<ResourceType>()?);
                        }
                        FieldName::Other(resource) => {
                            if Option::is_some(&resource_amount) {
                                return Err(A::Error::duplicate_field(
                                    "<dynamic ResourceType-named amount field>",
                                ));
                            }
                            resource_amount = Some((resource, access.next_value::<i32>()?));
                        }
                        FieldName::Ignored => {
                            let _ = access.next_value::<IgnoredAny>()?;
                        }
                    }
                }
                let id = id.ok_or_else(|| A::Error::missing_field("_id"))?;
                let room = room.ok_or_else(|| A::Error::missing_field("room"))?;
                let x = x.ok_or_else(|| A::Error::missing_field("x"))?;
                let y = y.ok_or_else(|| A::Error::missing_field("y"))?;
                let resource_type =
                    resource_type.ok_or_else(|| A::Error::missing_field("resourceType"))?;
                let (found_resource_type, amount) = resource_amount
                    .ok_or_else(|| A::Error::missing_field("<dynamic ResouceType-named field>"))?;

                if resource_type != found_resource_type {
                    struct ResourceTypeMismatchError(ResourceType, ResourceType);

                    impl fmt::Display for ResourceTypeMismatchError {
                        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                            write!(
                                f,
                                "this structure expects both a resourceType field declaring a name, and a field \
                                 named by that name; found that resourceType field's value ({:?}) and name of \
                                 amount field ({:?}) did not match.",
                                self.0, self.1
                            )
                        }
                    }

                    return Err(A::Error::custom(ResourceTypeMismatchError(
                        resource_type,
                        found_resource_type,
                    )));
                }
                Ok(Resource {
                    id: id,
                    room: room,
                    x: x,
                    y: y,
                    resource_type: resource_type,
                    amount: amount,
                })
            }
        }
        const FIELDS: &[&str] = &["id", "room", "x", "y", "resourceType"];
        Deserializer::deserialize_struct(deserializer, "Resource", FIELDS, ResourceVisitor)
    }
}

impl<'de> Deserialize<'de> for ResourceUpdate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ResourceUpdateVisitor;
        impl<'de> Visitor<'de> for ResourceUpdateVisitor {
            type Value = ResourceUpdate;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                fmt::Formatter::write_str(formatter, "struct ResourceUpdate")
            }

            #[inline]
            fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut id: Option<String> = None;
                let mut room: Option<RoomName> = None;
                let mut x: Option<u32> = None;
                let mut y: Option<u32> = None;
                let mut resource_type: Option<ResourceType> = None;
                let mut amount: Option<i32> = None;
                while let Some(key) = access.next_key::<FieldName>()? {
                    match key {
                        FieldName::Id => {
                            if Option::is_some(&id) {
                                return Err(A::Error::duplicate_field("_id"));
                            }
                            id = Some(access.next_value::<String>()?);
                        }
                        FieldName::Room => {
                            if Option::is_some(&room) {
                                return Err(A::Error::duplicate_field("room"));
                            }
                            room = Some(access.next_value::<RoomName>()?);
                        }
                        FieldName::X => {
                            if Option::is_some(&x) {
                                return Err(A::Error::duplicate_field("x"));
                            }
                            x = Some(access.next_value::<u32>()?);
                        }
                        FieldName::Y => {
                            if Option::is_some(&y) {
                                return Err(A::Error::duplicate_field("y"));
                            }
                            y = Some(access.next_value::<u32>()?);
                        }
                        FieldName::ResourceType => {
                            if Option::is_some(&resource_type) {
                                return Err(A::Error::duplicate_field("resourceType"));
                            }
                            resource_type = Some(access.next_value::<ResourceType>()?);
                        }
                        FieldName::Other(_) => {
                            struct CanBeNullI32(Option<i32>);
                            impl<'de> Deserialize<'de> for CanBeNullI32 {
                                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                                where
                                    D: Deserializer<'de>,
                                {
                                    Ok(CanBeNullI32(Option::deserialize(deserializer)?))
                                }
                            }
                            if let CanBeNullI32(Some(value)) =
                                access.next_value::<CanBeNullI32>()?
                            {
                                if amount.is_some() {
                                    return Err(A::Error::duplicate_field(
                                        "<dynamic ResourceType-named amount field>",
                                    ));
                                }
                                amount = Some(value);
                            }
                        }
                        FieldName::Ignored => {
                            let _ = access.next_value::<IgnoredAny>()?;
                        }
                    }
                }

                Ok(ResourceUpdate {
                    id: id,
                    room: room,
                    x: x,
                    y: y,
                    resource_type: resource_type,
                    amount: amount,
                })
            }
        }
        const FIELDS: &'static [&'static str] = &["id", "room", "x", "y", "resourceType"];
        Deserializer::deserialize_struct(
            deserializer,
            "ResourceUpdate",
            FIELDS,
            ResourceUpdateVisitor,
        )
    }
}

#[cfg(test)]
mod test {
    use serde::Deserialize;

    use crate::data::RoomName;

    use super::{Resource, ResourceType};

    #[test]
    fn parse_resource() {
        let json = json!({
            "_id": "596990a3165c8c77de71ecf1",
            "energy": 7,
            "resourceType": "energy",
            "room": "W65N19",
            "type": "energy",
            "x": 8,
            "y": 34
        });

        let obj = Resource::deserialize(json).unwrap();

        assert_eq!(
            obj,
            Resource {
                id: "596990a3165c8c77de71ecf1".to_owned(),
                room: RoomName::new("W65N19").unwrap(),
                x: 8,
                y: 34,
                resource_type: ResourceType::Energy,
                amount: 7,
            }
        );
    }
}
