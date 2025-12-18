//! Holds the implementation of the `FxHash` algorithm.

use core::hash::{BuildHasher, Hasher};

use osom_lib_arrays::{
    const_helpers::{fill_const, from_le_const_u64, subslice_mut_const},
    fixed_array::ConstBuffer,
};
use osom_lib_reprc::macros::reprc;

use crate::traits::HashFunction;

const MULTIPLIER: u64 = 0x517cc1b727220a95;

/// Implementation of the `FxHash` algorithm.
///
/// This is a fast, non-cryptographic hash function,
/// suitable for use in hash tables.
#[reprc]
#[must_use]
pub struct FxHash {
    state: u64,
    bufferer: ConstBuffer<8, u8>,
}

macro_rules! update_slice {
    ($state:expr, $slice:expr) => {{
        let word = unsafe { from_le_const_u64(($slice), 0) };
        (($state).rotate_left(15) ^ word).wrapping_mul(MULTIPLIER)
    }};
}

impl FxHash {
    /// Creates a new [`FxHash`] instance with the default initial state.
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            state: 0,
            bufferer: ConstBuffer::new(),
        }
    }

    /// Creates a new [`FxHash`] instance with the given seed.
    #[inline]
    pub const fn with_seed(seed: u64) -> Self {
        let mut hasher = Self::new();
        hasher.update_const(&seed.to_le_bytes());
        hasher
    }

    /// Updates the internal state with the given data.
    pub const fn update_const(&mut self, data: &[u8]) {
        let mut iterator = self.bufferer.buffer_const(data);
        while let Some(block) = iterator.next() {
            self.state = update_slice!(self.state, block);
        }
    }

    /// Returns the final hash value.
    ///
    /// This function does not update the internal state, and thus
    /// [`FxHash`] can still be used afterwards.
    #[must_use]
    pub const fn result_const(&self) -> u64 {
        if self.bufferer.length().as_usize() == 0 {
            return self.state;
        }
        let current_bufferer = self.bufferer.clone_const();
        let mut current_array = current_bufferer.release_const();
        let current_array_len = current_array.length().as_usize();
        let raw_current_array = unsafe { current_array.as_raw_slice_mut_const() };
        unsafe {
            fill_const(subslice_mut_const(raw_current_array, current_array_len..8), 0);
        }
        update_slice!(self.state, raw_current_array)
    }

    /// Clones the [`FxHash`] instance.
    #[inline(always)]
    pub const fn clone_const(&self) -> Self {
        Self {
            state: self.state,
            bufferer: self.bufferer.clone_const(),
        }
    }
}

impl Clone for FxHash {
    #[inline(always)]
    fn clone(&self) -> Self {
        self.clone_const()
    }
}

impl Default for FxHash {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl HashFunction for FxHash {
    type Output = [u8; 8];

    #[inline(always)]
    fn update(&mut self, data: impl AsRef<[u8]>) {
        self.update_const(data.as_ref());
    }

    #[inline(always)]
    fn write_result(&self, output: &mut Self::Output) {
        *output = self.result_const().to_le_bytes();
    }
}

impl Hasher for FxHash {
    #[inline(always)]
    fn finish(&self) -> u64 {
        self.result_const()
    }

    #[inline(always)]
    fn write(&mut self, bytes: &[u8]) {
        self.update_const(bytes);
    }
}

/// Represents a builder for [`FxHash`].
#[reprc]
#[repr(transparent)]
#[must_use]
pub struct FxHashBuilder {
    inner: FxHash,
}

impl Clone for FxHashBuilder {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone_const(),
        }
    }
}

impl FxHashBuilder {
    /// Creates a new [`FxHashBuilder`] instance with the default initial state.
    #[inline(always)]
    pub const fn new() -> Self {
        Self { inner: FxHash::new() }
    }

    /// Updates the seed for the [`FxHash`] instance.
    #[inline(always)]
    pub const fn set_seed(mut self, seed: u64) -> Self {
        self.inner = FxHash::with_seed(seed);
        self
    }

    /// Creates a new [`FxHash`] instance from the builder.
    #[inline(always)]
    pub const fn create_hasher(&self) -> FxHash {
        self.inner.clone_const()
    }
}

impl BuildHasher for FxHashBuilder {
    type Hasher = FxHash;

    #[inline(always)]
    fn build_hasher(&self) -> Self::Hasher {
        self.create_hasher()
    }
}

impl Default for FxHashBuilder {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}
