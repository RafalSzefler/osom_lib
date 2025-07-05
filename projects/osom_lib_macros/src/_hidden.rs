/// Wrapper around a simple function call.
///
/// This is used in order to avoid "unnecessary unsafe" warnings.
#[inline(always)]
pub fn call<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    f()
}
