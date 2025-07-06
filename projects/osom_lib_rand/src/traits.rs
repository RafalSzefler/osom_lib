//! Holds traits for random number generators and randomness sources.
use crate::number::{MAX_NUMBER_SIZE, Number};

/// Simple trait for pseudo random number generators. Types implementing
/// this trait should aim for efficiency above all.
pub trait PseudoRandomGenerator {
    type TNumber: Number;

    /// Creates a new [`PseudoRandomGenerator`] seeded from a [`RandomnessSource`].
    fn from_randomness_source(source: &mut impl RandomnessSource<TNumber = Self::TNumber>) -> Self;

    /// Returns the next random number.
    fn next_number(&mut self) -> Self::TNumber;

    /// Fills the given mut slice with random bytes.
    fn fill_bytes(&mut self, bytes: &mut [u8]) {
        fill_bytes_from_gens(bytes, || self.next_number());
    }
}

/// Simple trait for randomness source.
///
/// This looks (and in fact is)
/// the same as [`PseudoRandomGenerator`], however it is distinguished
/// by the way it works. It is allowed for implementors of this trait
/// to be inefficient, but have potentially better randomness (e.g. by
/// reading from appropriate hardware source).
///
/// It is also a good idea to seed [`PseudoRandomGenerator`] objects
/// with whatever [`RandomnessSource`] returns.
pub trait RandomnessSource: Default {
    type TNumber: Number;

    /// Returns the next random number.
    fn next_number(&mut self) -> Self::TNumber;

    /// Fills the given mut slice with random bytes.
    fn fill_bytes(&mut self, bytes: &mut [u8]) {
        fill_bytes_from_gens(bytes, || self.next_number());
    }
}

fn fill_bytes_from_gens<T: Number, F: FnMut() -> T>(bytes: &mut [u8], mut generator: F) {
    let size = T::SIZE;
    let bytes_len = bytes.len();
    let number_of_chunks = bytes_len / size;
    let missing_elements = bytes_len % size;
    let mut ptr = bytes.as_mut_ptr().cast::<T>();
    for _ in 0..number_of_chunks {
        unsafe {
            *ptr = generator();
            ptr = ptr.add(1);
        }
    }
    if missing_elements > 0 {
        let value = generator();
        let mut value_bytes = [0u8; MAX_NUMBER_SIZE];
        unsafe {
            let value_ptr = value_bytes.as_mut_ptr().cast::<T>();
            *value_ptr = value;
        }
        let index = number_of_chunks * size;
        let remaining_bytes = &mut bytes[index..(index + missing_elements)];
        remaining_bytes.copy_from_slice(&value_bytes[..missing_elements]);
    }
}
