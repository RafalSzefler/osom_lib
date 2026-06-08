//! Holds the implementation of the standard entropy generator, based
//! on the operating system's available entropy sources.

mod fill_impl;

use core::{convert::Infallible, marker::PhantomData};

use osom_lib_reprc::macros::reprc;
use osom_lib_try_clone::TryClone;

use crate::traits::{EntropyConcreteGenerator, EntropyGenerator};

/// An enum for handling various failures of entropy generation.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[reprc]
#[repr(u8)]
pub enum StdEntropyError {
    /// Kernel returned an error.
    GenericKernelError = 0,

    /// The current platform is not supported.
    UnsupportedPlatform = 1,
}

impl TryClone for StdEntropyError {
    type Error = Infallible;
    fn try_clone(&self) -> Result<Self, Self::Error> {
        Ok(*self)
    }
}

osom_lib_macros::unreachable_from_infallible!(StdEntropyError);

impl core::fmt::Display for StdEntropyError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "StdEntropyError::{}",
            match self {
                StdEntropyError::GenericKernelError => "GenericKernelError",
                StdEntropyError::UnsupportedPlatform => "UnsupportedPlatform",
            }
        )
    }
}

/// The standard entropy generator. It uses various os available
/// operations to generate entropy:
///
/// * `macos` <- it utilizes `getentropy` syscall
/// * `linux` <- it utilizes `getrandom` syscall
/// * `windows` <- it utilizes `ProcessPrng` syscall
/// * Other platforms are unsupported and attempting to use the generator will
///   result in a [`StdEntropyError::UnsupportedPlatform`] error.
#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
#[reprc]
#[repr(transparent)]
#[must_use]
pub struct StdEntropyGenerator {
    _phantom: PhantomData<()>,
}

impl TryClone for StdEntropyGenerator {
    type Error = Infallible;
    fn try_clone(&self) -> Result<Self, Self::Error> {
        Ok(*self)
    }
}

impl StdEntropyGenerator {
    /// Creates a new instance of [`StdEntropyGenerator`]. This
    /// method is basically free.
    #[inline(always)]
    pub const fn new() -> Self {
        Self { _phantom: PhantomData }
    }
}

impl EntropyGenerator for StdEntropyGenerator {
    type Error = StdEntropyError;

    unsafe fn fill_raw(&mut self, buffer_ptr: *mut u8, buffer_len: usize) -> Result<(), Self::Error> {
        fill_impl::fill(buffer_ptr, buffer_len)
    }
}

#[inline]
fn generate_random_t<T: Copy, TGen: EntropyGenerator>(gene: &mut TGen) -> Result<T, TGen::Error> {
    let mut item = core::mem::MaybeUninit::<T>::uninit();
    let item_ptr = item.as_mut_ptr();
    let slice = unsafe { core::slice::from_raw_parts_mut(item_ptr.cast::<u8>(), size_of::<T>()) };
    gene.fill(slice)?;
    Ok(unsafe { item.assume_init() })
}

impl<const N: usize> EntropyConcreteGenerator<StdEntropyGenerator> for [u8; N] {
    #[inline(always)]
    fn generate(generator: &mut StdEntropyGenerator) -> Result<Self, StdEntropyError> {
        if N == 0 {
            return Ok([0u8; N]);
        }
        generate_random_t(generator)
    }
}

macro_rules! concrete {
    ( $t: ty ) => {
        impl EntropyConcreteGenerator<StdEntropyGenerator> for $t {
            #[inline(always)]
            fn generate(generator: &mut StdEntropyGenerator) -> Result<Self, StdEntropyError>
            {
                generate_random_t(generator)
            }
        }
    };
    ( $t: ty, $($ts:ty),* $(,)?) => {
        concrete!($t);
        concrete!($($ts),*);
    };
}

concrete!(i16, u16, i32, u32, i64, u64, i128, u128, usize, isize);
