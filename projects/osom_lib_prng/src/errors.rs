//! Holds definitions of various PRNG errors.

use osom_lib_reprc::macros::reprc;

/// Represents serialization errors for PRNGs.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[reprc]
#[repr(u8)]
pub enum SerializeError {
    BufferTooSmall = 0,
}

/// Represents deserialization errors for PRNGs.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[reprc]
#[repr(u8)]
pub enum DeserializeError {
    BufferTooSmall = 0,
    InvalidFormat = 1,
}
