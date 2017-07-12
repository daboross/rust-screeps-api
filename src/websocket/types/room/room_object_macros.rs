//! Module containing macros which simplify making "updateable" structures.

use data::RoomName;

use time::Timespec;

use serde_json;

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
    ($name: ident) => (
        impl Updatable for $name {
            type Update = $name;

            fn apply_update(&mut self, update: Self::Update) {
                *self = update;
            }

            fn create_from_update(update: Self::Update) -> Option<Self> {
                Some(update)
            }
        }
    );
    ($name: ident, $($extra_name:ident),*) => (
        // nice recursive syntax.
        basic_updatable!($name);
        basic_updatable!($($extra_name),*);
    )
}

basic_updatable!(bool, u8, u16, u32, u64, i8, i16, i32, i64, String, Timespec);
basic_updatable!(RoomName);

impl Updatable for serde_json::Value {
    type Update = serde_json::Value;

    fn apply_update(&mut self, update: Self::Update) {
        use serde_json::Value::*;
        match update {
            Object(map) => {
                match *self {
                    Object(ref mut here_map) => here_map.extend(map.into_iter()),
                    _ => *self = Object(map),
                }
            }
            other => *self = other,
        }
    }

    fn create_from_update(update: Self::Update) -> Option<Self> {
        Some(update)
    }
}

impl<T> Updatable for Option<T>
    where T: Updatable
{
    type Update = Option<T::Update>;

    fn apply_update(&mut self, update: Self::Update) {
        match update {
            Some(value_update) => {
                match *self {
                    Some(ref mut existing) => existing.apply_update(value_update),
                    None => *self = T::create_from_update(value_update),
                }
            }
            None => *self = None,
        }
    }

    fn create_from_update(update: Self::Update) -> Option<Self> {
        Some(update.and_then(T::create_from_update))
    }
}

/// Mostly an implementation detail of `with_update_struct`, but can be used independently to
/// implement Updatable on external structures.
macro_rules! implement_update_for_no_extra_meta {
    (
        $name:ident;

        $( #[$struct_attr:meta] )*
        pub struct $update_name:ident {
            $(
                $( #[$field_attr:meta] )*
                priv $field:ident : $type:ty,
            )*
        }
    ) => (
        $( #[$struct_attr] )*
        pub struct $update_name {
            $(
                $( #[$field_attr] )*
                $field: $type,
            )*
        }

        impl ::websocket::types::room::room_object_macros::Updatable for $name {
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
                                ::websocket::types::room::room_object_macros::Updatable::create_from_update) {
                            Some(v) => v,
                            None => return None
                        },
                    )*
                };

                Some(finished)
            }
        }
    )
}

/// Any value that is present is considered Some value, including null.
///
/// Implementation detail of `implement_update_for!()`.
///
/// Thanks to @dtolnay, see https://github.com/serde-rs/serde/issues/984.
pub(crate) mod always_some {
    use serde::{Deserialize, Deserializer};

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
        where T: Deserialize<'de>,
              D: Deserializer<'de>
    {
        Deserialize::deserialize(deserializer).map(Some)
    }
}

/// Mostly an implementation detail of `with_update_struct`, but can be used independently to
/// implement Updatable on external structures.
///
/// Adds a few extra meta attributes for serde deserialization to make "null" correctly erase values in an update.
macro_rules! implement_update_for {
    (
        $name:ident;

        $(
            #[$struct_attr:meta]
        )*
        pub struct $update_name:ident {
            $(
                $(#[$field_attr:meta])*
                priv $field:ident : $type:ty,
            )*
        }
    ) => (
        implement_update_for_no_extra_meta! {
            $name;

            $( #[$struct_attr] )*
            pub struct $update_name {
                $(
                    #[serde(default, with = "::websocket::types::room::room_object_macros::always_some")]
                    $( #[$field_attr] )*
                    priv $field: $type,
                )*
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
                priv $field:ident : $type:ty,
            )*
        }
    ) => (
        implement_update_for_no_extra_meta! {
            $name;

            $( #[$struct_attr] )*
            pub struct $update_name {
                $(
                    $( #[$field_attr] )*
                    priv $field: $type,
                )*
            }
        }
    )
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
                pub $field:ident : $type:ty,
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
                $( #[$field_attr:meta] )*
                pub $field:ident : $type:ty,
            )*
        }

        $( #[$update_struct_attr:meta] )*
        $( ($update_extra:tt) )*
        pub struct $update_name:ident {
            $(
                $( #[$update_field_attr:meta] )*
                - $update_field:ident : $update_type:ty,
            )*
        }
    ) => (
        $( #[$struct_attr] )*
        pub struct $name {
            $(
                $( #[$field_attr] )*
                pub $field: $type,
            )*
        }

        implement_update_for! {
            $name;

            $( #[$update_struct_attr] )*
            $( ($update_extra) )*
            pub struct $update_name {
                $(
                    $( #[$update_field_attr] )*
                    priv $update_field:
                        Option<<$update_type as ::websocket::types::room::room_object_macros::Updatable>::Update>,
                )*
            }
        }

        impl $name {
            /// Updates this structure with all values present in the given update.
            pub fn update(&mut self, update: $update_name) {
                <Self as ::websocket::types::room::room_object_macros::Updatable>::apply_update(self, update);
            }
        }
    )
}

/// This macro creates the struct described within the invocation, but with an additional 4 fields common to all
/// RoomObjects, and with `#[derive(Deserialize)]`. The structure definition is then passed on to `with_update_struct`.
macro_rules! with_base_fields_and_update_struct {
    (
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
        with_base_fields_and_update_struct! {
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
        with_update_struct! {
            $( #[$struct_attr] )*
            #[derive(Deserialize)]
            pub struct $name {
                /// Unique 'id' identifier for all game objects on a server.
                #[serde(rename = "_id")]
                pub id: String,
                /// Room object is in.
                pub room: RoomName,
                /// X position within the room (0-50).
                pub x: u16,
                /// Y position within the room (0-50).
                pub y: u16,
                $( $struct_field )*
            }

            $( #[$update_struct_attr] )*
            #[derive(Deserialize)]
            $( ($update_extra) )*
            pub struct $update_name {
                #[serde(rename = "_id")]
                - id: String,
                - room: RoomName,
                - x: u16,
                - y: u16,
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
            $( $struct_field:tt )*
        }

        $( #[$update_struct_attr:meta] )*
        $( ($update_extra:tt) )*
        pub struct $update_name:ident { ... }
    ) => (
        with_base_fields_and_update_struct! {
            $( #[$struct_attr] )*
            pub struct $name {
                /// The current number of hit-points this structure has.
                pub hits: i32,
                /// The maximum number of hit-points this structure has.
                #[serde(rename = "hitsMax")]
                pub hits_max: i32,
                $( $struct_field )*
            }

            $( #[$update_struct_attr] )*
            $( ($update_extra) )*
            pub struct $update_name { ... }
        }
    );
    (
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
                $( $struct_field )*
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
