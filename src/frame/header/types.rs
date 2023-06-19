const FRAME_TYPE_SETUP: u8 = 0x01;
const FRAME_TYPE_LEASE: u8 = 0x02;
const FRAME_TYPE_KEEPALIVE: u8 = 0x03;
const FRAME_TYPE_REQUEST_RESPONSE: u8 = 0x04;
const FRAME_TYPE_REQUEST_FNF: u8 = 0x05;
const FRAME_TYPE_REQUEST_STREAM: u8 = 0x06;
const FRAME_TYPE_REQUEST_CHANNEL: u8 = 0x07;
const FRAME_TYPE_REQUEST_N: u8 = 0x08;
const FRAME_TYPE_CANCEL: u8 = 0x09;
const FRAME_TYPE_PAYLOAD: u8 = 0x0A;
const FRAME_TYPE_ERROR: u8 = 0x0B;
const FRAME_TYPE_METADATA_PUSH: u8 = 0x0C;
const FRAME_TYPE_RESUME: u8 = 0x0D;
const FRAME_TYPE_RESUME_OK: u8 = 0x0E;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FrameType {
    Setup = FRAME_TYPE_SETUP,
    Lease = FRAME_TYPE_LEASE,
    Keepalive = FRAME_TYPE_KEEPALIVE,
    RequestResponse = FRAME_TYPE_REQUEST_RESPONSE,
    RequestFNF = FRAME_TYPE_REQUEST_FNF,
    RequestStream = FRAME_TYPE_REQUEST_STREAM,
    RequestChannel = FRAME_TYPE_REQUEST_CHANNEL,
    RequestN = FRAME_TYPE_REQUEST_N,
    Cancel = FRAME_TYPE_CANCEL,
    Payload = FRAME_TYPE_PAYLOAD,
    Error = FRAME_TYPE_ERROR,
    MetadataPush = FRAME_TYPE_METADATA_PUSH,
    Resume = FRAME_TYPE_RESUME,
    ResumeOk = FRAME_TYPE_RESUME_OK,
    Other(u8),
}

impl From<u8> for FrameType {
    #[inline(always)]
    fn from(value: u8) -> Self {
        match value {
            | FRAME_TYPE_SETUP => FrameType::Setup,
            | FRAME_TYPE_LEASE => FrameType::Lease,
            | FRAME_TYPE_KEEPALIVE => FrameType::Keepalive,
            | FRAME_TYPE_REQUEST_RESPONSE => FrameType::RequestResponse,
            | FRAME_TYPE_REQUEST_FNF => FrameType::RequestFNF,
            | FRAME_TYPE_REQUEST_STREAM => FrameType::RequestStream,
            | FRAME_TYPE_REQUEST_CHANNEL => FrameType::RequestChannel,
            | FRAME_TYPE_REQUEST_N => FrameType::RequestN,
            | FRAME_TYPE_CANCEL => FrameType::Cancel,
            | FRAME_TYPE_PAYLOAD => FrameType::Payload,
            | FRAME_TYPE_ERROR => FrameType::Error,
            | FRAME_TYPE_METADATA_PUSH => FrameType::MetadataPush,
            | FRAME_TYPE_RESUME => FrameType::Resume,
            | FRAME_TYPE_RESUME_OK => FrameType::ResumeOk,
            | v => FrameType::Other(v),
        }
    }
}

impl From<FrameType> for u8 {
    #[inline(always)]
    fn from(value: FrameType) -> Self {
        match value {
            | FrameType::Setup => FRAME_TYPE_SETUP,
            | FrameType::Lease => FRAME_TYPE_LEASE,
            | FrameType::Keepalive => FRAME_TYPE_KEEPALIVE,
            | FrameType::RequestResponse => FRAME_TYPE_REQUEST_RESPONSE,
            | FrameType::RequestFNF => FRAME_TYPE_REQUEST_FNF,
            | FrameType::RequestStream => FRAME_TYPE_REQUEST_STREAM,
            | FrameType::RequestChannel => FRAME_TYPE_REQUEST_CHANNEL,
            | FrameType::RequestN => FRAME_TYPE_REQUEST_N,
            | FrameType::Cancel => FRAME_TYPE_CANCEL,
            | FrameType::Payload => FRAME_TYPE_PAYLOAD,
            | FrameType::Error => FRAME_TYPE_ERROR,
            | FrameType::MetadataPush => FRAME_TYPE_METADATA_PUSH,
            | FrameType::Resume => FRAME_TYPE_RESUME,
            | FrameType::ResumeOk => FRAME_TYPE_RESUME_OK,
            | FrameType::Other(v) => v,
        }
    }
}
