use osom_lib_reprc::macros::reprc;

/// Represents possible errors when interacting with [`PowerOfTwo32`][`super::PowerOfTwo32`]
/// and [`PowerOfTwo64`][`super::PowerOfTwo64`].
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[reprc]
#[repr(u8)]
#[must_use]
pub enum PowerOfTwoError {
    NotAPowerOfTwo = 0,
}

impl core::fmt::Display for PowerOfTwoError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            PowerOfTwoError::NotAPowerOfTwo => write!(f, "PowerOfTwoError::NotAPowerOfTwo"),
        }
    }
}
