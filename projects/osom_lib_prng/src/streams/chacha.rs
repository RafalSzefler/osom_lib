#![allow(
    clippy::cast_possible_truncation,
)]

use core::{convert::Infallible, mem::MaybeUninit};

use osom_lib_reprc::macros::reprc;
use osom_lib_try_clone::TryClone;

use crate::{
    aligned_array::AlignArray_4, prngs::SplitMix64, traits::{PRStream, PRNGenerator, Seedable}
};

/// The Osom imlementation of `ChaCha` algorithm, as defined
/// in [RFC 8439](https://www.rfc-editor.org/rfc/rfc8439).
/// 
/// [`ChaChaStream`] is cryptographically secure stream.
/// 
/// Note that the recommended `ROUNDS` value is 20.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[reprc]
#[must_use]
pub struct ChaChaStream<const ROUNDS: u32> {
    // The data necessary to generate blocks
    key: [u32; 8],
    nonce: [u32; 3],
    counter: u32,

    // Current buffer to serve data from
    buffer: [u8; 64],
    buffer_len: u32,
}

impl<const ROUNDS: u32> TryClone for ChaChaStream<ROUNDS> {
    type Error = Infallible;

    fn try_clone(&self) -> Result<Self, Self::Error> {
        Ok(*self)
    }
}

impl<const ROUNDS: u32> ChaChaStream<ROUNDS> {
    /// Creates a new [`ChaChaStream`] from a given seed. This method
    /// puts the seed into the key, and passes 0 as the nonce.
    /// 
    /// For better control use [`Self::from_arrays`] or [`Self::from_slices`].
    pub fn from_seed(seed: u128) -> Self {
        let mut key = [0u8; 32];
        let seed_bytes = seed.to_le_bytes();
        let mut index = 0usize;
        while index < size_of::<u128>() {
            key[index] = seed_bytes[index];
            index += 1;
        }

        #[allow(clippy::cast_possible_truncation)]
        let mut mixer = SplitMix64::with_seed(seed as u64);

        let nonce = mixer.generate::<[u8; 12]>();
        Self::from_arrays(key, nonce)
    }

    /// Creates a new [`ChaChaStream`] from a given key and nonce.
    #[inline(always)]
    pub fn from_arrays(key: [u8; 32], nonce: [u8; 12]) -> Self {
        Self::from_slices(&key, &nonce)
    }

    /// Creates a new [`ChaChaStream`] from a given key and nonce slices.
    #[inline(always)]
    pub fn from_slices(key: &[u8], nonce: &[u8]) -> Self {
        Self::from_slices_and_counter(key, nonce, 0)
    }

    /// Creates a new [`ChaChaStream`] from a given key, nonce slices
    /// and internal counter.
    /// 
    /// # Panics
    /// 
    /// Panics if the key or nonce is not of size 32 or 12 respectively.
    /// Also when counter is `u32::MAX`.
    pub fn from_slices_and_counter(key: &[u8], nonce: &[u8], counter: u32) -> Self {
        const {
            assert!(ROUNDS >= 8, "ChaCha rounds must be at least 8, otherwise it won't be secure. The recommended value is 20.");
            assert!(ROUNDS <= 1000, "ChaCha rounds must be at most 1000. That number is definitely too much for any purpose. The recommended value is 20.");
        }
        assert!(key.len() == 32, "ChaCha key must be of size 32");
        assert!(nonce.len() == 12, "ChaCha nonce must be of size 12");
        assert!(counter < u32::MAX, "ChaCha counter must be smaller than u32::MAX. It is recommended to make it 0 or 1 or something low. Otherwise overflow will occur fast.");

        // We are going to reinterpre u8 slices as u32 slices. And thus we need to
        // ensure they are properly aligned to 4.
        let aligned_key = AlignArray_4::<32>::from_slice(key);
        let aligned_nonce = AlignArray_4::<12>::from_slice(nonce);

        let mut real_key = MaybeUninit::<[u32; 8]>::uninit();
        let mut real_nonce = MaybeUninit::<[u32; 3]>::uninit();

        let mut stream;
        #[allow(clippy::needless_range_loop)]
        unsafe {
            let aligned_key = aligned_key.as_slice();
            let aligned_nonce = aligned_nonce.as_slice();
            let real_key_ptr = real_key.as_mut_ptr().cast::<u32>();
            let real_nonce_ptr = real_nonce.as_mut_ptr().cast::<u32>();
            for idx in 0..8 {
                real_key_ptr.add(idx).write(from_le_u32(aligned_key, idx * 4));
            }
            for idx in 0..3 {
                real_nonce_ptr.add(idx).write(from_le_u32(aligned_nonce, idx * 4));
            }

            stream = Self { key: real_key.assume_init(), nonce: real_nonce.assume_init(), counter, buffer: [0u8; 64], buffer_len: 0 };
        }

        stream.buffer = Self::serialize_block(&stream.next_u32_block());
        stream
    }

    /// Generates the next chacha block.
    /// 
    /// # Panics
    /// 
    /// When the internal counter overflows, i.e. when this function
    /// is called more than `u32::MAX` times.
    #[must_use]
    pub fn next_u32_block(&mut self) -> [u32; 16] {
        assert!(self.counter != u32::MAX, "ChaCha stream counter overflow.");
        let counter = self.counter;
        self.counter += 1;
        calculate_chacha_block(self.key, self.nonce, counter, ROUNDS)
    }

