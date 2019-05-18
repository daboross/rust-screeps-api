//! Module containing macros which simplify making "updateable" structures.
use time::Timespec;

use crate::data::{Badge, RoomName};

/// Helper trait for the below macros, to help reduce boilerplate further.
///
/// This is implemented trivially for basic types, then specifically for
/// any 'sub-updates' we have, like a spawn's inner spawn, or a room sign.
pub(super) trait Updatable: Sized {
    type Update;

    /// Updates all fields of this struct with all present fields in the update.
    fn apply_update(&mut self, update: Self::Update);

    /// If all fields are present, creates this structure from the update. Otherwise,
    /// returns None.
    fn create_from_update(update: Self::Update) -> Option<Self>;
}

macro_rules! basic_updatable {
    ($($name:ty),*$(,)?) => (
        $(
            impl crate::websocket::types::room::room_object_macros::Updatable for $name {
                type Update = $name;

                fn apply_update(&mut self, update: Self::Update) {
                    *self = update;
                }

                fn create_from_update(update: Self::Update) -> Option<Self> {
                    Some(update)
                }
            }
        )*
    );
}

basic_updatable!(bool, u8, u16, u32, u64, i8, i16, i32, i64, f32, f64);
basic_updatable!(String, Timespec, RoomName, Badge, ());

pub(crate) mod vec_update {
    use std::marker::PhantomData;

    use std::{cmp, fmt};

    use serde::de::{Deserialize, Deserializer, Error, MapAccess, Unexpected, Visitor};

    use super::Updatable;

    /// Update structure for a Vec.
    #[derive(Debug, Clone)]
    pub(crate) enum VecUpdate<T> {
        Array(Vec<T>),
        PartialObj(VecPartialUpdate<T>),
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_VecUpdate: () = {
        extern crate serde as _serde;
        impl<'de, T> _serde::Deserialize<'de> for VecUpdate<T>
        where
            T: _serde::Deserialize<'de>,
        {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                let err1;
                let err2;
                let __content =
                    <_serde::private::de::Content as Deserialize>::deserialize(__deserializer)?;
                match Result::map(
                    Vec::<T>::deserialize(
                        _serde::private::de::ContentRefDeserializer::<__D::Error>::new(&__content),
                    ),
                    VecUpdate::Array,
                ) {
                    Ok(value) => return Ok(value),
                    Err(e) => err1 = e,
                }
                match Result::map(
                    VecPartialUpdate::<T>::deserialize(
                        _serde::private::de::ContentRefDeserializer::<__D::Error>::new(&__content),
                    ),
                    VecUpdate::PartialObj,
                ) {
                    Ok(value) => return Ok(value),
                    Err(e) => err2 = e,
                }
                _serde::export::Err(_serde::de::Error::custom(format!(
                    "data did not match any variant of \
                     untagged enum VecUpdate (error for \
                     Array: {}, error for PartialObj: {})",
                    err1, err2
                )))
            }
        }
    };

    #[derive(Debug, Clone)]
    pub struct VecPartialUpdate<T>(Vec<(u32, T)>);

    struct VecPartialUpdateVisitor<T> {
        marker: PhantomData<T>,
    }

    impl<T> VecPartialUpdateVisitor<T> {
        pub fn new() -> Self {
            VecPartialUpdateVisitor {
                marker: PhantomData,
            }
        }
    }

    impl<'de, T> Visitor<'de> for VecPartialUpdateVisitor<T>
    where
        T: Deserialize<'de>,
    {
        type Value = VecPartialUpdate<T>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map")
        }

        #[inline]
        fn visit_unit<E>(self) -> Result<Self::Value, E> {
            Ok(VecPartialUpdate(Vec::new()))
        }

        #[inline]
        fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
        {
            struct StringKeyAsInt(u32);

            impl<'de> Deserialize<'de> for StringKeyAsInt {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: Deserializer<'de>,
                {
                    struct StringKeyAsIntVisitor;

                    impl<'de> Visitor<'de> for StringKeyAsIntVisitor {
                        type Value = StringKeyAsInt;

                        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                            write!(formatter, "a string formatted number")
                        }

                        #[inline]
                        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                        where
                            E: Error,
                        {
                            Ok(StringKeyAsInt(value.parse().map_err(|_| {
                                E::invalid_value(Unexpected::Str(value), &self)
                            })?))
                        }
                    }
                    deserializer.deserialize_str(StringKeyAsIntVisitor)
                }
            }

