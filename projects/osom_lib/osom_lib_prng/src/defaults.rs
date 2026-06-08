//! Holds the implementation of the default PRNGs, recommended for general use.

use core::ops::RangeBounds;

use osom_lib_reprc::macros::reprc;
use osom_lib_try_clone::TryClone;

use crate::{
    prngs::{ChaCha, LinearCongruentialGenerator128},
    traits::{
        CryptographicallySecure, PRNConcreteBoundedGenerator, PRNConcreteGenerator, PRNGenerator, Seedable, Splittable,
    },
};

/// The recommended [`PRNGenerator`] for fast PRNG.
///
/// At the moment it uses [`LinearCongruentialGenerator128`] as an underlying PRNG.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[reprc]
#[repr(transparent)]
#[must_use]
pub struct DefaultFastPRNG {
    prng: LinearCongruentialGenerator128,
}

impl TryClone for DefaultFastPRNG {
    type Error = <LinearCongruentialGenerator128 as TryClone>::Error;

    fn try_clone(&self) -> Result<Self, Self::Error> {
        let prng = self.prng.try_clone()?;
        Ok(Self { prng })
    }
}

impl PRNGenerator for DefaultFastPRNG {
    unsafe fn fill_raw(&mut self, dst_ptr: *mut u8, dst_len: usize) {
        unsafe { self.prng.fill_raw(dst_ptr, dst_len) };
    }
}

impl<const N: usize> PRNConcreteGenerator<DefaultFastPRNG> for [u8; N] {
    #[inline(always)]
    fn generate(generator: &mut DefaultFastPRNG) -> Self {
        generator.prng.generate::<Self>()
    }
}

impl PRNConcreteGenerator<DefaultFastPRNG> for bool {
    fn generate(generator: &mut DefaultFastPRNG) -> Self {
        generator.prng.generate::<bool>()
    }
}

impl PRNConcreteGenerator<DefaultFastPRNG> for u8 {
    fn generate(generator: &mut DefaultFastPRNG) -> Self {
        generator.prng.generate::<u8>()
    }
}

impl PRNConcreteGenerator<DefaultFastPRNG> for i8 {
    fn generate(generator: &mut DefaultFastPRNG) -> Self {
        generator.prng.generate::<i8>()
    }
}

impl PRNConcreteGenerator<DefaultFastPRNG> for u32 {
    fn generate(generator: &mut DefaultFastPRNG) -> Self {
        generator.prng.generate::<u32>()
    }
}

impl PRNConcreteGenerator<DefaultFastPRNG> for i32 {
    fn generate(generator: &mut DefaultFastPRNG) -> Self {
        generator.prng.generate::<i32>()
    }
}

impl PRNConcreteGenerator<DefaultFastPRNG> for u64 {
    fn generate(generator: &mut DefaultFastPRNG) -> Self {
        generator.prng.generate::<u64>()
    }
}

impl PRNConcreteGenerator<DefaultFastPRNG> for i64 {
    fn generate(generator: &mut DefaultFastPRNG) -> Self {
        generator.prng.generate::<i64>()
    }
}

impl PRNConcreteGenerator<DefaultFastPRNG> for u128 {
    fn generate(generator: &mut DefaultFastPRNG) -> Self {
        generator.prng.generate::<u128>()
    }
}

impl PRNConcreteGenerator<DefaultFastPRNG> for i128 {
    fn generate(generator: &mut DefaultFastPRNG) -> Self {
        generator.prng.generate::<i128>()
    }
}

impl PRNConcreteBoundedGenerator<DefaultFastPRNG> for u32 {
    fn generate<TBounds: RangeBounds<Self>>(generator: &mut DefaultFastPRNG, range: TBounds) -> Self {
        generator.prng.generate_in_range(range)
    }
}

impl PRNConcreteBoundedGenerator<DefaultFastPRNG> for u64 {
    fn generate<TBounds: RangeBounds<Self>>(generator: &mut DefaultFastPRNG, range: TBounds) -> Self {
        generator.prng.generate_in_range(range)
    }
}

impl PRNConcreteBoundedGenerator<DefaultFastPRNG> for i32 {
    fn generate<TBounds: RangeBounds<Self>>(generator: &mut DefaultFastPRNG, range: TBounds) -> Self {
        generator.prng.generate_in_range(range)
    }
}

impl PRNConcreteBoundedGenerator<DefaultFastPRNG> for i64 {
    fn generate<TBounds: RangeBounds<Self>>(generator: &mut DefaultFastPRNG, range: TBounds) -> Self {
        generator.prng.generate_in_range(range)
    }
}

impl PRNConcreteBoundedGenerator<DefaultFastPRNG> for f32 {
    fn generate<TBounds: RangeBounds<Self>>(generator: &mut DefaultFastPRNG, range: TBounds) -> Self {
        generator.prng.generate_in_range(range)
    }
}

impl PRNConcreteBoundedGenerator<DefaultFastPRNG> for f64 {
    fn generate<TBounds: RangeBounds<Self>>(generator: &mut DefaultFastPRNG, range: TBounds) -> Self {
        generator.prng.generate_in_range(range)
    }
}

impl Seedable<u128> for DefaultFastPRNG {
    fn with_seed(seed: u128) -> Self {
        Self {
            prng: LinearCongruentialGenerator128::with_seed(seed),
        }
    }
}

