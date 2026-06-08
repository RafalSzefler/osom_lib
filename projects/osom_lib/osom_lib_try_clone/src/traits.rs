/// A trait for types that can be cloned with a result.
pub trait TryClone: Sized {
    type Error: core::fmt::Debug;

    /// Tries to clone the type.
    ///
    /// # Errors
    ///
    /// Returns the error if the clone fails for whatever reason.
    fn try_clone(&self) -> Result<Self, Self::Error>;
}
