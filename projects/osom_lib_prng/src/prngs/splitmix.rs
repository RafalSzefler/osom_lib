#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
)]

use core::{convert::Infallible, ops::RangeBounds};

use osom_lib_reprc::macros::reprc;
use osom_lib_try_clone::TryClone;

use crate::{
    prngs::helpers::{
        fill_raw_from_array_generator, generate_f32_in_range, generate_f64_in_range, generate_i32_in_range,
        generate_i64_in_range, generate_u32_in_range, generate_u64_in_range
    },
    traits::{PRNConcreteBoundedGenerator, PRNConcreteGenerator, PRNGenerator, Seedable},
};

/// The Rust imlementation of `SplitMix64` algorithm. It is good
/// for generating seeds and good enough general purpose PRNG.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[reprc]
#[repr(transparent)]
#[must_use]
pub struct SplitMix64 {
    state: u64
}

impl TryClone for SplitMix64 {
    type Error = Infallible;

    fn try_clone(&self) -> Result<Self, Self::Error> {
        Ok(*self)
    }
}

impl SplitMix64 {
    /// Creates a new [`SplitMix64`] from a seed.
    #[inline(always)]
    pub const fn with_seed(seed: u64) -> Self {
        let state = if seed != 0 { seed } else { 1 };
        Self { state }
    }

    /// Generates a new pseudo-random u64.
    pub const fn next(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9e3779b97f4a7c15);
        let mut result = self.state;
        result = (result ^ (result >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        result = (result ^ (result >> 27)).wrapping_mul(0x94d049bb133111eb);
        result ^ (result >> 31)
    }
}

impl Seedable<u64> for SplitMix64 {
    fn with_seed(seed: u64) -> Self {
        Self::with_seed(seed)
    }
}

impl PRNGenerator for SplitMix64 {
    unsafe fn fill_raw(&mut self, dst_ptr: *mut u8, dst_len: usize) {
        fill_raw_from_array_generator(|| self.next().to_be_bytes(), dst_ptr, dst_len);
    }
}

impl PRNConcreteGenerator<SplitMix64> for bool {
    fn generate(generator: &mut SplitMix64) -> Self {
        (generator.next() & 1) == 1
    }
}

impl<const N: usize> PRNConcreteGenerator<SplitMix64> for [u8; N] {
    fn generate(generator: &mut SplitMix64) -> Self {
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

impl PRNConcreteGenerator<SplitMix64> for u8 {
    fn generate(generator: &mut SplitMix64) -> Self {
        u8::from_le_bytes(generator.generate::<[u8; 1]>())
    }
}

impl PRNConcreteGenerator<SplitMix64> for i8 {
    fn generate(generator: &mut SplitMix64) -> Self {
        i8::from_le_bytes(generator.generate::<[u8; 1]>())
    }
}


impl PRNConcreteGenerator<SplitMix64> for u32 {
    fn generate(generator: &mut SplitMix64) -> Self {
        u32::from_le_bytes(generator.generate::<[u8; 4]>())
    }
}

impl PRNConcreteGenerator<SplitMix64> for i32 {
    fn generate(generator: &mut SplitMix64) -> Self {
        i32::from_le_bytes(generator.generate::<[u8; 4]>())
    }
}

impl PRNConcreteGenerator<SplitMix64> for u64 {
    fn generate(generator: &mut SplitMix64) -> Self {
        u64::from_le_bytes(generator.generate::<[u8; 8]>())
    }
}

impl PRNConcreteGenerator<SplitMix64> for i64 {
    fn generate(generator: &mut SplitMix64) -> Self {
        i64::from_le_bytes(generator.generate::<[u8; 8]>())
    }
}

impl PRNConcreteBoundedGenerator<SplitMix64> for u32 {
    fn generate<TBounds: RangeBounds<Self>>(generator: &mut SplitMix64, range: TBounds) -> Self {
        generate_u32_in_range(generator, range)
    }
}

impl PRNConcreteBoundedGenerator<SplitMix64> for u64 {
    fn generate<TBounds: RangeBounds<Self>>(generator: &mut SplitMix64, range: TBounds) -> Self {
        generate_u64_in_range(generator, range)
    }
}

impl PRNConcreteBoundedGenerator<SplitMix64> for i32 {
    fn generate<TBounds: RangeBounds<Self>>(generator: &mut SplitMix64, range: TBounds) -> Self {
        generate_i32_in_range(generator, range)
    }
}

impl PRNConcreteBoundedGenerator<SplitMix64> for i64 {
    fn generate<TBounds: RangeBounds<Self>>(generator: &mut SplitMix64, range: TBounds) -> Self {
        generate_i64_in_range(generator, range)
    }
}

impl PRNConcreteBoundedGenerator<SplitMix64> for f32 {
    fn generate<TBounds: RangeBounds<Self>>(generator: &mut SplitMix64, range: TBounds) -> Self {
        generate_f32_in_range(generator, range)
    }
}

impl PRNConcreteBoundedGenerator<SplitMix64> for f64 {
    fn generate<TBounds: RangeBounds<Self>>(generator: &mut SplitMix64, range: TBounds) -> Self {
        generate_f64_in_range(generator, range)
    }
}
