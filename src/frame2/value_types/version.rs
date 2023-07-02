#[derive(Debug, Copy, Clone, Eq, PartialEq, recode::Recode)]
#[recode(decoder(error = "crate::Error"), encoder(error = "crate::Error"))]
pub struct Version {
    pub major: u16,
    pub minor: u16,
}
