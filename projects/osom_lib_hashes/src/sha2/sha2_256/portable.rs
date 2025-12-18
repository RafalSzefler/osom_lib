//! Holds the portable implementation of the `SHA2-256` algorithm.

#![allow(clippy::many_single_char_names)]

use osom_lib_arrays::const_helpers::{from_be_const_u32, subslice_mut_const};
use osom_lib_arrays::fixed_array::ConstBuffer;
use osom_lib_reprc::macros::reprc;

use super::sha2_256_shared::{INITIAL_STATE, K, calculate_final_blocks};
use crate::traits::HashFunction;

const CHUNK_SIZE_U32: usize = 16;

/// A portable implementation of the `SHA2-256` algorithm.
///
/// This implementation is portable, will work on any platform
/// and is const friendly.
///
/// # Notes
///
/// The concrete algorithm follows closely the `SHA2-256` specification
/// taken from [RFC 4634](https://www.rfc-editor.org/rfc/rfc4634).
#[reprc]
#[must_use]
pub struct SHA2_256_Portable {
    // This field is used in the final block calculation.
    // It is first due to its size (we are using #[repr(C)]).
    pub(super) total_length: u64,

    // The internal state of SHA2-256 hasher.
    pub(super) state: [u32; 8],

    // The buffered message block. We need to keep it because final block needs to be processed differently.
    pub(super) bufferer: ConstBuffer<64, u8>,
}

#[inline(always)]
const fn ch(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (!x & z)
}

#[inline(always)]
const fn maj(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (x & z) ^ (y & z)
}

#[inline(always)]
const fn bsig0(x: u32) -> u32 {
    x.rotate_right(2) ^ x.rotate_right(13) ^ x.rotate_right(22)
}

#[inline(always)]
const fn bsig1(x: u32) -> u32 {
    x.rotate_right(6) ^ x.rotate_right(11) ^ x.rotate_right(25)
}

#[inline(always)]
const fn ssig0(x: u32) -> u32 {
    x.rotate_right(7) ^ x.rotate_right(18) ^ (x >> 3)
}

#[inline(always)]
const fn ssig1(x: u32) -> u32 {
    x.rotate_right(17) ^ x.rotate_right(19) ^ (x >> 10)
}

const fn prepare_w_schedule(data: &[u32; 16]) -> [u32; 4 * CHUNK_SIZE_U32] {
    let mut w = [0u32; 4 * CHUNK_SIZE_U32];
    let mut index = 0;
    while index < CHUNK_SIZE_U32 {
        w[index] = data[index];
        index += 1;
    }

    let mut index = CHUNK_SIZE_U32;
    while index < 4 * CHUNK_SIZE_U32 {
        w[index] = ssig1(w[index - 2])
            .wrapping_add(w[index - 7])
            .wrapping_add(ssig0(w[index - 15]))
            .wrapping_add(w[index - 16]);
        index += 1;
    }

    w
}

const fn update_state(state: &mut [u32; 8], data: &[u8; 64]) {
    let data = {
        let mut initial_data = [0u32; CHUNK_SIZE_U32];
        let mut index = 0;
        while index < CHUNK_SIZE_U32 {
            initial_data[index] = unsafe { from_be_const_u32(data, index * 4) };
            index += 1;
        }
        initial_data
    };

    let w = prepare_w_schedule(&data);

    let mut a = state[0];
    let mut b = state[1];
    let mut c = state[2];
    let mut d = state[3];
    let mut e = state[4];
    let mut f = state[5];
    let mut g = state[6];
    let mut h = state[7];

    let mut index = 0;
    while index < 64 {
        let temp1 = h
            .wrapping_add(bsig1(e))
            .wrapping_add(ch(e, f, g))
            .wrapping_add(K[index])
            .wrapping_add(w[index]);
        let temp2 = bsig0(a).wrapping_add(maj(a, b, c));
        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(temp1);
        d = c;
        c = b;
        b = a;
        a = temp1.wrapping_add(temp2);
        index += 1;
    }

    state[0] = state[0].wrapping_add(a);
    state[1] = state[1].wrapping_add(b);
    state[2] = state[2].wrapping_add(c);
    state[3] = state[3].wrapping_add(d);
    state[4] = state[4].wrapping_add(e);
    state[5] = state[5].wrapping_add(f);
    state[6] = state[6].wrapping_add(g);
    state[7] = state[7].wrapping_add(h);
}

