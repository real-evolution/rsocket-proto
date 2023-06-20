use super::codec::EmptyBody;

#[derive(Debug, Clone)]
pub struct Cancel;

impl EmptyBody for Cancel {}
