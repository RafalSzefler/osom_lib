//! Holds the implementation of the `SipHash` algorithm.

#![allow(clippy::cast_possible_truncation, clippy::doc_markdown)]

use core::hash::{BuildHasher, Hasher};

use osom_lib_arrays::const_helpers::from_le_const_u64;
use osom_lib_reprc::macros::reprc;

use crate::traits::HashFunction;

/// Packs up to 7 bytes from `buf[start..start+len]` into a little-endian u64.
const fn u8to64_le(buf: &[u8], start: usize, len: usize) -> u64 {
    let mut out: u64 = 0;
    let mut i = 0;
    while i < len {
        out |= (buf[start + i] as u64) << (i * 8);
        i += 1;
    }
    out
}

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
    /// Current state stored as [v0, v2, v1, v3] so the algorithm's natural
    /// (v0,v2) and (v1,v3) pairs are memory-adjacent for SIMD auto-vectorization.
    state: [u64; 4],

    /// Unprocessed bytes packed as a little-endian u64.
    tail: u64,

    /// How many bytes are valid in `tail` (0..=7).
    ntail: u8,

    /// How many bytes have been hashed so far, mod 256.
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
            // [v0, v2, v1, v3]: (v0,v2) and (v1,v3) are adjacent pairs.
            state: [
                key0 ^ 0x736f6d6570736575, // v0
                key0 ^ 0x6c7967656e657261, // v2
                key1 ^ 0x646f72616e646f6d, // v1
                key1 ^ 0x7465646279746573, // v3
            ],
            tail: 0,
            ntail: 0,
            last_byte: 0,
        }
    }

    #[inline]
    const fn process_block(state: &mut [u64; 4], value: u64) {
        state[3] ^= value; // v3 ^= value
        sip_rounds::<C>(state);
        state[0] ^= value; // v0 ^= value
    }

    /// Updates the underlying state with the given block.
    ///
    /// # Panics
    ///
    /// If the length of the block exceeds `u32::MAX`.
    pub const fn update_const(&mut self, data: &[u8]) {
        let length = data.len();

        assert!(
            length <= u32::MAX as usize,
            "The max size of a chunk for SipHash is u32::MAX."
        );

        self.last_byte = self.last_byte.wrapping_add(length as u8);

        let mut data_offset = 0usize;

        if self.ntail != 0 {
            let needed = 8 - self.ntail as usize;
            let fill_len = if length < needed { length } else { needed };
            self.tail |= u8to64_le(data, 0, fill_len) << (8 * self.ntail as u32);
            if length < needed {
                self.ntail += length as u8;
                return;
            }
            Self::process_block(&mut self.state, self.tail);
            self.ntail = 0;
            self.tail = 0;
            data_offset = needed;
        }

        let remaining = length - data_offset;
        let left = remaining & 0x7;
        let end = data_offset + remaining - left;

        while data_offset < end {
            let mi = unsafe { from_le_const_u64(data, data_offset) };
            Self::process_block(&mut self.state, mi);
            data_offset += 8;
        }

        self.tail = u8to64_le(data, data_offset, left);
        self.ntail = left as u8;
    }

    /// Calculates the final hash value.
    ///
    /// This function does not update the internal state, and thus
    /// [`SipHash`] can still be used afterwards.
    #[must_use]
    pub const fn result_const(&self) -> u64 {
        let mut state = self.state;
        let b: u64 = ((self.last_byte as u64) << 56) | self.tail;

        state[3] ^= b;
        sip_rounds::<C>(&mut state);
        state[0] ^= b;

        state[1] ^= 0xff; // v2 ^= 0xff (v2 is at index 1 in [v0, v2, v1, v3])
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

#[inline]
const fn sip_rounds<const ROUNDS: u32>(state: &mut [u64; 4]) {
    // State layout: [v0, v2, v1, v3] at indices [0, 1, 2, 3].
    let mut index = 0;
    while index < ROUNDS {
        state[0] = state[0].wrapping_add(state[2]);
        state[2] = state[2].rotate_left(13);
        state[2] ^= state[0];
        state[0] = state[0].rotate_left(32);
        state[1] = state[1].wrapping_add(state[3]);
        state[3] = state[3].rotate_left(16);
        state[3] ^= state[1];
        state[0] = state[0].wrapping_add(state[3]);
        state[3] = state[3].rotate_left(21);
        state[3] ^= state[0];
        state[1] = state[1].wrapping_add(state[2]);
        state[2] = state[2].rotate_left(17);
        state[2] ^= state[1];
        state[1] = state[1].rotate_left(32);
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
