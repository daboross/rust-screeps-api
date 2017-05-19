use std::marker::PhantomData;

/// Notification for Update for a user's last tick CPU usage and total memory usage.
#[derive(Deserialize, Clone, Hash, Debug)]
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
    /// Phantom data in order to allow adding any additional fields in the future.
    #[serde(skip_deserializing)]
    _phantom: PhantomData<()>,
}