            let mut values = Vec::with_capacity(cmp::min(access.size_hint().unwrap_or(0), 4069));

            while let Some((key, value)) = access.next_entry::<StringKeyAsInt, _>()? {
                values.push((key.0, value));
            }

            Ok(VecPartialUpdate(values))
        }
    }

    impl<'de, T> Deserialize<'de> for VecPartialUpdate<T>
    where
        T: Deserialize<'de>,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_map(VecPartialUpdateVisitor::new())
        }
    }

    impl<T> Updatable for Vec<T>
    where
        T: Updatable,
    {
        type Update = VecUpdate<<T as Updatable>::Update>;

        fn apply_update(&mut self, update: Self::Update) {
            // TODO: proper erroring out here.
            match update {
                VecUpdate::Array(vec) => {
                    if let Some(vec) = vec
                        .into_iter()
                        .map(T::create_from_update)
                        .collect::<Option<Vec<T>>>()
                    {
                        *self = vec;
                    }
                }
                VecUpdate::PartialObj(map) => {
                    for (index, value) in map.0 {
                        let index = index as usize;
                        if index > self.len() {
                            continue; // what to do here..?
                        } else if index == self.len() {
                            if let Some(value) = T::create_from_update(value) {
                                self.push(value);
                            }
                            continue;
                        }
                        self[index as usize].apply_update(value);
                    }
                }
            }
        }

        fn create_from_update(update: Self::Update) -> Option<Self> {
            match update {
                VecUpdate::Array(vec) => vec
                    .into_iter()
                    .map(T::create_from_update)
                    .collect::<Option<Self>>(),
                VecUpdate::PartialObj(_) => None,
            }
        }
    }
}

impl Updatable for serde_json::Value {
    type Update = serde_json::Value;

    fn apply_update(&mut self, update: Self::Update) {
        use serde_json::Value::*;
        match update {
            Object(map) => match *self {
                Object(ref mut here_map) => here_map.extend(map.into_iter()),
                _ => *self = Object(map),
            },
            other => *self = other,
        }
    }

    fn create_from_update(update: Self::Update) -> Option<Self> {
        Some(update)
    }
}

impl<T> Updatable for Option<T>
where
    T: Updatable,
{
    type Update = Option<T::Update>;

    fn apply_update(&mut self, update: Self::Update) {
        match update {
            Some(value_update) => match *self {
                Some(ref mut existing) => existing.apply_update(value_update),
                None => *self = T::create_from_update(value_update),
            },
            None => *self = None,
        }
    }

    fn create_from_update(update: Self::Update) -> Option<Self> {
        Some(update.and_then(T::create_from_update))
    }
}

