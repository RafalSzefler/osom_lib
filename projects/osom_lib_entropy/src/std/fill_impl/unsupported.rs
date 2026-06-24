use crate::std::StdEntropyError;

#[inline(always)]
pub const fn fill(_: *mut u8, _: usize) -> Result<(), StdEntropyError> {
    Err(StdEntropyError::UnsupportedPlatform)
}
