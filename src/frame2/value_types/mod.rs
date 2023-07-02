mod metadata;
mod version;

pub use metadata::Metadata;
pub use version::Version;

use recode::codec;

pub type Data = codec::UnprefixedBuffer;
pub type MimeType = codec::Ascii<u8>;
pub type ResumeToken = codec::Buffer<u16>;
