mod body;
mod header;
mod primitives;

pub use body::*;
pub use header::*;
pub use primitives::*;

#[derive(Debug)]
pub struct Frame<'a> {
    pub header: FrameHeader,
    pub body: FrameBody<'a>,
}
