use derive_getters::Getters;
use recode::Recode;

#[derive(Debug, Clone, Getters, Recode)]
#[recode(error = "crate::Error")]
pub struct Keepalive {
    pub(crate) last_received_position: u64,
    pub(crate) data: super::Data,
}
