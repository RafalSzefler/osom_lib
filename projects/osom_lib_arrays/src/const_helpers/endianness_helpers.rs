use osom_lib_macros::debug_check_or_release_hint;

/// Converts slice with offset to a little-endian u64.
///
/// # Safety
///
/// This function does not verify the out-of-bounds access.
/// It is up to the caller to ensure that the range is valid.
///
/// # Panics
///
/// It will panic on out-of-bounds access, but only in debug mode.
#[allow(private_bounds)]
#[inline(always)]
#[must_use]
pub const unsafe fn from_le_const_u64(slice: &[u8], start: usize) -> u64 {
    debug_check_or_release_hint!(slice.len() >= start + size_of::<u64>());
    let array_ptr = unsafe { slice.as_ptr().add(start) }.cast::<[u8; size_of::<u64>()]>();
    u64::from_le_bytes(unsafe { *array_ptr })
}

/// Converts slice with offset to a big-endian u32.
///
/// # Safety
///
/// This function does not verify the out-of-bounds access.
/// It is up to the caller to ensure that the range is valid.
///
/// # Panics
///
/// It will panic on out-of-bounds access, but only in debug mode.
#[allow(private_bounds)]
#[inline(always)]
#[must_use]
pub const unsafe fn from_be_const_u32(slice: &[u8], start: usize) -> u32 {
    debug_check_or_release_hint!(slice.len() >= start + size_of::<u32>());
    let array_ptr = unsafe { slice.as_ptr().add(start) }.cast::<[u8; size_of::<u32>()]>();
    u32::from_be_bytes(unsafe { *array_ptr })
}
