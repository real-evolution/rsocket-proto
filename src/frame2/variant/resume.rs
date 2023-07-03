#[derive(Debug, recode::Recode)]
#[recode(
    error = "crate::Error",
    decoder(buffer_type = "super::Buffer"),
    encoder(buffer_type = "super::BufferMut")
)]
pub struct Resume {
    pub version: super::Version,
    pub resume_identification_token: super::ResumeToken,
    pub last_received_server_position: u64,
    pub first_available_client_position: u64,
}
