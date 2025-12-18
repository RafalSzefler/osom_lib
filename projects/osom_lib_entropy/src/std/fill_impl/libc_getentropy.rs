use libc::getentropy;

use crate::std::StdEntropyError;

const MAX_GETENTROPY_LENGTH: usize = 256;

#[inline(always)]
pub fn fill(dst_ptr: *mut u8, dst_len: usize) -> Result<(), StdEntropyError> {
    let mut ptr = dst_ptr;
    let mut length = dst_len;
    while length > MAX_GETENTROPY_LENGTH {
        let result = unsafe {
            getentropy(ptr.cast(), MAX_GETENTROPY_LENGTH)
        };
        if result != 0 {
            return Err(StdEntropyError::GenericKernelError);
        }
        length -= MAX_GETENTROPY_LENGTH;
        ptr = unsafe { ptr.add(MAX_GETENTROPY_LENGTH) };
    }
    if length > 0 {
        let result = unsafe {
            getentropy(ptr.cast(), length)
        };
        if result != 0 {
            return Err(StdEntropyError::GenericKernelError);
        }
    }
    Ok(())
}
