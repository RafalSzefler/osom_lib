#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
)]

use core::ops::RangeBounds;

use osom_lib_reprc::macros::reprc;

use crate::{
    errors::{DeserializeError, SerializeError},
    prngs::helpers::{
        calculate_crc8, fill_raw_from_array_generator, generate_f32_in_range, generate_f64_in_range,
        generate_i32_in_range, generate_i64_in_range, generate_u32_in_range, generate_u64_in_range
    },
    traits::{
        DeserializationResult, PRNConcreteBoundedGenerator, PRNConcreteGenerator, PRNGSerialize, PRNGenerator, Seedable, Splittable
    }
};

/// The general linear congruential generator. Internally it holds
/// `u128` state and follows the `next_state = A*old_state + C` formula
/// (over 128-bit modulo arithmetic).
/// 
/// Additionally all LCGs implement [`Splittable`] through an ad-hoc method.
/// However users should be warned that statistical properties of this method
/// are barely verified.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[reprc]
#[repr(transparent)]
#[must_use]
pub struct GeneralLinearCongruentialGenerator128<const A: u128, const C: u128> {
    state: u128,
}

impl<const A: u128, const C: u128> GeneralLinearCongruentialGenerator128<A, C> {
    /// Generates a new pseudo-random `u128` value.
    #[inline(always)]
    pub const fn next(&mut self) -> u128 {
        let new_value = self.state.wrapping_mul(A).wrapping_add(C);
        self.state = new_value;
        new_value
    }
}

impl<const A: u128, const C: u128> Seedable<u128> for GeneralLinearCongruentialGenerator128<A, C> {
    fn with_seed(seed: u128) -> Self {
        let state = core::hint::select_unpredictable(seed != 0, seed, 1);
        Self { state }
    }
}

impl<const A: u128, const C: u128> Seedable<u64> for GeneralLinearCongruentialGenerator128<A, C> {
    fn with_seed(seed: u64) -> Self {
        let mut splitmix = crate::prngs::SplitMix64::with_seed(seed);
        let upper = u128::from(splitmix.next());
        let lower = u128::from(splitmix.next());
        Self::with_seed((upper << 64) + lower)
    }
}

impl<const A: u128, const C: u128> PRNGenerator for GeneralLinearCongruentialGenerator128<A, C> {
    unsafe fn fill_raw(&mut self, dst_ptr: *mut u8, dst_len: usize) {
        fill_raw_from_array_generator(|| self.next().to_le_bytes(), dst_ptr, dst_len);
    }
}

impl<const A: u128, const C: u128> PRNConcreteGenerator<GeneralLinearCongruentialGenerator128<A, C>> for bool {
    fn generate(generator: &mut GeneralLinearCongruentialGenerator128<A, C>) -> Self {
        (generator.next() & 1) == 1
    }
}

impl<const N: usize, const A: u128, const C: u128> PRNConcreteGenerator<GeneralLinearCongruentialGenerator128<A, C>> for [u8; N] {
    fn generate(generator: &mut GeneralLinearCongruentialGenerator128<A, C>) -> Self {
        if N == 0 {
            return [0u8; N];
        }
        let mut item = core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            generator.fill_raw(item.as_mut_ptr().cast(), size_of::<Self>());
            item.assume_init()
        }
    }
}

impl<const A: u128, const C: u128> PRNConcreteGenerator<GeneralLinearCongruentialGenerator128<A, C>> for u8 {
    fn generate(generator: &mut GeneralLinearCongruentialGenerator128<A, C>) -> Self {
        u8::from_le_bytes(generator.generate::<[u8; 1]>())
    }
}

impl<const A: u128, const C: u128> PRNConcreteGenerator<GeneralLinearCongruentialGenerator128<A, C>> for i8 {
    fn generate(generator: &mut GeneralLinearCongruentialGenerator128<A, C>) -> Self {
        i8::from_le_bytes(generator.generate::<[u8; 1]>())
    }
}

impl<const A: u128, const C: u128> PRNConcreteGenerator<GeneralLinearCongruentialGenerator128<A, C>> for u32 {
    fn generate(generator: &mut GeneralLinearCongruentialGenerator128<A, C>) -> Self {
        u32::from_le_bytes(generator.generate::<[u8; 4]>())
    }
}

impl<const A: u128, const C: u128> PRNConcreteGenerator<GeneralLinearCongruentialGenerator128<A, C>> for i32 {
    fn generate(generator: &mut GeneralLinearCongruentialGenerator128<A, C>) -> Self {
        i32::from_le_bytes(generator.generate::<[u8; 4]>())
    }
}

impl<const A: u128, const C: u128> PRNConcreteGenerator<GeneralLinearCongruentialGenerator128<A, C>> for u64 {
    fn generate(generator: &mut GeneralLinearCongruentialGenerator128<A, C>) -> Self {
        u64::from_le_bytes(generator.generate::<[u8; 8]>())
    }
}

impl<const A: u128, const C: u128> PRNConcreteGenerator<GeneralLinearCongruentialGenerator128<A, C>> for i64 {
    fn generate(generator: &mut GeneralLinearCongruentialGenerator128<A, C>) -> Self {
        i64::from_le_bytes(generator.generate::<[u8; 8]>())
    }
}

impl<const A: u128, const C: u128> PRNConcreteGenerator<GeneralLinearCongruentialGenerator128<A, C>> for u128 {
    fn generate(generator: &mut GeneralLinearCongruentialGenerator128<A, C>) -> Self {
        generator.next()
    }
}

