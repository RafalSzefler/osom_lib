use windows_sys::Win32::Security::Cryptography::ProcessPrng;

use crate::std::StdEntropyError;

#[inline(always)]
pub fn fill(dst_ptr: *mut u8, dst_len: usize) -> Result<(), StdEntropyError> {
    let result = unsafe {
        ProcessPrng(dst_ptr.cast(), dst_len)
    };
    debug_assert_eq!(result, 1);
    Ok(())
}
