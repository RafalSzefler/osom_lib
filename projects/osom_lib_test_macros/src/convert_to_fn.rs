#[macro_export]
#[doc(hidden)]
macro_rules! _fn_signature {
    ($abi:literal, fn() -> $ret:ty) => {
        unsafe extern $abi fn() -> $ret
    };
    ($abi:literal, fn($arg_type:ty) -> $ret:ty) => {
        unsafe extern $abi fn($arg_type) -> $ret
    };
    ($abi:literal, fn($arg_type:ty $(,$arg_type2:ty)*) -> $ret:ty) => {
        unsafe extern $abi fn($arg_type $(,$arg_type2)*) -> $ret
    };
}

/// Transforms pointerable into a function pointer.
///
/// # Arguments
///
/// * `abi` - The ABI of the function, e.g. `"sysv64"`.
/// * `pointerable` - The pointerable to transform into a function pointer. This
///   is any object that implements [`crate::traits::Pointerable`] trait.
/// * `offset` - the offset in the instructions `u8` buffer to jump to, relative
///   to the beginning of the buffer.
/// * `tokens` - optional function signature, e.g. `fn(i32, bool) -> u8`.
#[macro_export]
macro_rules! convert_to_fn_with_offset {
    ($abi:literal, $pointerable:expr, $offset:expr, $($tokens:tt)*) => {
        $crate::hidden::call(|| {
            unsafe {
                type _FnSignature = $crate::_fn_signature!($abi, $($tokens)*);
                let _ptr = $crate::traits::Pointerable::as_ptr(&($pointerable))
                    .add(($offset) as usize);
                return core::mem::transmute::<*mut u8, _FnSignature>(_ptr);
            }
        })
    };
    ($abi: literal, $pointerable:expr, $offset:expr) => {
        $crate::convert_to_fn_with_offset!($abi, $pointerable, $offset, fn() -> i64)
    };
}

/// Transforms pointerable into a function pointer.
///
/// # Arguments
///
/// * `abi` - The ABI of the function, e.g. `"sysv64"`.
/// * `pointerable` - The pointerable to transform into a function pointer. This
///   is any object that implements [`crate::traits::Pointerable`] trait.
/// * `tokens` - optional function signature, e.g. `fn(i32, bool) -> u8`.
#[macro_export]
macro_rules! convert_to_fn {
    ($abi:literal, $pointerable:expr, $($tokens:tt)*) => {
        $crate::convert_to_fn_with_offset!($abi, $pointerable, 0, $($tokens)*)
    };
    ($abi:literal, $pointerable:expr) => {
        $crate::convert_to_fn_with_offset!($abi, $pointerable, 0, fn() -> i64)
    };
}
