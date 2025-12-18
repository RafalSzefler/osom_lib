use core::ops::Range;

use osom_lib_macros::debug_check_or_release_hint;

/// A const variant of `&slice[range]`.
///
/// # Panics
///
/// This function will panic if the range is invalid. This check
/// is only performed in debug mode.
///
/// # Safety
///
/// This function does not verify the out-of-bounds access.
/// It is up to the caller to ensure that the range is valid.
pub const unsafe fn subslice_const<T>(slice: &[T], range: Range<usize>) -> &[T] {
    let start = range.start;
    let end = range.end;
    let len = slice.len();
    debug_check_or_release_hint!((start <= end) && (start < len) && (end <= len));
    unsafe { core::slice::from_raw_parts(slice.as_ptr().add(start), end - start) }
}

/// A const variant of `&mut slice[range]`.
///
/// # Panics
///
/// This function will panic if the range is invalid. This check
/// is only performed in debug mode.
///
/// # Safety
///
/// This function does not verify the out-of-bounds access.
/// It is up to the caller to ensure that the range is valid.
pub const unsafe fn subslice_mut_const<T>(slice: &mut [T], range: Range<usize>) -> &mut [T] {
    let start = range.start;
    let end = range.end;
    let len = slice.len();
    debug_check_or_release_hint!((start <= end) && (start < len) && (end <= len));
    unsafe { core::slice::from_raw_parts_mut(slice.as_mut_ptr().add(start), end - start) }
}

/// Fills the given slice with the given, copyable value.
///
/// # Notes
///
/// This requires `T: Copy` constraint, because it is const.
#[inline(always)]
pub const fn fill_const<T: Copy>(slice: &mut [T], value: T) {
    let mut index = 0;
    let end = slice.len();
    while index < end {
        slice[index] = value;
        index += 1;
    }
}
