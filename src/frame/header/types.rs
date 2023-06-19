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
pub enum RSocketFrameType {
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

impl From<u8> for RSocketFrameType {
    #[inline(always)]
    fn from(value: u8) -> Self {
        match value {
            | FRAME_TYPE_SETUP => RSocketFrameType::Setup,
            | FRAME_TYPE_LEASE => RSocketFrameType::Lease,
            | FRAME_TYPE_KEEPALIVE => RSocketFrameType::Keepalive,
            | FRAME_TYPE_REQUEST_RESPONSE => RSocketFrameType::RequestResponse,
            | FRAME_TYPE_REQUEST_FNF => RSocketFrameType::RequestFNF,
            | FRAME_TYPE_REQUEST_STREAM => RSocketFrameType::RequestStream,
            | FRAME_TYPE_REQUEST_CHANNEL => RSocketFrameType::RequestChannel,
            | FRAME_TYPE_REQUEST_N => RSocketFrameType::RequestN,
            | FRAME_TYPE_CANCEL => RSocketFrameType::Cancel,
            | FRAME_TYPE_PAYLOAD => RSocketFrameType::Payload,
            | FRAME_TYPE_ERROR => RSocketFrameType::Error,
            | FRAME_TYPE_METADATA_PUSH => RSocketFrameType::MetadataPush,
            | FRAME_TYPE_RESUME => RSocketFrameType::Resume,
            | FRAME_TYPE_RESUME_OK => RSocketFrameType::ResumeOk,
            | v => RSocketFrameType::Other(v),
        }
    }
}

impl From<RSocketFrameType> for u8 {
    #[inline(always)]
    fn from(value: RSocketFrameType) -> Self {
        match value {
            | RSocketFrameType::Setup => FRAME_TYPE_SETUP,
            | RSocketFrameType::Lease => FRAME_TYPE_LEASE,
            | RSocketFrameType::Keepalive => FRAME_TYPE_KEEPALIVE,
            | RSocketFrameType::RequestResponse => FRAME_TYPE_REQUEST_RESPONSE,
            | RSocketFrameType::RequestFNF => FRAME_TYPE_REQUEST_FNF,
            | RSocketFrameType::RequestStream => FRAME_TYPE_REQUEST_STREAM,
            | RSocketFrameType::RequestChannel => FRAME_TYPE_REQUEST_CHANNEL,
            | RSocketFrameType::RequestN => FRAME_TYPE_REQUEST_N,
            | RSocketFrameType::Cancel => FRAME_TYPE_CANCEL,
            | RSocketFrameType::Payload => FRAME_TYPE_PAYLOAD,
            | RSocketFrameType::Error => FRAME_TYPE_ERROR,
            | RSocketFrameType::MetadataPush => FRAME_TYPE_METADATA_PUSH,
            | RSocketFrameType::Resume => FRAME_TYPE_RESUME,
            | RSocketFrameType::ResumeOk => FRAME_TYPE_RESUME_OK,
            | RSocketFrameType::Other(v) => v,
        }
    }
}
