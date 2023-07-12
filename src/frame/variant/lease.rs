use derive_getters::Getters;
use recode::Recode;

#[derive(Debug, Clone, Getters, Recode)]
#[recode(
    error = "crate::Error",
    decoder(buffer_type = "super::Buffer"),
    encoder(buffer_type = "super::BufferMut")
)]
pub struct Lease {
    pub(crate) ttl: super::NonZero<u32>,
    pub(crate) number_of_requests: super::NonZero<u32>,
    #[recode(with = "super::Metadata")]
    pub(crate) metadata: Option<super::Metadata>,
}
