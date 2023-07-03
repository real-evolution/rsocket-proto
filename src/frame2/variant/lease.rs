#[derive(Debug, Clone, recode::Recode)]
#[recode(
    error = "crate::Error",
    decoder(buffer_type = "super::Buffer"),
    encoder(buffer_type = "super::BufferMut")
)]
pub struct Lease {
    pub ttl: super::NonZero<u32>,
    pub number_of_requests: super::NonZero<u32>,
    pub metadata: Option<super::Metadata>,
}
