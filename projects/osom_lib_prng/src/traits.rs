//! Holds definitions of various PRNG traits.

use core::ops::RangeBounds;

use osom_lib_reprc::{macros::reprc, traits::ReprC};

use crate::errors::{DeserializeError, SerializeError};

/// Generates pseudo random instance of current type.
///
/// Similar to [`PRNGenerator`] but works with a single type
/// only.
pub trait PRNConcreteGenerator<TGenerator: PRNGenerator>: Sized + ReprC {
    /// Generates a new pseudo random instance of itself.
    #[must_use]
    fn generate(generator: &mut TGenerator) -> Self;
}

/// Represents a trait for generating pseudo-random data
pub trait PRNConcreteBoundedGenerator<TGenerator: PRNGenerator>: Sized + ReprC {
    /// Generates a new pseudo random instance of itself in given range.
    #[must_use]
    fn generate<TBounds: RangeBounds<Self>>(generator: &mut TGenerator, range: TBounds) -> Self;
}

/// Generates pseudo random numbers.
pub trait PRNGenerator: Sized + ReprC + Clone {
    /// Fills the given raw buffer with randomness.
    ///
    /// # Safety
    ///
    /// This function assumes that both `dst_ptr` and `dst_len`
    /// correspond to valid chunk of memory. Otherwise the behaviour
    /// is undefined.
    unsafe fn fill_raw(&mut self, dst_ptr: *mut u8, dst_len: usize);

    /// Fills the given buffer with randomness.
    #[inline(always)]
    fn fill(&mut self, dst: &mut [u8]) {
        unsafe {
            self.fill_raw(dst.as_mut_ptr(), dst.len());
        }
    }

    /// Generates pseudo random instance of given type if supported.
    /// This function should support at least i32, u32, i64 and u64
    /// types.
    #[inline(always)]
    #[must_use]
    fn generate<T>(&mut self) -> T
    where
        T: PRNConcreteGenerator<Self>,
    {
        T::generate(self)
    }

    /// Generates a pseudo random number in given range, if supported.
    #[must_use]
    fn generate_in_range<T, TBounds: RangeBounds<T>>(&mut self, range: TBounds) -> T
    where
        T: PRNConcreteBoundedGenerator<Self>,
    {
        T::generate(self, range)
    }
}

/// A marker trait for cryptographically secure PRNGs.
///
/// # Safety
///
/// It is up to the implementor to ensure this invariant.
pub unsafe trait CryptographicallySecure: PRNGenerator {}

/// A trait for splittable PRNGs. This means that
/// given PRNG can split into two generators. This
/// is different from creating a new PRNG (with a random
/// seed), because splitting is a deterministic process.
///
/// Also this is different from cloning, since clones
/// generally generate the same sequences of numbers
/// (since clones have the exact same state).
/// We want the result of splitting look like
/// it is independent (even though in reality it is not).
///
/// It is especially useful for deterministic, multi-threaded
/// simulations where each thread needs a PRNG.
pub trait Splittable: PRNGenerator {
    /// Create a new PRNG from the current one. In a
    /// deterministic, yet non-obvious (pseudo random) way.
    #[must_use]
    fn split(&mut self) -> Self;
}

/// Represents deserialization result.
#[derive(Debug)]
#[reprc]
pub struct DeserializationResult<T: ReprC> {
    /// The real amount of bytes read from the buffer.
    pub read_bytes: usize,

    /// The actual deserialized value.
    pub value: T,
}

/// Represents platform independent (de)serialization trait
/// of given pseudo random number generator.
pub trait PRNGSerialize: PRNGenerator {
    /// Concrete serialization error.
    type SerializeError: Into<SerializeError> + ReprC;

    /// Concrete deserialization error.
    type DeserializeError: Into<DeserializeError> + ReprC;

    /// The maximum size of serialized value, in bytes. This should be used by callers
    /// to determine how big the buffer needs to be for [`PRNGSerialize::serialize`]
    /// to succeed.
    ///
    /// Typically this should be in the order of few kb max
    /// (e.g. for the 128-bit linear congruential generator that would be the current
    /// state, i.e. only 16 bytes if we don't count the multiplier and the increment).
    /// But of course there is no such guarantee in general.
    const MAX_SERIALIZED_SIZE: usize;

    /// Serializes self into the buffer. Returns the actual amount of bytes that
    /// were written on success.
    ///
    /// # Errors
    ///
    /// For errors see concrete [`PRNGSerialize::SerializeError`] type.
    fn serialize(&self, buffer: &mut [u8]) -> Result<usize, Self::SerializeError>;

    /// Deserializes self from the passed buffer. Returns deserialization info
    /// on success, i.e. the actual value and the amount of bytes read from the
    /// buffer.
    ///
    /// # Errors
    ///
    /// For errors see concrete [`PRNGSerialize::DeserializeError`] type.
    fn deserialize(buffer: &[u8]) -> Result<DeserializationResult<Self>, Self::DeserializeError>;
}

/// Represents a pseudo random stream. These are easily convertible
/// into PRNGs through the [`StreamPRNG`][`crate::stream_prng::StreamPRNG`] wrapper.
pub trait PRStream: Sized + Clone + ReprC {
    /// Fills the given raw buffer with randomness.
    ///
    /// # Safety
    ///
    /// This function assumes that both `dst_ptr` and `dst_len`
    /// correspond to valid chunk of memory. Otherwise the behaviour
    /// is undefined.
    unsafe fn fill_raw(&mut self, dst_ptr: *mut u8, dst_len: usize);

    /// Fills the given block with randomness.
    #[inline(always)]
    fn fill_block(&mut self, block: &mut [u8]) {
        unsafe {
            self.fill_raw(block.as_mut_ptr(), block.len());
        }
    }

    /// Generates the next block of given length.
    #[inline(always)]
    #[must_use]
    fn next_block<const N: usize>(&mut self) -> [u8; N] {
        let mut block = [0u8; N];
        self.fill_block(&mut block);
        block
    }
}

/// A trait for seeding PRNGs.
pub trait Seedable<TSeed: ReprC + Copy> {
    /// Creates a new instance from a given seed.
    #[must_use]
    fn with_seed(seed: TSeed) -> Self;
}
