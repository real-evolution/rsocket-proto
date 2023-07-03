#[derive(Debug, Clone, recode::Recode)]
#[recode(
    error = "crate::Error",
    decoder(buffer_type = "super::Buffer"),
    encoder(buffer_type = "super::BufferMut")
)]
pub struct RequestChannel {
    pub initial_request_n: super::NonZero<u32>,
    #[recode(with = "super::Metadata")]
    pub metadata: Option<super::Metadata>,
    pub data: super::Data,
}