    /// Serializes the given chacha block, by applying little endian encoding to each element.
    #[must_use]
    pub fn serialize_block(block: &[u32; 16]) -> [u8; 64] {
        let mut result = MaybeUninit::<[u8; 64]>::uninit();
        let mut ptr = result.as_mut_ptr().cast::<u8>();
        for item in block {
            unsafe {
                let bytes = item.to_le_bytes();
                let bytes_ptr = (&raw const bytes).cast();
                ptr.copy_from_nonoverlapping(bytes_ptr, size_of::<u32>());
                ptr = ptr.add(size_of::<u32>());
            }
        }

        unsafe { result.assume_init() }
    }
}

impl<const ROUNDS: u32> PRStream for ChaChaStream<ROUNDS> {
    unsafe fn fill_raw(&mut self, dst_ptr: *mut u8, dst_len: usize) {
        let mut dst = dst_ptr;
        let mut len = dst_len;

        // Fill from whatever it is missing in the current block
        let diff = 64 - self.buffer_len;
        let to_write = core::cmp::min(len, diff as usize);
        unsafe {
            dst.copy_from_nonoverlapping(self.buffer.as_ptr().add(self.buffer_len as usize), to_write);
            dst = dst.add(to_write);
            len -= to_write;
        };

        if len == 0 {
            self.buffer_len += to_write as u32;
            return;
        }

        // Fill the buffer block by block
        self.buffer = Self::serialize_block(&self.next_u32_block());
        self.buffer_len = 0;

        while len > 64 {
            unsafe {
                dst.copy_from_nonoverlapping(self.buffer.as_ptr(), 64);
                dst = dst.add(64);
            };
            self.buffer = Self::serialize_block(&self.next_u32_block());
            self.buffer_len = 0;
            len -= 64;
        }

        // If anything remains, fill it from the current block.
        if len > 0 {
            unsafe {
                dst.copy_from_nonoverlapping(self.buffer.as_ptr(), len);
            };
            self.buffer_len = len as u32;
        }
    }
    

    // #[inline(always)]
    // fn next_block(&mut self) -> Self::BlockType {
    //     Self::serialize_block(&self.next_u32_block())
    // }
}

#[inline(always)]
fn from_le_u32(arr: &[u8], start: usize) -> u32 {
    #[allow(clippy::cast_ptr_alignment)]
    unsafe {
        let ptr = arr.as_ptr().add(start).cast::<u32>();
        debug_assert!(ptr.is_aligned(), "ChaCha: misaligned initial u32 data");
        (*ptr).to_le()
    }
}

impl<const ROUNDS: u32> Seedable<u128> for ChaChaStream<ROUNDS> {
    fn with_seed(seed: u128) -> Self {
        Self::from_seed(seed)
    }
}

impl<const ROUNDS: u32> Seedable<u64> for ChaChaStream<ROUNDS> {
    fn with_seed(seed: u64) -> Self {
        Self::from_seed(u128::from(seed))
    }
}

/// The recommended [`ChaChaStream`] with 20 rounds.
pub type DefaultChaChaStream = ChaChaStream<20>;

macro_rules! qr {
    ($arr: expr, $a:literal, $b:literal, $c:literal, $d:literal) => {{
        $arr[$a] = $arr[$a].wrapping_add($arr[$b]);
        $arr[$d] ^= $arr[$a];
        $arr[$d] = $arr[$d].rotate_left(16);

        $arr[$c] = $arr[$c].wrapping_add($arr[$d]);
        $arr[$b] ^= $arr[$c];
        $arr[$b] = $arr[$b].rotate_left(12);

        $arr[$a] = $arr[$a].wrapping_add($arr[$b]);
        $arr[$d] ^= $arr[$a];
        $arr[$d] = $arr[$d].rotate_left(8);

        $arr[$c] = $arr[$c].wrapping_add($arr[$d]);
        $arr[$b] ^= $arr[$c];
        $arr[$b] = $arr[$b].rotate_left(7);
    }};
}

#[inline(always)]
const fn mutate_state(state: &mut [u32; 16], rounds: u32) {
    let mut index = 0;
    while index < rounds {
        // Odd round
        qr!(state, 0, 4, 8, 12);
        qr!(state, 1, 5, 9, 13);
        qr!(state, 2, 6, 10, 14);
        qr!(state, 3, 7, 11, 15);

        // Even round
        qr!(state, 0, 5, 10, 15);
        qr!(state, 1, 6, 11, 12);
        qr!(state, 2, 7, 8, 13);
        qr!(state, 3, 4, 9, 14);

        index += 2;
    }
}

#[inline(always)]
fn initialize_block(key: [u32; 8], nonce: [u32; 3], counter: u32) -> [u32; 16] {
    let mut block = [0u32; 16];
    block[0] = 0x61707865;
    block[1] = 0x3320646e;
    block[2] = 0x79622d32;
    block[3] = 0x6b206574;
    block[4..12].copy_from_slice(&key);
    block[12] = counter;
    block[13..16].copy_from_slice(&nonce);
    block
}

fn calculate_chacha_block(key: [u32; 8], nonce: [u32; 3], counter: u32, rounds: u32) -> [u32; 16] {
    let block = initialize_block(key, nonce, counter);

    #[allow(clippy::clone_on_copy)]
    let mut working_block = block.clone();

    mutate_state(&mut working_block, rounds);

    for index in 0..block.len() {
        working_block[index] = working_block[index].wrapping_add(block[index]);
    }

    working_block
}