impl Seedable<u64> for DefaultFastPRNG {
    fn with_seed(seed: u64) -> Self {
        Self {
            prng: LinearCongruentialGenerator128::with_seed(seed),
        }
    }
}

impl Splittable for DefaultFastPRNG {
    fn split(&mut self) -> Self {
        Self {
            prng: self.prng.split(),
        }
    }
}

/// The recommended [`PRNGenerator`] PRNG for cryptographically secure purposes.
///
/// At the moment it uses [`ChaCha<20>`] as an underlying PRNG.
#[derive(Debug, PartialEq, Eq, Clone)]
#[reprc]
#[repr(transparent)]
#[must_use]
pub struct DefaultSecurePRNG {
    prng: ChaCha<20>,
}

unsafe impl CryptographicallySecure for DefaultSecurePRNG {}

impl PRNGenerator for DefaultSecurePRNG {
    unsafe fn fill_raw(&mut self, dst_ptr: *mut u8, dst_len: usize) {
        unsafe { self.prng.fill_raw(dst_ptr, dst_len) };
    }
}

impl<const N: usize> PRNConcreteGenerator<DefaultSecurePRNG> for [u8; N] {
    #[inline(always)]
    fn generate(generator: &mut DefaultSecurePRNG) -> Self {
        generator.prng.generate::<Self>()
    }
}

impl PRNConcreteGenerator<DefaultSecurePRNG> for bool {
    fn generate(generator: &mut DefaultSecurePRNG) -> Self {
        generator.prng.generate::<bool>()
    }
}

impl PRNConcreteGenerator<DefaultSecurePRNG> for u8 {
    fn generate(generator: &mut DefaultSecurePRNG) -> Self {
        generator.prng.generate::<u8>()
    }
}

impl PRNConcreteGenerator<DefaultSecurePRNG> for i8 {
    fn generate(generator: &mut DefaultSecurePRNG) -> Self {
        generator.prng.generate::<i8>()
    }
}

impl PRNConcreteGenerator<DefaultSecurePRNG> for u32 {
    fn generate(generator: &mut DefaultSecurePRNG) -> Self {
        generator.prng.generate::<u32>()
    }
}

impl PRNConcreteGenerator<DefaultSecurePRNG> for i32 {
    fn generate(generator: &mut DefaultSecurePRNG) -> Self {
        generator.prng.generate::<i32>()
    }
}

impl PRNConcreteGenerator<DefaultSecurePRNG> for u64 {
    fn generate(generator: &mut DefaultSecurePRNG) -> Self {
        generator.prng.generate::<u64>()
    }
}

impl PRNConcreteGenerator<DefaultSecurePRNG> for i64 {
    fn generate(generator: &mut DefaultSecurePRNG) -> Self {
        generator.prng.generate::<i64>()
    }
}

impl PRNConcreteGenerator<DefaultSecurePRNG> for u128 {
    fn generate(generator: &mut DefaultSecurePRNG) -> Self {
        generator.prng.generate::<u128>()
    }
}

impl PRNConcreteGenerator<DefaultSecurePRNG> for i128 {
    fn generate(generator: &mut DefaultSecurePRNG) -> Self {
        generator.prng.generate::<i128>()
    }
}

impl PRNConcreteBoundedGenerator<DefaultSecurePRNG> for u32 {
    fn generate<TBounds: RangeBounds<Self>>(generator: &mut DefaultSecurePRNG, range: TBounds) -> Self {
        generator.prng.generate_in_range(range)
    }
}

impl PRNConcreteBoundedGenerator<DefaultSecurePRNG> for u64 {
    fn generate<TBounds: RangeBounds<Self>>(generator: &mut DefaultSecurePRNG, range: TBounds) -> Self {
        generator.prng.generate_in_range(range)
    }
}

impl PRNConcreteBoundedGenerator<DefaultSecurePRNG> for i32 {
    fn generate<TBounds: RangeBounds<Self>>(generator: &mut DefaultSecurePRNG, range: TBounds) -> Self {
        generator.prng.generate_in_range(range)
    }
}

impl PRNConcreteBoundedGenerator<DefaultSecurePRNG> for i64 {
    fn generate<TBounds: RangeBounds<Self>>(generator: &mut DefaultSecurePRNG, range: TBounds) -> Self {
        generator.prng.generate_in_range(range)
    }
}

impl PRNConcreteBoundedGenerator<DefaultSecurePRNG> for f32 {
    fn generate<TBounds: RangeBounds<Self>>(generator: &mut DefaultSecurePRNG, range: TBounds) -> Self {
        generator.prng.generate_in_range(range)
    }
}

impl PRNConcreteBoundedGenerator<DefaultSecurePRNG> for f64 {
    fn generate<TBounds: RangeBounds<Self>>(generator: &mut DefaultSecurePRNG, range: TBounds) -> Self {
        generator.prng.generate_in_range(range)
    }
}

impl Seedable<u128> for DefaultSecurePRNG {
    fn with_seed(seed: u128) -> Self {
        Self {
            prng: ChaCha::with_seed(seed),
        }
    }
}

impl Seedable<u64> for DefaultSecurePRNG {
    fn with_seed(seed: u64) -> Self {
        Self {
            prng: ChaCha::with_seed(seed),
        }
    }
}

impl Splittable for DefaultSecurePRNG {
    fn split(&mut self) -> Self {
        Self {
            prng: self.prng.split(),
        }
    }
}
