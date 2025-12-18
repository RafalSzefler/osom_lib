use libc::getrandom;

use crate::std::StdEntropyError;

#[inline(always)]
pub fn fill(dst_ptr: *mut u8, dst_len: usize) -> Result<(), StdEntropyError> {
    let result = unsafe {
        getrandom(dst_ptr.cast(), dst_len, 0)
    };
    if result == -1 {
        return Err(StdEntropyError::GenericKernelError);
    }
    Ok(())
}