impl<const A: u128, const C: u128> PRNConcreteGenerator<GeneralLinearCongruentialGenerator128<A, C>> for i128 {
    fn generate(generator: &mut GeneralLinearCongruentialGenerator128<A, C>) -> Self {
        generator.generate::<u128>() as i128
    }
}


impl<const A: u128, const C: u128> PRNGSerialize for GeneralLinearCongruentialGenerator128<A, C> {
    type SerializeError = SerializeError;

    type DeserializeError = DeserializeError;

    const MAX_SERIALIZED_SIZE: usize = size_of::<u128>() + 4;

    fn serialize(&self, buffer: &mut [u8]) -> Result<usize, Self::SerializeError> {
        if buffer.len() < Self::MAX_SERIALIZED_SIZE {
            return Err(SerializeError::BufferTooSmall);
        }
        buffer[0] = b'L';
        buffer[1] = b'C';
        buffer[2] = b'G';
        let state = self.state.to_le_bytes();
        buffer[3] = calculate_crc8(&state);
        buffer[4..Self::MAX_SERIALIZED_SIZE].copy_from_slice(&state);
        Ok(Self::MAX_SERIALIZED_SIZE)
    }

    fn deserialize(buffer: &[u8]) -> Result<DeserializationResult<Self>, Self::DeserializeError> {
        if buffer.len() < Self::MAX_SERIALIZED_SIZE {
            return Err(DeserializeError::BufferTooSmall);
        }

        if buffer[0..3] != [b'L', b'C', b'G'] {
            return Err(DeserializeError::InvalidFormat);
        }

        let crc8_expected_value = buffer[3];

        let mut data = [0u8; size_of::<u128>()];
        data.copy_from_slice(&buffer[4..Self::MAX_SERIALIZED_SIZE]);
        let crc8_value = calculate_crc8(&data);
        if crc8_expected_value != crc8_value {
            return Err(DeserializeError::InvalidFormat);
        }

        let state = u128::from_le_bytes(data);

        let result = DeserializationResult {
            read_bytes: Self::MAX_SERIALIZED_SIZE,
            value: Self::with_seed(state),
        };
        Ok(result)
    }
}

impl<const A: u128, const C: u128> Splittable for GeneralLinearCongruentialGenerator128<A, C> {
    fn split(&mut self) -> Self {
        let next = self.generate::<u64>();
        Self::with_seed(next)
    }
}

impl<const A: u128, const C: u128> PRNConcreteBoundedGenerator<GeneralLinearCongruentialGenerator128<A, C>> for u32 {
    fn generate<TBounds: RangeBounds<Self>>(generator: &mut GeneralLinearCongruentialGenerator128<A, C>, range: TBounds) -> Self {
        generate_u32_in_range(generator, range)
    }
}

impl<const A: u128, const C: u128> PRNConcreteBoundedGenerator<GeneralLinearCongruentialGenerator128<A, C>> for u64 {
    fn generate<TBounds: RangeBounds<Self>>(generator: &mut GeneralLinearCongruentialGenerator128<A, C>, range: TBounds) -> Self {
        generate_u64_in_range(generator, range)
    }
}

impl<const A: u128, const C: u128> PRNConcreteBoundedGenerator<GeneralLinearCongruentialGenerator128<A, C>> for i32 {
    fn generate<TBounds: RangeBounds<Self>>(generator: &mut GeneralLinearCongruentialGenerator128<A, C>, range: TBounds) -> Self {
        generate_i32_in_range(generator, range)
    }
}

impl<const A: u128, const C: u128> PRNConcreteBoundedGenerator<GeneralLinearCongruentialGenerator128<A, C>> for i64 {
    fn generate<TBounds: RangeBounds<Self>>(generator: &mut GeneralLinearCongruentialGenerator128<A, C>, range: TBounds) -> Self {
        generate_i64_in_range(generator, range)
    }
}


impl<const A: u128, const C: u128> PRNConcreteBoundedGenerator<GeneralLinearCongruentialGenerator128<A, C>> for f32 {
    fn generate<TBounds: RangeBounds<Self>>(generator: &mut GeneralLinearCongruentialGenerator128<A, C>, range: TBounds) -> Self {
        generate_f32_in_range(generator, range)
    }
}

impl<const A: u128, const C: u128> PRNConcreteBoundedGenerator<GeneralLinearCongruentialGenerator128<A, C>> for f64 {
    fn generate<TBounds: RangeBounds<Self>>(generator: &mut GeneralLinearCongruentialGenerator128<A, C>, range: TBounds) -> Self {
        generate_f64_in_range(generator, range)
    }
}

/// Alias for the default 128-bit [`GeneralLinearCongruentialGenerator128`]
/// with carefully chosen `A` and `C`.
/// 
/// # Notes
/// 
/// The `A` multiplier is chosen based on "Computationally Easy, Spectrally
/// Good Multipliers for Congruential Pseudorandom Number Generators" paper
/// by Guy Steele & Sebastiano Vigna.
/// 
/// The `C` constant is a prime not dividing `A`, chosen based on
/// "The Art of Computer Programming, Volume 2: Seminumerical Algorithms"
/// by Donald E. Knuth.
/// 
/// With those parameters, and thanks to the internal state being 128-bit, the
/// generator has a excelent statistical properties. And can be safely used
/// in any simulation scenario. Note that it is not cryptographically secure.
pub type LinearCongruentialGenerator128 = GeneralLinearCongruentialGenerator128<
    0xdb36357734e34abb0050d0761fcdfc15,
    0x86e9>;
