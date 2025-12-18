use osom_lib_reprc::macros::reprc;

/// Represents possible erros when interacting with [`PowerOfTwo32`][`super::PowerOfTwo32`]
/// and [`PowerOfTwo64`][`super::PowerOfTwo64`].
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[reprc]
#[repr(u8)]
#[must_use]
pub enum PowerOfTwoError {
    NotAPowerOfTwo = 0,
}
