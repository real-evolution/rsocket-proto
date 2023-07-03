#[derive(Debug, recode::Recode)]
#[recode(
    error = "crate::Error",
    decoder(buffer_type = "super::Buffer"),
    encoder(buffer_type = "super::BufferMut")
)]
pub struct Setup {
    version: super::Version,
    keepalive: super::NonZero<u32>,
    lifetime: super::NonZero<u32>,
    token: super::ResumeToken,
    mime_metadata: super::MimeType,
    mime_data: super::MimeType,
    metadata: Option<super::Metadata>,
    data: super::Data,
}
