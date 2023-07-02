use from_to_repr::from_to_other;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[from_to_other(base_type = u8)]
#[repr(u8)]
pub enum FrameType {
    Setup = 0x01,
    Lease = 0x02,
    Keepalive = 0x03,
    RequestResponse = 0x04,
    RequestFNF = 0x05,
    RequestStream = 0x06,
    RequestChannel = 0x07,
    RequestN = 0x08,
    Cancel = 0x09,
    Payload = 0x0A,
    Error = 0x0B,
    MetadataPush = 0x0C,
    Resume = 0x0D,
    ResumeOk = 0x0E,
    Ext = 0x3F,
    Other(u8),
}

impl FrameType {
    pub const fn flags_mask(&self) -> super::Flags {
        use crate::const_flags as f;

        match self {
            | FrameType::Setup => f![METADATA | RESUME | LEASE],
            | FrameType::Lease => f![METADATA],
            | FrameType::Keepalive => f![RESPOND],
            | FrameType::RequestResponse => f![METADATA | FOLLOW],
            | FrameType::RequestFNF => f![METADATA | FOLLOW],
            | FrameType::RequestStream => f![METADATA | FOLLOW],
            | FrameType::RequestChannel => f![METADATA | FOLLOW | COMPLETE],
            | FrameType::RequestN => f![],
            | FrameType::Cancel => f![],
            | FrameType::Payload => f![METADATA | FOLLOW | COMPLETE | NEXT],
            | FrameType::Error => f![],
            | FrameType::MetadataPush => f![METADATA],
            | FrameType::Resume => f![],
            | FrameType::ResumeOk => f![],
            | FrameType::Ext => f![IGNORE | METADATA],
            | FrameType::Other(_) => f![],
        }
    }
}
