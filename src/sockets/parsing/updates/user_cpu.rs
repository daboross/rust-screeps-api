use std::marker::PhantomData;

/// Update for a user's last tick CPU usage and total memory usage.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct UserCpuUpdate {
    /// The CPU usage last tick.
    #[serde(rename = "cpu")]
    pub last_tick_cpu: u32,
    /// The total memory usage in bytes, as of last tick.
    ///
    /// To clarify: this memory is the in-game persistent memory, not RAM.
    /// This is number of bytes the stringified memory takes.
    #[serde(rename = "memory")]
    pub memory_usage_bytes: u32,
    #[serde(skip_deserializing)]
    /// Phantom data in order to allow adding any additional fields in the future.
    _phantom: PhantomData<()>,
}
