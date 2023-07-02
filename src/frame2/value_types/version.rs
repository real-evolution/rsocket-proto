#[derive(Debug, Copy, Clone, Eq, PartialEq, recode::Recode)]
pub struct Version {
    pub major: u16,
    pub minor: u16,
}
