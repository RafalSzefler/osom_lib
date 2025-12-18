//! Holds the implementation of the CPRNG entropy generator.

use core::convert::Infallible;

use osom_lib_entropy::{
    std::StdEntropyGenerator,
    traits::{EntropyConcreteGenerator, EntropyGenerator},
};
use osom_lib_prng::traits::{CryptographicallySecure, PRNGenerator, Seedable};
use osom_lib_reprc::macros::reprc;

/// Represents a generic entropy generator, which generates randomness
/// from a cryptographically secure PRNG, but is seed from the actual
/// OS entropy.
#[reprc]
#[repr(transparent)]
#[derive(Clone)]
pub struct CPRNGEntropy<TPrng>
where
    TPrng: PRNGenerator + CryptographicallySecure + Seedable<u128>,
{
    cprng: TPrng,
}

impl<TPrng> CPRNGEntropy<TPrng>
where
    TPrng: PRNGenerator + CryptographicallySecure + Seedable<u128>,
{
    /// Creates a new instance of [`CPRNGEntropy`]
    ///
    /// # Errors
    ///
    /// These get propagated from [`StdEntropyGenerator`] when
    /// generating a new seed fails.
    pub fn new() -> Result<Self, <StdEntropyGenerator as EntropyGenerator>::Error> {
        let mut entropy = StdEntropyGenerator::new();
        let seed = entropy.generate::<u128>()?;
        let cprng = TPrng::with_seed(seed);
        Ok(Self { cprng })
    }

    /// Generates a new random item.
    #[allow(clippy::missing_panics_doc)]
    #[inline(always)]
    pub fn generate<T: EntropyConcreteGenerator<CPRNGEntropy<TPrng>>>(&mut self) -> T {
        // We explicitly specify the type here, so that unwrap() actually is valid.
        // This is a safeguard against potential future modifications to Error type.
        let result: Result<T, Infallible> = <Self as EntropyGenerator>::generate::<T>(self);
        result.unwrap()
    }
}

impl<TPrng> EntropyGenerator for CPRNGEntropy<TPrng>
where
    TPrng: PRNGenerator + CryptographicallySecure + Seedable<u128>,
{
    /// [`CPRNGEntropy`] cannot fail at randomness generation.
    type Error = Infallible;

    unsafe fn fill_raw(&mut self, buffer_ptr: *mut u8, buffer_len: usize) -> Result<(), Infallible> {
        unsafe {
            self.cprng.fill_raw(buffer_ptr, buffer_len);
        }
        Ok(())
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

impl<const N: usize, TPrng> EntropyConcreteGenerator<CPRNGEntropy<TPrng>> for [u8; N]
where
    TPrng: PRNGenerator + CryptographicallySecure + Seedable<u128>,
{
    #[inline(always)]
    fn generate(generator: &mut CPRNGEntropy<TPrng>) -> Result<Self, <CPRNGEntropy<TPrng> as EntropyGenerator>::Error> {
        if N == 0 {
            return Ok([0u8; N]);
        }
        generate_random_t(generator)
    }
}

macro_rules! concrete {
    ( $t: ty ) => {
        impl<TPrng> EntropyConcreteGenerator<CPRNGEntropy<TPrng>> for $t
        where
            TPrng: PRNGenerator + CryptographicallySecure + Seedable<u128>
        {
            #[inline(always)]
            fn generate(generator: &mut CPRNGEntropy<TPrng>) -> Result<Self, <CPRNGEntropy<TPrng> as EntropyGenerator>::Error>
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
