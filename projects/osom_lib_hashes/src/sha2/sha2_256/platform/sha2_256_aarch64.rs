#![allow(clippy::wildcard_imports, clippy::cast_possible_wrap, clippy::needless_return)]

use osom_lib_arrays::fixed_array::ConstBufferer;
use osom_lib_reprc::macros::reprc;

use crate::sha2::sha2_256::portable::SHA2_256_Portable;
use crate::traits::HashFunction;

use super::sha2_256_template::{SHA2_256_Template, SHA2_256_Updater};

#[cfg(target_arch = "aarch64")]
use core::arch::aarch64::*;

#[cfg(target_arch = "aarch64")]
use crate::sha2::sha2_256::sha2_256_shared::K;

#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "sha2")]
unsafe fn sha2_256_update_state_aarch64(state: &mut [u32; 8], bufferer: &mut ConstBufferer<'_, 64, u8>) {
    unsafe {
        let mut abcd = vld1q_u32(state[0..4].as_ptr());
        let mut efgh = vld1q_u32(state[4..8].as_ptr());

        // Iterate through the message blocks.
        while let Some(block) = bufferer.next() {
            // Keep original state values.
            let abcd_orig = abcd;
            let efgh_orig = efgh;

            // Load the message block into vectors, assuming little endianness.
            let mut s0 = vreinterpretq_u32_u8(vrev32q_u8(vld1q_u8(block[0..16].as_ptr())));
            let mut s1 = vreinterpretq_u32_u8(vrev32q_u8(vld1q_u8(block[16..32].as_ptr())));
            let mut s2 = vreinterpretq_u32_u8(vrev32q_u8(vld1q_u8(block[32..48].as_ptr())));
            let mut s3 = vreinterpretq_u32_u8(vrev32q_u8(vld1q_u8(block[48..64].as_ptr())));

            // Rounds 0 to 3
            let mut tmp = vaddq_u32(s0, vld1q_u32(&K[0]));
            let mut abcd_prev = abcd;
            abcd = vsha256hq_u32(abcd_prev, efgh, tmp);
            efgh = vsha256h2q_u32(efgh, abcd_prev, tmp);

            // Rounds 4 to 7
            tmp = vaddq_u32(s1, vld1q_u32(&K[4]));
            abcd_prev = abcd;
            abcd = vsha256hq_u32(abcd_prev, efgh, tmp);
            efgh = vsha256h2q_u32(efgh, abcd_prev, tmp);

            // Rounds 8 to 11
            tmp = vaddq_u32(s2, vld1q_u32(&K[8]));
            abcd_prev = abcd;
            abcd = vsha256hq_u32(abcd_prev, efgh, tmp);
            efgh = vsha256h2q_u32(efgh, abcd_prev, tmp);

            // Rounds 12 to 15
            tmp = vaddq_u32(s3, vld1q_u32(&K[12]));
            abcd_prev = abcd;
            abcd = vsha256hq_u32(abcd_prev, efgh, tmp);
            efgh = vsha256h2q_u32(efgh, abcd_prev, tmp);

            for t in (16..64).step_by(16) {
                // Rounds t to t + 3
                s0 = vsha256su1q_u32(vsha256su0q_u32(s0, s1), s2, s3);
                tmp = vaddq_u32(s0, vld1q_u32(&K[t]));
                abcd_prev = abcd;
                abcd = vsha256hq_u32(abcd_prev, efgh, tmp);
                efgh = vsha256h2q_u32(efgh, abcd_prev, tmp);

                // Rounds t + 4 to t + 7
                s1 = vsha256su1q_u32(vsha256su0q_u32(s1, s2), s3, s0);
                tmp = vaddq_u32(s1, vld1q_u32(&K[t + 4]));
                abcd_prev = abcd;
                abcd = vsha256hq_u32(abcd_prev, efgh, tmp);
                efgh = vsha256h2q_u32(efgh, abcd_prev, tmp);

                // Rounds t + 8 to t + 11
                s2 = vsha256su1q_u32(vsha256su0q_u32(s2, s3), s0, s1);
                tmp = vaddq_u32(s2, vld1q_u32(&K[t + 8]));
                abcd_prev = abcd;
                abcd = vsha256hq_u32(abcd_prev, efgh, tmp);
                efgh = vsha256h2q_u32(efgh, abcd_prev, tmp);

                // Rounds t + 12 to t + 15
                s3 = vsha256su1q_u32(vsha256su0q_u32(s3, s0), s1, s2);
                tmp = vaddq_u32(s3, vld1q_u32(&K[t + 12]));
                abcd_prev = abcd;
                abcd = vsha256hq_u32(abcd_prev, efgh, tmp);
                efgh = vsha256h2q_u32(efgh, abcd_prev, tmp);
            }

            // Add the block-specific state to the original state.
            abcd = vaddq_u32(abcd, abcd_orig);
            efgh = vaddq_u32(efgh, efgh_orig);
        }

        // Store vectors into state.
        vst1q_u32(state[0..4].as_mut_ptr(), abcd);
        vst1q_u32(state[4..8].as_mut_ptr(), efgh);
    }
}

