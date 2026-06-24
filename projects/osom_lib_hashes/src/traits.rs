//! Defines hash function traits.

use core::mem::MaybeUninit;

use osom_lib_reprc::traits::ReprC;

/// Represents a general hash function. Unlike [`core::hash::Hasher`]
/// trait it doesn't mandate output type, except that it needs to
/// be [`AsRef<[u8]>`]. This makes it suitable for implementing larger
/// hash function like SHA.
pub trait HashFunction: ReprC {
    type Output: AsRef<[u8]> + ReprC;

    /// Updates the internal state of the [`HashFunction`] with given data.
    fn update(&mut self, data: impl AsRef<[u8]>);

    /// Writes the final result to the output passed as ref parameter.
    fn write_result(&self, output: &mut Self::Output);

    /// A wrapper around [`HashFunction::write_result`] that returns the hash
    /// instead of writing it to output parameter.
    fn result(&self) -> Self::Output {
        let mut array = MaybeUninit::<Self::Output>::uninit();
        let raw_ref = unsafe { &mut *array.as_mut_ptr() };
        self.write_result(raw_ref);
        unsafe { array.assume_init() }
    }
}

/// A marker trait that ensures that the marked function is
/// cryptographicall secure.
///
/// # Safety
///
/// It is up to the implementor to ensure this invariant.
pub unsafe trait CryptographicallySecureHash: HashFunction {}
