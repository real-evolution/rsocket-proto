mod body;
mod header;

pub use body::*;
pub use header::*;

#[derive(Debug)]
pub struct Frame {
    header: FrameHeader,
    body: FrameBody,
}
