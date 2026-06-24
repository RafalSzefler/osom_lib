//! Holds the implementation of the `FxHash` algorithm.

use core::hash::{BuildHasher, Hasher};

use osom_lib_reprc::macros::reprc;

use crate::traits::HashFunction;

const MULTIPLIER: u64 = 0x517cc1b727220a95;

#[inline(always)]
const fn mix(state: u64, word: u64) -> u64 {
    (state.rotate_left(15) ^ word).wrapping_mul(MULTIPLIER)
}

/// Implementation of the `FxHash` algorithm.
///
/// This is a fast, non-cryptographic hash function,
/// suitable for use in hash tables.
#[reprc]
#[must_use]
pub struct FxHash {
    state: u64,
    pending: u64,
    pending_len: u8,
}

impl FxHash {
    /// Creates a new [`FxHash`] instance with the default initial state.
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            state: 0,
            pending: 0,
            pending_len: 0,
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
        let mut i = 0usize;

        // Fill pending word if partially full
        if self.pending_len > 0 {
            while i < data.len() && self.pending_len < 8 {
                self.pending |= (data[i] as u64) << (self.pending_len as u32 * 8);
                self.pending_len += 1;
                i += 1;
            }
            if self.pending_len == 8 {
                self.state = mix(self.state, self.pending);
                self.pending = 0;
                self.pending_len = 0;
            }
        }

        // Process full 8-byte chunks
        while i + 8 <= data.len() {
            let word = u64::from_le_bytes([
                data[i],
                data[i + 1],
                data[i + 2],
                data[i + 3],
                data[i + 4],
                data[i + 5],
                data[i + 6],
                data[i + 7],
            ]);
            self.state = mix(self.state, word);
            i += 8;
        }

        // Accumulate remaining bytes into pending
        while i < data.len() {
            self.pending |= (data[i] as u64) << (self.pending_len as u32 * 8);
            self.pending_len += 1;
            i += 1;
        }
    }

    /// Returns the final hash value.
    ///
    /// This function does not update the internal state, and thus
    /// [`FxHash`] can still be used afterwards.
    #[must_use]
    #[inline(always)]
    pub const fn result_const(&self) -> u64 {
        if self.pending_len == 0 {
            return self.state;
        }
        mix(self.state, self.pending)
    }

    /// Clones the [`FxHash`] instance.
    #[inline(always)]
    pub const fn clone_const(&self) -> Self {
        Self {
            state: self.state,
            pending: self.pending,
            pending_len: self.pending_len,
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
