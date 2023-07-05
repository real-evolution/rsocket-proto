#[derive(Debug, recode::Recode)]
#[recode(
    error = "crate::Error",
    decoder(buffer_type = "super::Buffer"),
    encoder(buffer_type = "super::BufferMut")
)]
pub struct ResumeOk {
    pub(crate) last_received_client_position: u64,
}
