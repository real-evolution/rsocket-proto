#[derive(Debug, Clone, recode::Recode)]
#[recode(error = "crate::Error")]
pub struct RequestN {
    pub request_n: super::NonZero<u32>,
}
