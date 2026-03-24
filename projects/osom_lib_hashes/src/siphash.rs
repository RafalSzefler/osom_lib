//! Holds the implementation of the `SipHash` algorithm.

#![allow(clippy::assign_op_pattern, clippy::cast_possible_truncation, clippy::doc_markdown)]

use core::hash::{BuildHasher, Hasher};

use osom_lib_arrays::{
    const_helpers::{fill_const, from_le_const_u64, subslice_mut_const},
    fixed_array::ConstBuffer,
};
use osom_lib_reprc::macros::reprc;

use crate::traits::HashFunction;

/// Implementation of the `SipHash` algorithm.
///
/// This algorithm is resistant to various hash attacks when C >= 2 and D >= 4.
///
/// # Notes
///
/// The `HashFunction` implementation returns values in little-endian order,
/// and thus is cross-platform (with slightly better performance on little-endian platforms).
///
/// This algorithm is an implementation of the "SipHash: a fast short-input PRF" paper
/// by Jean-Philippe Aumasson and Daniel J. Bernstein.
#[reprc]
#[must_use]
pub struct GeneralSipHash<const C: u32, const D: u32> {
    /// Current state of the hash function.
    state: [u64; 4],

    bufferer: ConstBuffer<8, u8>,

    /// How many bytes have been hashed so far, mod 256. This value is used
    /// at the final step of the algorithm.
    last_byte: u8,
}

unsafe impl<const C: u32, const D: u32> Send for GeneralSipHash<C, D> {}
unsafe impl<const C: u32, const D: u32> Sync for GeneralSipHash<C, D> {}

impl<const C: u32, const D: u32> GeneralSipHash<C, D> {
    /// Creates a new [`GeneralSipHash`] instance from a given array key.
    #[inline]
    pub const fn for_array_key(key: &[u8; 2 * size_of::<u64>()]) -> Self {
        unsafe {
            let k0 = from_le_const_u64(key, 0);
            let k1 = from_le_const_u64(key, size_of::<u64>());
            Self::for_keys(k0, k1)
        }
    }

    /// Creates a new [`GeneralSipHash`] instance from a given slice.
    ///
    /// # Panics
    ///
    /// If the key is not exactly 16 bytes long, this function will panic.
    #[inline]
    pub const fn for_slice_key(key: &[u8]) -> Self {
        assert!(
            key.len() == 2 * size_of::<u64>(),
            "The key must be exactly 16 bytes long."
        );
        unsafe {
            let k0 = from_le_const_u64(key, 0);
            let k1 = from_le_const_u64(key, size_of::<u64>());
            Self::for_keys(k0, k1)
        }
    }

    /// Creates a new [`GeneralSipHash`] instance from a key pair.
    #[inline]
    pub const fn for_keys(key0: u64, key1: u64) -> Self {
        Self {
            state: [
                key0 ^ 0x736f6d6570736575,
                key1 ^ 0x646f72616e646f6d,
                key0 ^ 0x6c7967656e657261,
                key1 ^ 0x7465646279746573,
            ],
            bufferer: ConstBuffer::new(),
            last_byte: 0,
        }
    }

    const fn update_with_full_block(state: &mut [u64; 4], data: &[u8]) {
        debug_assert!(data.len() >= 8, "The block must be at least 8 bytes long.");
        let value = unsafe { from_le_const_u64(data, 0) };
        state[3] = state[3] ^ value;
        sip_rounds::<C>(state);
        state[0] = state[0] ^ value;
    }

    /// Updates the underlying state with the given block.
    ///
    /// # Panics
    ///
    /// If the length of the block exceeds `u32::MAX`.
    pub const fn update_const(&mut self, data: &[u8]) {
        let len = data.len();

        assert!(
            len <= u32::MAX as usize,
            "The max size of a chunk for SipHash is u32::MAX."
        );

        // This is a safety cast, in case `usize` is 32-bit.
        // We need 64-bit in case u32 overflows. While unlikely,
        // better safe than sorry.
        let len = len as u64;

        self.last_byte = self.last_byte.wrapping_add(len as u8);

        let mut iterator = self.bufferer.buffer_const(data);
        while let Some(block) = iterator.next() {
            Self::update_with_full_block(&mut self.state, block);
        }
    }

