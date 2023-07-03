#[derive(Debug, Clone, recode::Recode)]
#[recode(error = "crate::Error")]
pub struct Keepalive {
    pub last_received_position: super::NonZero<u64>,
    pub data: super::Data,
}