/// Mostly an implementation detail of `with_update_struct`, but can be used independently to
/// implement Updatable on external structures.
///
/// Adds a few extra meta attributes for serde deserialization to make "null" correctly erase values in an update.
macro_rules! implement_update_for {
    (
        $name:ident;

        $( #[$struct_attr:meta] )*
        pub struct $update_name:ident {
            $(
                $( #[$field_attr:meta] )*
                $( ($field_extra:tt) )*
                priv $field:ident : $type:ty,
            )*
        }
    ) => (
        add_metadata!{
            $( #[$struct_attr] )*
            pub struct $update_name {
                <<>>
                $(
                    $( #[$field_attr] )*
                    $( ($field_extra) )*
                    priv $field: $type,
                )*
            }
        }

        impl crate::websocket::types::room::room_object_macros::Updatable for $name {
            type Update = $update_name;

            fn apply_update(&mut self, update: Self::Update) {
                $(
                    if let Some(value_update) = update.$field {
                        self.$field.apply_update(value_update);
                    }
                )*
            }

            fn create_from_update(update: Self::Update) -> Option<Self> {
                let finished = $name {
                    $(
                        $field: match update.$field.and_then(
                                crate::websocket::types::room::room_object_macros::Updatable::create_from_update) {
                            Some(v) => v,
                            None => return None
                        },
                    )*
                };

                Some(finished)
            }
        }
    );
    (
        $name:ident;

        $(
            #[$struct_attr:meta]
        )*
        (no_extra_meta)
        pub struct $update_name:ident {
            $(
                $(#[$field_attr:meta])*
                $( ($field_extra:tt) )*
                priv $field:ident : $type:ty,
            )*
        }
    ) => (
        implement_update_for! {
            $name;

            $( #[$struct_attr] )*
            pub struct $update_name {
                $(
                    $( #[$field_attr] )*
                    (no_extra_meta)
                    priv $field: $type,
                )*
            }
        }
    )
}

/// Any value that is present is considered Some value, including null.
///
/// Implementation detail of `implement_update_for!()`.
///
/// Thanks to @dtolnay, see <https://github.com/serde-rs/serde/issues/984>.
pub(crate) mod always_some {
    use serde::{Deserialize, Deserializer};

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
    where
        T: Deserialize<'de>,
        D: Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer).map(Some)
    }
}

/// Rule about adding metadata to a single field...
macro_rules! add_metadata {
    (
        $( #[$struct_attr:meta] )*
        pub struct $update_name:ident {
            <<$( $( #[$built_field_attr:meta] )* priv $built_field:ident: $built_type:ty, )*>>
            $(#[$field_attr:meta])*
            priv $field:ident : $type:ty,
            $($rest:tt)*
        }
    ) => (
        add_metadata!{
            $(#[$struct_attr])*
            pub struct $update_name {
                <<
                    $(
                        $( #[$built_field_attr] )*
                        priv $built_field: $built_type,
                    )*

                    #[serde(default, with = "crate::websocket::types::room::room_object_macros::always_some")]
                    $( #[$field_attr] )*
                    priv $field: $type,
                >>
                $($rest)*
            }
        }
    );
    (
        $( #[$struct_attr:meta] )*
        pub struct $update_name:ident {
            <<$( $( #[$built_field_attr:meta] )* priv $built_field:ident: $built_type:ty, )*>>
            $(#[$field_attr:meta])*
            (no_extra_meta)
            priv $field:ident : $type:ty,
            $($rest:tt)*
        }
    ) => (
        add_metadata!{
            $(#[$struct_attr])*
            pub struct $update_name {
                <<
                    $(
                        $( #[$built_field_attr] )*
                        priv $built_field: $built_type,
                    )*

                    $( #[$field_attr] )*
                    priv $field: $type,
                >>
                $($rest)*
            }
        }
    );
    (
        $( #[$struct_attr:meta] )*
        pub struct $update_name:ident {
            <<$( $( #[$built_field_attr:meta] )* priv $built_field:ident: $built_type:ty, )*>>
        }
    ) => (
        $(#[$struct_attr])*
        pub struct $update_name {
            $(
                $( #[$built_field_attr] )*
                $built_field: $built_type,
            )*
        }
    );
}

/// This creates the structure described within the macro invocation, and then creates another "update"
/// structure with the same fields, but with all fields as Options.
///
/// A method `update` is implemented on the base structure which will take an instance of the update
/// structure and apply all changes to the base structure's fields.
macro_rules! with_update_struct {
    (
        $( #[$struct_attr:meta] )*
        pub struct $name:ident {
            $(
                $( #[$field_attr:meta] )*
                $( ($field_extra:tt) )*
                $field_vis:vis $field:ident : $type:ty,
            )*
        }

        $( #[$update_struct_attr:meta] )*
        $( ($update_extra:tt) )*
        pub struct $update_name:ident { ... }
    ) => (
        with_update_struct! {
            $( #[$struct_attr] )*
            pub struct $name {
                $(
                    $( #[$field_attr] )*
                    $field_vis $field : $type,
                )*
            }

            $( #[$update_struct_attr] )*
            $( ($update_extra) )*
            pub struct $update_name {
                $(
                    $( #[$field_attr] )*
                    $( ($field_extra) )*
                    - $field : $type,
                )*
            }
        }
    );
    (
        $( #[$struct_attr:meta] )*
        pub struct $name:ident {
            $(
                $( #[$field_attr:meta] )*
                $field_vis:vis $field:ident : $type:ty,
            )*
        }

        $( #[$update_struct_attr:meta] )*
        $( ($update_extra:tt) )*
        pub struct $update_name:ident {
            $(
                $( #[$update_field_attr:meta] )*
                $( ($update_field_extra:tt) )*
                - $update_field:ident : $update_type:ty,
            )*
        }
    ) => (
        $( #[$struct_attr] )*
        pub struct $name {
            $(
                $( #[$field_attr] )*
                $field_vis $field: $type,
            )*
        }

        implement_update_for! {
            $name;

            $( #[$update_struct_attr] )*
            $( ($update_extra) )*
            pub struct $update_name {
                $(
                    $( #[$update_field_attr] )*
                    $( ($update_field_extra) )*
                    priv $update_field:
                        Option<<$update_type as crate::websocket::types::room::room_object_macros::Updatable>::Update>,
                )*
            }
        }

        impl $name {
            /// Updates this structure with all values present in the given update.
            pub fn update(&mut self, update: $update_name) {
                <Self as crate::websocket::types::room::room_object_macros::Updatable>::apply_update(self, update);
            }
        }
    )
}

/// This macro creates the struct described within the invocation, but with an additional 4 fields common to all
/// room objects, and with `#[derive(serde_derive::Deserialize)]`. The structure definition is then passed on to `with_update_struct`.
macro_rules! with_base_fields_and_update_struct {
    (
        $( #[$struct_attr:meta] )*
        pub struct $name:ident {
            $(
                $(#[$field_attr:meta])*
                $( ($field_extra:tt) )*
                pub $field:ident : $type:ty,
            )*
        }

        $( #[$update_struct_attr:meta] )*
        $( ($update_extra:tt) )*
        pub struct $update_name:ident { ... }
    ) => (
        with_base_fields_and_update_struct! {
            $( #[$struct_attr] )*
            pub struct $name {
                $(
                    $( #[$field_attr] )*
                    $( ($field_extra:tt) )*
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
        $( #[$struct_attr:meta] )*
        pub struct $name:ident {
            $(
                $(#[$field_attr:meta])*
                $( ($field_extra:tt) )*
                pub $field:ident : $type:ty,
            )*
        }

        $( #[$update_struct_attr:meta] )*
        $( ($update_extra:tt) )*
        pub struct $update_name:ident {
            $( $update_field:tt )*
        }
    ) => (
        with_update_struct! {
            $( #[$struct_attr] )*
            #[derive(serde_derive::Deserialize)]
            pub struct $name {
                /// Unique 'id' identifier for all game objects on a server.
                #[serde(rename = "_id")]
                pub id: String,
                /// Room object is in.
                pub room: RoomName,
                /// X position within the room (0-50).
                #[serde(with = "crate::decoders::u32_or_str_containing")]
                pub x: u32,
                /// Y position within the room (0-50).
                #[serde(with = "crate::decoders::u32_or_str_containing")]
                pub y: u32,
                $(
                    $( #[$field_attr] )*
                    $( ($field_extra:tt) )*
                    pub $field : $type,
                )*
            }

            $( #[$update_struct_attr] )*
            #[derive(serde_derive::Deserialize)]
            $( ($update_extra) )*
            pub struct $update_name {
                #[serde(rename = "_id")]
                - id: String,
                - room: RoomName,
                #[serde(default, with = "crate::decoders::optional_u32_or_str_containing")]
                (no_extra_meta)
                - x: u32,
                #[serde(default, with = "crate::decoders::optional_u32_or_str_containing")]
                (no_extra_meta)
                - y: u32,
                $( $update_field )*
            }
        }
    )
}

/// This macro creates the struct described within the invocation, but with an additional 2 fields common to all
/// Structures, and with everything provided by `with_base_fields_and_update_struct!`.
macro_rules! with_structure_fields_and_update_struct {
    (
        $( #[$struct_attr:meta] )*
        pub struct $name:ident {
            $(
                $(#[$field_attr:meta])*
                $( ($field_extra:tt) )*
                pub $field:ident : $type:ty,
            )*
        }

        $( #[$update_struct_attr:meta] )*
        $( ($update_extra:tt) )*
        pub struct $update_name:ident { ... }
    ) => (
        with_structure_fields_and_update_struct! {
            $( #[$struct_attr] )*
            pub struct $name {
                $(
                    $( #[$field_attr] )*
                    $( ($field_extra:tt) )*
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
        $( #[$struct_attr:meta] )*
        pub struct $name:ident {
            $(
                $(#[$field_attr:meta])*
                $( ($field_extra:tt) )*
                pub $field:ident : $type:ty,
            )*
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
                #[serde(default)]
                pub hits: i32,
                /// The maximum number of hit-points this structure has.
                #[serde(default, rename = "hitsMax")]
                pub hits_max: i32,
                $(
                    $( #[$field_attr] )*
                    $( ($field_extra:tt) )*
                    pub $field : $type,
                )*
            }

            $( #[$update_struct_attr] )*
            $( ($update_extra) )*
            pub struct $update_name {
                - hits: i32,
                #[serde(rename = "hitsMax")]
                - hits_max: i32,
                $( $update_field )*
            }
        }
    )
}
