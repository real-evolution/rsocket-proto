#[derive(Debug, Clone, recode::Recode)]
#[recode(error = "crate::Error")]
pub struct Keepalive {
    pub(crate) last_received_position: u64,
    pub(crate) data: super::Data,
}
