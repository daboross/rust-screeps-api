//! Structures which are shared and exist as sub-field of multiple structures.

with_update_struct! {
    /// A struct describing the destination of various actions within action logs.
    #[derive(serde::Deserialize, Clone, Debug, PartialEq)]
    pub struct ActionLogTarget {
        /// The in-room x position of this target.
        pub x: u32,
        /// The in-room x position of this target.
        pub y: u32,
    }

    /// The update structure for an `ActionLogTarget`.
    #[derive(serde::Deserialize, Clone, Debug)]
    pub struct ActionLogTargetUpdate { ... }
}
