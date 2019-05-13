//! Structures which are shared and exist as sub-field of multiple structures.

with_update_struct! {
    /// A struct describing the destination of various actions within action logs.
    #[derive(serde_derive::Deserialize, Clone, Debug, PartialEq)]
    pub struct ActionLogTarget {
        /// The in-room x position of this target.
        pub x: u16,
        /// The in-room x position of this target.
        pub y: u16,
    }

    /// The update structure for an `ActionLogTarget`.
    #[derive(serde_derive::Deserialize, Clone, Debug)]
    pub struct ActionLogTargetUpdate { ... }
}
