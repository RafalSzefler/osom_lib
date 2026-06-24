use windows_sys::Win32::Security::Cryptography::ProcessPrng;

use crate::std::StdEntropyError;

#[inline]
pub fn fill(dst_ptr: *mut u8, dst_len: usize) -> Result<(), StdEntropyError> {
    let result = unsafe {
        ProcessPrng(dst_ptr.cast(), dst_len)
    };
    if result != 1 {
        return Err(StdEntropyError::GenericKernelError);
    }
    Ok(())
}