impl SHA2_256_Portable {
    /// Creates a new [`SHA2_256_Portable`] instance.
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            total_length: 0,
            state: INITIAL_STATE,
            bufferer: ConstBuffer::new(),
        }
    }

    #[inline(always)]
    pub(super) const fn from_pieces(total_length: u64, state: [u32; 8], bufferer: ConstBuffer<64, u8>) -> Self {
        Self {
            total_length,
            state,
            bufferer,
        }
    }

    /// Writes a block of data to the underlying state.
    ///
    /// # Panics
    ///
    /// This function will panic if the length of the data is greater than `u32::MAX`,
    /// and when the total processed length exceeds `u64::MAX`.
    pub const fn update_const(&mut self, data: &[u8]) {
        let len = data.len();
        if len == 0 {
            return;
        }

        assert!(
            len <= u32::MAX as usize,
            "The max size of a chunk for SHA2_256_Portable is u32::MAX."
        );

        let len: u64 = len as u64;

        assert!(
            self.total_length <= (u64::MAX / 8) - len,
            "The total length of the data that SHA2-256 can process is u64::MAX / 8. This limit is reached."
        );

        // This is safe due to previous assertions.
        self.total_length = unsafe { self.total_length.unchecked_add(len) };

        let mut iterator = self.bufferer.buffer_const(data);
        while let Some(block) = iterator.next() {
            update_state(&mut self.state, block);
        }
    }

    /// Writes the final has value to the passed output.
    ///
    /// # Notes
    ///
    /// This function does not update the internal state, and thus
    /// the [`SHA2_256_Portable`] can still be used afterwards.
    pub const fn write_result_const(&self, output: &mut [u8; 32]) {
        // Build the final SHA2-256 block.
        let mut state = self.state;

        let final_blocks = calculate_final_blocks(self.total_length, self.bufferer.current_state_const());
        let mut tmp_buffer = ConstBuffer::<64, u8>::new();
        let mut iterator = tmp_buffer.buffer_const(final_blocks.as_slice());
        while let Some(block) = iterator.next() {
            update_state(&mut state, block);
        }

        // Finally write the output via big-endian encoding.
        let mut index = 0;
        while index < 8 {
            let dst = unsafe {
                let range = (4 * index)..(4 * (index + 1));
                subslice_mut_const(output, range)
            };
            let src = &state[index].to_be_bytes();
            dst.copy_from_slice(src);
            index += 1;
        }
    }

    /// Returns the final hash value.
    ///
    /// # Notes
    ///
    /// This function does not update the internal state, and thus
    /// the [`SHA2_256_Portable`] can still be used afterwards.
    #[inline(always)]
    #[must_use]
    pub const fn result_const(&self) -> [u8; 32] {
        let mut array = core::mem::MaybeUninit::<[u8; 32]>::uninit();
        let raw_ref = unsafe { &mut *array.as_mut_ptr() };
        self.write_result_const(raw_ref);
        unsafe { array.assume_init() }
    }
}

impl Default for SHA2_256_Portable {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl HashFunction for SHA2_256_Portable {
    type Output = [u8; 32];

    #[inline(always)]
    fn update(&mut self, data: impl AsRef<[u8]>) {
        self.update_const(data.as_ref());
    }

    #[inline(always)]
    fn write_result(&self, output: &mut Self::Output) {
        self.write_result_const(output);
    }
}
