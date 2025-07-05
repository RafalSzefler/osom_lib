/// An object implementing this trait can be converted to a raw mut pointer.
///
/// # Safety
///
/// This trait doesn't impose any safety guarantees on the returned pointer,
/// it is up to the caller to ensure that the object is valid
/// and that the pointer is used correctly. In particular no lifetime checks
/// are performed.
pub unsafe trait Pointerable {
    /// Reinterprets `&self` as `*mut u8`.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it allows converting an object to a pointer.
    /// The caller must ensure that the object is valid and that the pointer is
    /// used correctly. In particular no lifetime checks are performed.
    unsafe fn as_ptr(&self) -> *mut u8;
}
