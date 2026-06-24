#![allow(clippy::wildcard_imports, clippy::cast_possible_wrap, clippy::needless_return)]

use osom_lib_arrays::fixed_array::ConstBufferer;
use osom_lib_reprc::macros::reprc;

use crate::sha2::sha2_256::portable::SHA2_256_Portable;
use crate::traits::HashFunction;

use super::sha2_256_template::{SHA2_256_Template, SHA2_256_Updater};

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use crate::sha2::sha2_256::sha2_256_shared::K;

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe fn schedule(v0: __m128i, v1: __m128i, v2: __m128i, v3: __m128i) -> __m128i {
    unsafe {
        let t1 = _mm_sha256msg1_epu32(v0, v1);
        let t2 = _mm_alignr_epi8(v3, v2, 4);
        let t3 = _mm_add_epi32(t1, t2);
        _mm_sha256msg2_epu32(t3, v3)
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
macro_rules! rounds4 {
    ($abef:ident, $cdgh:ident, $rest:expr, $i:expr) => {{
        let idx: usize = 4 * ($i);
        let kv = _mm_set_epi32(K[idx + 3] as i32, K[idx + 2] as i32, K[idx + 1] as i32, K[idx] as i32);
        let t1 = _mm_add_epi32($rest, kv);
        $cdgh = _mm_sha256rnds2_epu32($cdgh, $abef, t1);
        let t2 = _mm_shuffle_epi32(t1, 0x0E);
        $abef = _mm_sha256rnds2_epu32($abef, $cdgh, t2);
    }};
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
macro_rules! schedule_rounds4 {
    (
        $abef:ident, $cdgh:ident,
        $w0:expr, $w1:expr, $w2:expr, $w3:expr, $w4:expr,
        $i: expr
    ) => {{
        $w4 = schedule($w0, $w1, $w2, $w3);
        rounds4!($abef, $cdgh, $w4, $i);
    }};
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "sha,sse2,sse3,ssse3,sse4.1")]
unsafe fn sha2_256_update_state_x86(state: &mut [u32; 8], bufferer: &mut ConstBufferer<'_, 64, u8>) {
    unsafe {
        let mask = _mm_set_epi64x(0x0C0D_0E0F_0809_0A0Bu64 as i64, 0x0405_0607_0001_0203u64 as i64);

        let state_ptr: *const __m128i = state.as_ptr().cast();
        let dcba = _mm_loadu_si128(state_ptr.add(0));
        let hgfe = _mm_loadu_si128(state_ptr.add(1));

        let cdab = _mm_shuffle_epi32(dcba, 0xB1);
        let efgh = _mm_shuffle_epi32(hgfe, 0x1B);
        let mut abef = _mm_alignr_epi8(cdab, efgh, 8);
        let mut cdgh = _mm_blend_epi16(efgh, cdab, 0xF0);

        while let Some(block) = bufferer.next() {
            let abef_save = abef;
            let cdgh_save = cdgh;

            let block_ptr: *const __m128i = block.as_ptr().cast();
            let mut w0 = _mm_shuffle_epi8(_mm_loadu_si128(block_ptr.add(0)), mask);
            let mut w1 = _mm_shuffle_epi8(_mm_loadu_si128(block_ptr.add(1)), mask);
            let mut w2 = _mm_shuffle_epi8(_mm_loadu_si128(block_ptr.add(2)), mask);
            let mut w3 = _mm_shuffle_epi8(_mm_loadu_si128(block_ptr.add(3)), mask);
            let mut w4;

            rounds4!(abef, cdgh, w0, 0);
            rounds4!(abef, cdgh, w1, 1);
            rounds4!(abef, cdgh, w2, 2);
            rounds4!(abef, cdgh, w3, 3);
            schedule_rounds4!(abef, cdgh, w0, w1, w2, w3, w4, 4);
            schedule_rounds4!(abef, cdgh, w1, w2, w3, w4, w0, 5);
            schedule_rounds4!(abef, cdgh, w2, w3, w4, w0, w1, 6);
            schedule_rounds4!(abef, cdgh, w3, w4, w0, w1, w2, 7);
            schedule_rounds4!(abef, cdgh, w4, w0, w1, w2, w3, 8);
            schedule_rounds4!(abef, cdgh, w0, w1, w2, w3, w4, 9);
            schedule_rounds4!(abef, cdgh, w1, w2, w3, w4, w0, 10);
            schedule_rounds4!(abef, cdgh, w2, w3, w4, w0, w1, 11);
            schedule_rounds4!(abef, cdgh, w3, w4, w0, w1, w2, 12);
            schedule_rounds4!(abef, cdgh, w4, w0, w1, w2, w3, 13);
            schedule_rounds4!(abef, cdgh, w0, w1, w2, w3, w4, 14);
            schedule_rounds4!(abef, cdgh, w1, w2, w3, w4, w0, 15);

            abef = _mm_add_epi32(abef, abef_save);
            cdgh = _mm_add_epi32(cdgh, cdgh_save);
        }

        let feba = _mm_shuffle_epi32(abef, 0x1B);
        let dchg = _mm_shuffle_epi32(cdgh, 0xB1);
        let dcba = _mm_blend_epi16(feba, dchg, 0xF0);
        let hgef = _mm_alignr_epi8(dchg, feba, 8);

        let state_ptr_mut: *mut __m128i = state.as_mut_ptr().cast();
        _mm_storeu_si128(state_ptr_mut.add(0), dcba);
        _mm_storeu_si128(state_ptr_mut.add(1), hgef);
    }
}

struct SHA2_256_x86_Updater;

impl SHA2_256_Updater for SHA2_256_x86_Updater {
    #[inline(always)]
    fn update_state(state: &mut [u32; 8], bufferer: &mut ConstBufferer<'_, 64, u8>) {
        cfg_select! {
            any(target_arch = "x86", target_arch = "x86_64") => {
                unsafe { sha2_256_update_state_x86(state, bufferer) };
            },
            _ => {
                let _ = state;
                let _ = bufferer;
                panic!("SHA2_256_x86 requires x86 or x86_64 target.");
            },
        }
    }
}

/// An `x86` and `x86_64` optimized implementation of the `SHA2_256` algorithm.
///
/// This implementation is only available on `x86` and `x86_64` targets.
/// The code will panic if the target is invalid.
///
/// # Safety
///
/// This implementation does not verify that the target supports the required instructions.
/// In particular `sha,sse4.1` features have to be supported. Otherwise the code will
/// likely crash at runtime.
#[reprc]
#[repr(transparent)]
#[must_use]
pub struct SHA2_256_x86 {
    inner: SHA2_256_Template<SHA2_256_x86_Updater>,
}

impl SHA2_256_x86 {
    /// Creates a new [`SHA2_256_x86`] instance.
    ///
    /// # Panics
    ///
    /// If `target_arch` is neither `x86` nor `x86_64`.
    #[inline(always)]
    pub const fn new() -> Self {
        cfg_select! {
            any(target_arch = "x86", target_arch = "x86_64") => {
                return Self {
                    inner: SHA2_256_Template::new(),
                };
            },
            _ => {
                panic!("SHA2_256_x86 requires x86 or x86_64 target.");
            },
        }
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

impl Default for SHA2_256_x86 {
    fn default() -> Self {
        Self::new()
    }
}

impl HashFunction for SHA2_256_x86 {
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

impl From<SHA2_256_Portable> for SHA2_256_x86 {
    #[inline(always)]
    fn from(portable: SHA2_256_Portable) -> Self {
        Self { inner: portable.into() }
    }
}

impl From<SHA2_256_x86> for SHA2_256_Portable {
    #[inline(always)]
    fn from(x86: SHA2_256_x86) -> Self {
        x86.inner.into()
    }
}
