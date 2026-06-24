//! Defines entropy traits.

use osom_lib_reprc::traits::ReprC;

/// Generates random type based on surrounding entropy, i.e.
/// based on what the operating system provides as "true" randomness.
///
/// Similar to [`EntropyGenerator`] but works with a single type
/// only.
pub trait EntropyConcreteGenerator<TGenerator: EntropyGenerator>: Sized + ReprC {
    /// Generates a new random instance of itself.
    ///
    /// # Errors
    ///
    /// The same as [`EntropyGenerator::Error`].
    fn generate(generator: &mut TGenerator) -> Result<Self, TGenerator::Error>;
}

/// Generates randomness based on surrounding entropy, i.e.
/// based on what the operating system provides as "true" randomness.
/// The generator should be expected to be quite slow, but should
/// also provide excelent randomness, as close to "true" randomness
/// as possible.
pub trait EntropyGenerator: Sized + ReprC + Clone {
    /// Represents a failure of entropy generation.
    type Error: ReprC;

    /// Fills raw ptr with randomness based on surrounding entropy.
    ///
    /// # Errors
    ///
    /// See concrete [`EntropyGenerator::Error`].
    ///
    /// # Safety
    ///
    /// This function assumes that both `buffer_ptr` and `buffer_len`
    /// point to a valid chunk of memory. Otherwise the behaviour is
    /// undefined.
    unsafe fn fill_raw(&mut self, buffer_ptr: *mut u8, buffer_len: usize) -> Result<(), Self::Error>;

    /// Fills slice with randomness based on surrounding entropy.
    ///
    /// # Errors
    ///
    /// See concrete [`EntropyGenerator::Error`].
    #[inline(always)]
    fn fill(&mut self, buffer: &mut [u8]) -> Result<(), Self::Error> {
        unsafe { self.fill_raw(buffer.as_mut_ptr(), buffer.len()) }
    }

    /// Generates random instance of given type.
    ///
    /// # Errors
    ///
    /// See concrete [`EntropyGenerator::Error`].
    #[inline(always)]
    fn generate<T>(&mut self) -> Result<T, Self::Error>
    where
        T: EntropyConcreteGenerator<Self>,
    {
        T::generate(self)
    }
}
