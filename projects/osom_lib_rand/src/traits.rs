//! Holds traits for random number generators and randomness sources.
use crate::number::Number;

/// Simple trait for pseudo random number generators. Types implementing
/// this trait should aim for efficiency above all.
///
/// In its essence this is the same as [`RandomnessSource`].
/// We distinguish those types mostly for type safety. These
/// have different purposes.
pub trait PseudoRandomNumberGenerator {
    type TNumber: Number;

    /// Whether the generator is cryptographically secure.
    const IS_CRYPTOGRAPHICALLY_SECURE: bool = false;

    /// Creates a new [`PseudoRandomNumberGenerator`] seeded from a [`RandomnessSource`].
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
/// the same as [`PseudoRandomNumberGenerator`], however it is distinguished
/// by the way it works. It is allowed for implementors of this trait
/// to be inefficient, but have potentially better randomness (e.g. by
/// reading from appropriate hardware source).
///
/// It is also a good idea to seed [`PseudoRandomNumberGenerator`] objects
/// with whatever [`RandomnessSource`] returns.
///
/// In its essence this is the same as [`PseudoRandomNumberGenerator`].
/// We distinguish those types mostly for type safety. These
/// have different purposes.
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
    if bytes.is_empty() {
        return;
    }

    let size = T::SIZE;
    let bytes_len = bytes.len();
    let number_of_chunks = bytes_len / size;
    let missing_elements = bytes_len % size;
    let mut ptr = bytes.as_mut_ptr().cast::<T>();
    for _ in 0..number_of_chunks {
        unsafe {
            ptr.write(generator());
            ptr = ptr.add(1);
        }
    }

    if missing_elements > 0 {
        let mut value_bytes = T::ByteRepr::default();
        let value_bytes_slice = value_bytes.as_mut();
        let value_ptr = value_bytes_slice.as_mut_ptr().cast::<T>();
        unsafe {
            value_ptr.write(generator());
        }
        let index = number_of_chunks * size;
        let remaining_bytes = &mut bytes[index..(index + missing_elements)];
        remaining_bytes.copy_from_slice(&value_bytes_slice[..missing_elements]);
    }
}
