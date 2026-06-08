use libc::getrandom;

use crate::std::StdEntropyError;

#[inline]
pub fn fill(dst_ptr: *mut u8, dst_len: usize) -> Result<(), StdEntropyError> {
    let mut ptr = dst_ptr;
    let mut len = dst_len;
    while len > 0 {
        let result = unsafe {
            getrandom(ptr.cast(), len, 0)
        };
        if result <= 0 {
            if result < 0 {
                return Err(StdEntropyError::GenericKernelError);
            }
            continue;
        }

        let offset = result.cast_unsigned();
        unsafe {
            ptr = ptr.add(offset);
        }
        len -= offset;
    }
    Ok(())
}
