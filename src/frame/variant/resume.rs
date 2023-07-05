#[derive(Debug, recode::Recode)]
#[recode(
    error = "crate::Error",
    decoder(buffer_type = "super::Buffer"),
    encoder(buffer_type = "super::BufferMut")
)]
pub struct Resume {
    pub(crate) version: super::Version,
    pub(crate) resume_identification_token: super::ResumeToken,
    pub(crate) last_received_server_position: u64,
    pub(crate) first_available_client_position: u64,
}