struct SHA2_256_aarch64_Updater;

impl SHA2_256_Updater for SHA2_256_aarch64_Updater {
    #[inline(always)]
    fn update_state(state: &mut [u32; 8], bufferer: &mut ConstBufferer<'_, 64, u8>) {
        osom_lib_cfg_ext::cfg_match!(
            (target_arch = "aarch64") => {
                unsafe { sha2_256_update_state_aarch64(state, bufferer) };
            },
            _ => {
                let _ = state;
                let _ = bufferer;
                panic!("SHA2_256_aarch64 requires aarch64 target.");
            },
        );
    }
}

/// An `aarch64` optimized implementation of the `SHA2_256` algorithm.
///
/// This implementation is only available on `aarch64` targets.
/// The code will panic if the target is invalid.
///
/// # Safety
///
/// This implementation does not verify that the target supports the required instructions.
/// In particular `sha2` feature has to be supported. Otherwise the code will
/// likely crash at runtime.
#[reprc]
#[repr(transparent)]
#[must_use]
pub struct SHA2_256_aarch64 {
    inner: SHA2_256_Template<SHA2_256_aarch64_Updater>,
}

impl SHA2_256_aarch64 {
    /// Creates a new [`SHA2_256_aarch64`] instance.
    ///
    /// # Panics
    ///
    /// If `target_arch` is not `aarch64`.
    #[inline(always)]
    pub const fn new() -> Self {
        osom_lib_cfg_ext::cfg_match!(
            (target_arch="aarch64") => {
                return Self {
                    inner: SHA2_256_Template::new(),
                };
            },
            _ => {
                panic!("SHA2_256_aarch64 requires aarch64 target.");
            },
        );
    }

    /// Writes a block of data to the underlying state.
    #[inline(always)]
    pub fn update(&mut self, data: impl AsRef<[u8]>) {
        self.inner.update(data);
    }

    /// Calculates the final hash value.
    #[inline(always)]
    pub fn result(&self, output: &mut [u8; 32]) {
        self.inner.result(output);
    }
}

impl Default for SHA2_256_aarch64 {
    fn default() -> Self {
        Self::new()
    }
}

impl HashFunction for SHA2_256_aarch64 {
    type Output = [u8; 32];

    #[inline(always)]
    fn update(&mut self, data: impl AsRef<[u8]>) {
        self.update(data);
    }

    #[inline(always)]
    fn write_result(&self, output: &mut Self::Output) {
        self.result(output);
    }
}

impl From<SHA2_256_Portable> for SHA2_256_aarch64 {
    #[inline(always)]
    fn from(portable: SHA2_256_Portable) -> Self {
        Self { inner: portable.into() }
    }
}

impl From<SHA2_256_aarch64> for SHA2_256_Portable {
    #[inline(always)]
    fn from(x86: SHA2_256_aarch64) -> Self {
        x86.inner.into()
    }
}
