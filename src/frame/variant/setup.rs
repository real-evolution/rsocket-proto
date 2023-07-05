#[derive(Debug, recode::Recode)]
#[recode(
    error = "crate::Error",
    decoder(buffer_type = "super::Buffer"),
    encoder(buffer_type = "super::BufferMut")
)]
pub struct Setup {
    pub(crate) version: super::Version,
    pub(crate) keepalive: super::NonZero<u32>,
    pub(crate) lifetime: super::NonZero<u32>,
    pub(crate) token: super::ResumeToken,
    pub(crate) mime_metadata: super::MimeType,
    pub(crate) mime_data: super::MimeType,
    #[recode(with = "super::Metadata")]
    pub(crate) metadata: Option<super::Metadata>,
    pub(crate) data: super::Data,
}
