#[derive(Debug, Clone, recode::Recode)]
#[recode(error = "crate::Error")]
pub struct RequestN {
    pub(crate) request_n: super::NonZero<u32>,
}
