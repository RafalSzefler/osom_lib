#![allow(non_camel_case_types, dead_code)]

use paste::paste;

macro_rules! align_array {
    ( $a: literal ) => {
        paste! {
            #[doc = "A wrapper around `[u8; N]` that enforces specific alignment: " $a "."]
            #[repr(C, align($a))]
            pub struct [<AlignArray_ $a>]<const N: usize> {
                inner: [u8; N]
            }

            impl<const N: usize> [< AlignArray_ $a>]<N> {
                #[inline(always)]
                pub fn from_slice(slice: &[u8]) -> Self {
                    assert!(slice.len() >= N);
                    let inner = unsafe { *slice.as_ptr().cast::<[u8; N]>() };
                    Self { inner }
                }

                #[inline(always)]
                pub const fn new(array: [u8; N]) -> Self {
                    Self { inner: array }
                }

                #[inline(always)]
                pub fn as_slice(&self) -> &[u8; N] {
                    let slice = &self.inner;
                    debug_assert!(slice.as_ptr().align_offset($a) == 0);
                    slice
                }

                #[inline(always)]
                pub fn as_slice_mut(&mut self) -> &mut [u8; N] {
                    let slice = &mut self.inner;
                    debug_assert!(slice.as_ptr().align_offset($a) == 0);
                    slice
                }

                #[inline(always)]
                pub fn into_array(self) -> [u8; N] {
                    self.inner
                }
            }
        }
    };
}

align_array!(4);
