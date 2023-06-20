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
    Other(u8),
}
