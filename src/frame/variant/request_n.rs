use derive_getters::Getters;
use recode::Recode;

#[derive(Debug, Clone, Getters, Recode)]
#[recode(error = "crate::Error")]
pub struct RequestN {
    pub(crate) request_n: super::NonZero<u32>,
}
