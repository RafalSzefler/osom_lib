//! Holds the implementation of the stream-based PRNG.

use core::ops::RangeBounds;

use osom_lib_reprc::{macros::reprc, traits::ReprC};
use osom_lib_try_clone::TryClone;

use crate::{
    prngs::helpers::{
        generate_f32_in_range, generate_f64_in_range, generate_i32_in_range, generate_i64_in_range,
        generate_u32_in_range, generate_u64_in_range,
    },
    traits::{PRNConcreteBoundedGenerator, PRNConcreteGenerator, PRNGenerator, PRStream, Seedable},
};

/// The standard stream-based PRNG. It accepts a stream and implements
/// the [`PRNGenerator`] trait on top of it.
#[derive(Debug, PartialEq, Eq, Clone)]
#[reprc]
pub struct StreamPRNG<T: PRStream> {
    stream: T,
}

impl<T: PRStream + TryClone> TryClone for StreamPRNG<T> {
    type Error = <T as TryClone>::Error;

    fn try_clone(&self) -> Result<Self, Self::Error> {
        let stream = self.stream.try_clone()?;
        Ok(Self { stream })
    }
}

impl<T: PRStream> StreamPRNG<T> {
    /// Creates a new [`StreamPRNG`] from a given stream.
    #[inline(always)]
    pub fn new(stream: T) -> Self {
        Self { stream }
    }
}

#[allow(clippy::cast_possible_truncation)]
impl<T: PRStream> PRNGenerator for StreamPRNG<T> {
    unsafe fn fill_raw(&mut self, dst_ptr: *mut u8, dst_len: usize) {
        unsafe { self.stream.fill_raw(dst_ptr, dst_len) };
    }
}

impl<const N: usize, T: PRStream> PRNConcreteGenerator<StreamPRNG<T>> for [u8; N] {
    #[inline(always)]
    fn generate(generator: &mut StreamPRNG<T>) -> Self {
        const {
            assert!(size_of::<Self>() == N);
        }
        if N == 0 {
            return [0u8; N];
        }
        let mut item = core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            generator.fill_raw(item.as_mut_ptr().cast(), N);
            item.assume_init()
        }
    }
}

impl<T: PRStream> PRNConcreteGenerator<StreamPRNG<T>> for bool {
    fn generate(generator: &mut StreamPRNG<T>) -> Self {
        (generator.generate::<u8>() & 1) == 1
    }
}

impl<T: PRStream> PRNConcreteGenerator<StreamPRNG<T>> for u8 {
    fn generate(generator: &mut StreamPRNG<T>) -> Self {
        u8::from_le_bytes(generator.generate::<[u8; 1]>())
    }
}

impl<T: PRStream> PRNConcreteGenerator<StreamPRNG<T>> for i8 {
    fn generate(generator: &mut StreamPRNG<T>) -> Self {
        i8::from_le_bytes(generator.generate::<[u8; 1]>())
    }
}

impl<T: PRStream> PRNConcreteGenerator<StreamPRNG<T>> for u16 {
    fn generate(generator: &mut StreamPRNG<T>) -> Self {
        u16::from_le_bytes(generator.generate::<[u8; 2]>())
    }
}

impl<T: PRStream> PRNConcreteGenerator<StreamPRNG<T>> for i16 {
    fn generate(generator: &mut StreamPRNG<T>) -> Self {
        i16::from_le_bytes(generator.generate::<[u8; 2]>())
    }
}

impl<T: PRStream> PRNConcreteGenerator<StreamPRNG<T>> for u32 {
    fn generate(generator: &mut StreamPRNG<T>) -> Self {
        u32::from_le_bytes(generator.generate::<[u8; 4]>())
    }
}

impl<T: PRStream> PRNConcreteGenerator<StreamPRNG<T>> for i32 {
    fn generate(generator: &mut StreamPRNG<T>) -> Self {
        i32::from_le_bytes(generator.generate::<[u8; 4]>())
    }
}

impl<T: PRStream> PRNConcreteGenerator<StreamPRNG<T>> for u64 {
    fn generate(generator: &mut StreamPRNG<T>) -> Self {
        u64::from_le_bytes(generator.generate::<[u8; 8]>())
    }
}

impl<T: PRStream> PRNConcreteGenerator<StreamPRNG<T>> for i64 {
    fn generate(generator: &mut StreamPRNG<T>) -> Self {
        i64::from_le_bytes(generator.generate::<[u8; 8]>())
    }
}

impl<T: PRStream> PRNConcreteGenerator<StreamPRNG<T>> for u128 {
    fn generate(generator: &mut StreamPRNG<T>) -> Self {
        u128::from_le_bytes(generator.generate::<[u8; 16]>())
    }
}

impl<T: PRStream> PRNConcreteGenerator<StreamPRNG<T>> for i128 {
    fn generate(generator: &mut StreamPRNG<T>) -> Self {
        i128::from_le_bytes(generator.generate::<[u8; 16]>())
    }
}

impl<T: PRStream> PRNConcreteBoundedGenerator<StreamPRNG<T>> for u32 {
    fn generate<TBounds: RangeBounds<Self>>(generator: &mut StreamPRNG<T>, range: TBounds) -> Self {
        generate_u32_in_range(generator, range)
    }
}

impl<T: PRStream> PRNConcreteBoundedGenerator<StreamPRNG<T>> for u64 {
    fn generate<TBounds: RangeBounds<Self>>(generator: &mut StreamPRNG<T>, range: TBounds) -> Self {
        generate_u64_in_range(generator, range)
    }
}

impl<T: PRStream> PRNConcreteBoundedGenerator<StreamPRNG<T>> for i32 {
    fn generate<TBounds: RangeBounds<Self>>(generator: &mut StreamPRNG<T>, range: TBounds) -> Self {
        generate_i32_in_range(generator, range)
    }
}

impl<T: PRStream> PRNConcreteBoundedGenerator<StreamPRNG<T>> for i64 {
    fn generate<TBounds: RangeBounds<Self>>(generator: &mut StreamPRNG<T>, range: TBounds) -> Self {
        generate_i64_in_range(generator, range)
    }
}

impl<T: PRStream> PRNConcreteBoundedGenerator<StreamPRNG<T>> for f32 {
    fn generate<TBounds: RangeBounds<Self>>(generator: &mut StreamPRNG<T>, range: TBounds) -> Self {
        generate_f32_in_range(generator, range)
    }
}

impl<T: PRStream> PRNConcreteBoundedGenerator<StreamPRNG<T>> for f64 {
    fn generate<TBounds: RangeBounds<Self>>(generator: &mut StreamPRNG<T>, range: TBounds) -> Self {
        generate_f64_in_range(generator, range)
    }
}

impl<TSeed: ReprC + Copy, T: PRStream + Seedable<TSeed>> Seedable<TSeed> for StreamPRNG<T> {
    fn with_seed(seed: TSeed) -> Self {
        Self::new(T::with_seed(seed))
    }
}
