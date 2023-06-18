use thiserror::Error;

/// A type to represent all possible errors that can occur when using this
/// library.
#[derive(Debug, Error)]
pub enum RSocketError {}
