#[derive(Debug, Clone, recode::Recode)]
#[recode(error = "crate::Error")]
pub struct Cancel;
