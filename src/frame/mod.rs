mod body;
mod header;

pub use body::*;
pub use header::*;

#[derive(Debug)]
pub struct RSocketFrame {
    header: RSocketFrameHeader,
    body: RSocketFrameBody,
}
