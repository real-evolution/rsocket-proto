#[derive(Debug, Copy, Clone, Eq, PartialEq, Default, recode::Recode)]
#[recode(error = "crate::Error")]
pub struct Version {
    pub major: u16,
    pub minor: u16,
}
