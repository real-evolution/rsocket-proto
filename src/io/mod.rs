mod fragmentation;

pub use fragmentation::{Defragmenter, Fragmenter};

#[cfg(feature = "codec")]
pub mod codec;
