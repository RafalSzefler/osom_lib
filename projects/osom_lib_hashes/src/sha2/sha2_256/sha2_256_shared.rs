use osom_lib_arrays::{
    const_helpers::{fill_const, subslice_mut_const},
    fixed_array::ConstFixedArray,
};
use osom_lib_reprc::macros::reprc;

pub const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5, 0xd807aa98,
    0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786,
    0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8,
    0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13,
    0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819,
    0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a,
    0x5b9cca4f, 0x682e6ff3, 0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
    0xc67178f2,
];

pub const INITIAL_STATE: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];

#[reprc]
#[repr(u8)]
#[must_use]
pub enum FinalBlockKind {
    Single([[u8; 64]; 1]),
    Double([[u8; 64]; 2]),
}

impl FinalBlockKind {
    #[inline(always)]
    #[must_use]
    pub const fn as_slice(&self) -> &[u8] {
        match self {
            FinalBlockKind::Single(block) => unsafe {
                core::slice::from_raw_parts(block.as_ptr().cast(), 64 * block.len())
            },
            FinalBlockKind::Double(block) => unsafe {
                core::slice::from_raw_parts(block.as_ptr().cast(), 64 * block.len())
            },
        }
    }
}

/// Calculates the final blocks for the SHA2-256 algorithm.
pub const fn calculate_final_blocks(total_length: u64, current_block: &ConstFixedArray<64, u8>) -> FinalBlockKind {
    const SUFFIX_OFFSET: usize = 64 - 8;

    let current_block_len = current_block.length().as_usize();

    // The multiplication by 8 is expected to be safe.
    let length_bits = unsafe { total_length.unchecked_mul(8) }.to_be_bytes();

    // First we add load the current block onto the stack. Note that even though this is
    // array of 64 bytes, it is actually `current_block_len` that tells how long it is.
    let mut first_block = unsafe { *current_block.as_raw_slice_const() };

    if current_block_len < SUFFIX_OFFSET {
        // We set the next bit (not byte) to `1`.
        first_block[current_block_len] = 0x80;

        // We fill the rest with zeros.
        unsafe {
            fill_const(subslice_mut_const(&mut first_block, current_block_len + 1..64), 0);
        }

        // If the current block has enough space for the length (which is 8 bytes),
        // then we write the length at the end.
        unsafe {
            subslice_mut_const(&mut first_block, SUFFIX_OFFSET..64).copy_from_slice(&length_bits);
        }
        FinalBlockKind::Single([first_block])
    } else {
        // Otherwise we need a new block.
        let mut second_block = [0u8; 64];

        if current_block_len < 64 {
            first_block[current_block_len] = 0x80;
            unsafe {
                fill_const(subslice_mut_const(&mut first_block, current_block_len + 1..64), 0);
            }
        } else {
            second_block[0] = 0x80;
        }

        unsafe {
            subslice_mut_const(&mut second_block, SUFFIX_OFFSET..64).copy_from_slice(&length_bits);
        }
        FinalBlockKind::Double([first_block, second_block])
    }
}