    /// Calculates the final hash value.
    ///
    /// This function does not update the internal state, and thus
    /// [`SipHash`] can still be used afterwards.
    #[must_use]
    pub const fn result_const(&self) -> u64 {
        // Update the final block
        let mut state = self.state;
        let current_bufferer = self.bufferer.clone_const();
        let mut current_array = current_bufferer.release_const();
        let current_array_len = current_array.length().as_usize();
        let raw_current_array = unsafe { current_array.as_raw_slice_mut_const() };

        unsafe {
            fill_const(subslice_mut_const(raw_current_array, current_array_len..7), 0);
            raw_current_array[7] = self.last_byte;
        }

        Self::update_with_full_block(&mut state, raw_current_array);

        // Finalization
        state[2] = state[2] ^ 0xff;
        sip_rounds::<D>(&mut state);
        state[0] ^ state[1] ^ state[2] ^ state[3]
    }
}

impl<const C: u32, const D: u32> HashFunction for GeneralSipHash<C, D> {
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

const fn sip_rounds<const ROUNDS: u32>(state: &mut [u64; 4]) {
    let mut index = 0;
    while index < ROUNDS {
        state[0] = state[0].wrapping_add(state[1]);
        state[1] = state[1].rotate_left(13);
        state[1] = state[1] ^ state[0];
        state[0] = state[0].rotate_left(32);
        state[2] = state[2].wrapping_add(state[3]);
        state[3] = state[3].rotate_left(16);
        state[3] = state[3] ^ state[2];
        state[0] = state[0].wrapping_add(state[3]);
        state[3] = state[3].rotate_left(21);
        state[3] = state[3] ^ state[0];
        state[2] = state[2].wrapping_add(state[1]);
        state[1] = state[1].rotate_left(17);
        state[1] = state[1] ^ state[2];
        state[2] = state[2].rotate_left(32);
        index += 1;
    }
}

impl<const C: u32, const D: u32> Hasher for GeneralSipHash<C, D> {
    #[inline(always)]
    fn finish(&self) -> u64 {
        self.result_const()
    }

    #[inline(always)]
    fn write(&mut self, bytes: &[u8]) {
        self.update_const(bytes);
    }
}

/// Represents a builder for [`GeneralSipHash`].
#[reprc]
#[must_use]
pub struct GeneralSipHashBuilder<const C: u32, const D: u32> {
    key0: u64,
    key1: u64,
}

impl<const C: u32, const D: u32> GeneralSipHashBuilder<C, D> {
    /// Creates a new [`GeneralSipHashBuilder`] instance with custom keys.
    #[inline(always)]
    pub const fn with_keys(key0: u64, key1: u64) -> Self {
        Self { key0, key1 }
    }

    /// Creates a new [`GeneralSipHashBuilder`] instance from a given array key.
    #[inline]
    pub const fn with_array_key(key: &[u8; 2 * size_of::<u64>()]) -> Self {
        unsafe {
            let k0 = from_le_const_u64(key, 0);
            let k1 = from_le_const_u64(key, size_of::<u64>());
            Self::with_keys(k0, k1)
        }
    }

    /// Creates a new [`GeneralSipHashBuilder`] instance from a given slice.
    ///
    /// # Panics
    ///
    /// If the key is not exactly 16 bytes long, this function will panic.
    #[inline]
    pub const fn with_slice_key(key: &[u8]) -> Self {
        assert!(
            key.len() == 2 * size_of::<u64>(),
            "The key must be exactly 16 bytes long."
        );

        unsafe {
            let k0 = from_le_const_u64(key, 0);
            let k1 = from_le_const_u64(key, size_of::<u64>());
            Self::with_keys(k0, k1)
        }
    }

    /// Creates a new [`GeneralSipHash`] instance from the builder.
    #[inline(always)]
    pub const fn create_hasher(&self) -> GeneralSipHash<C, D> {
        GeneralSipHash::for_keys(self.key0, self.key1)
    }
}

impl<const C: u32, const D: u32> BuildHasher for GeneralSipHashBuilder<C, D> {
    type Hasher = GeneralSipHash<C, D>;

    #[inline(always)]
    fn build_hasher(&self) -> Self::Hasher {
        self.create_hasher()
    }
}

impl<const C: u32, const D: u32> Clone for GeneralSipHashBuilder<C, D> {
    #[inline(always)]
    fn clone(&self) -> Self {
        Self {
            key0: self.key0,
            key1: self.key1,
        }
    }
}

/// The alias for [`GeneralSipHash<2, 4>`], which is an optimal choice of constants.
pub type SipHash = GeneralSipHash<2, 4>;

/// The alias for [`GeneralSipHashBuilder<2, 4>`], which is an optimal choice of constants.
pub type SipHashBuilder = GeneralSipHashBuilder<2, 4>;
