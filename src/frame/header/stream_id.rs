/// A type to represent a strongly-typed rsocket stream identifier.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, recode::Recode,
)]
#[recode(error = "crate::Error")]
pub struct StreamId {
    inner: u32,
}

impl StreamId {
    /// Gets whether the stream identifier is connection stream identifier.
    pub const fn is_cnnection(&self) -> bool {
        self.inner == 0
    }
}
