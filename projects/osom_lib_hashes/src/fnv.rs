//! Holds the implementation of several variants of Fowler–Noll–Vo hash functions
//! and their corresponding builders.

#![allow(non_camel_case_types)]

use core::hash::{BuildHasher, Hasher};

use osom_lib_reprc::macros::reprc;

use super::traits::HashFunction;

macro_rules! build_fnv {
    ( $variant: tt, $underlying_type: tt ) => {
        ::paste::paste! {
            #[doc = "The [FNV](https://en.wikipedia.org/wiki/Fowler%E2%80%93Noll%E2%80%93Vo_hash_function) algorithm in version " $variant " with " $underlying_type "bit size."]
            #[derive(PartialEq, Eq)]
            #[reprc]
            #[must_use]
            pub struct [< FNV $variant _ $underlying_type >] {
                state: [< u $underlying_type >],
            }

            impl Default for [< FNV $variant _ $underlying_type >] {
                #[inline(always)]
                fn default() -> Self {
                    Self::new()
                }
            }

            impl [< FNV $variant _ $underlying_type >] {
                #[doc = "Creates a new instance of [`FNV" $variant "_" $underlying_type "`] with the default initial state."]
                #[inline(always)]
                pub const fn new() -> Self {
                    Self {
                        state: Self::INITIAL,
                    }
                }

                #[doc = "Creates a new instance of [`FNV" $variant "_" $underlying_type "`] from seed."]
                pub const fn with_seed(seed: [< u $underlying_type >]) -> Self {
                    let seed = seed.to_le_bytes();
                    let mut result = Self::new();
                    result.result_const(&seed);
                    result
                }

                #[doc = "Returns calculated hash."]
                #[inline(always)]
                pub const fn update_const(&self) -> [< u $underlying_type >] {
                    self.state
                }

                #[inline(always)]
                const fn clone_const(&self) -> Self {
                    Self {
                        state: self.state,
                    }
                }
            }

            impl Hasher for [< FNV $variant _ $underlying_type >] {
                #[allow(clippy::cast_possible_truncation)]
                #[inline(always)]
                fn finish(&self) -> u64 {
                    self.update_const() as u64
                }

                #[inline(always)]
                fn write(&mut self, bytes: &[u8]) {
                    self.update(bytes);
                }
            }

            impl HashFunction for [< FNV $variant _ $underlying_type >] {
                type Output = [u8; size_of::<[< u $underlying_type >]>()];

                #[inline(always)]
                fn update(&mut self, data: impl AsRef<[u8]>) {
                    self.result_const(data.as_ref())
                }

                #[inline(always)]
                fn write_result(&self, output: &mut Self::Output ) {
                    *output = self.update_const().to_le_bytes();
                }
            }

            #[doc = "A standard builder that produces [`FNV" $variant "_" $underlying_type "`] hash with fixed seed."]
            #[reprc]
            #[derive(PartialEq, Eq)]
            #[must_use]
            pub struct [< FNV $variant _ $underlying_type HasherBuilder >] {
                inner: [< FNV $variant _ $underlying_type >],
            }

            impl [< FNV $variant _ $underlying_type HasherBuilder >] {
                #[doc = "Creates new [`FNV" $variant "_" $underlying_type "HasherBuilder`]."]
                #[inline(always)]
                pub const fn new() -> Self {
                    Self {
                        inner: [< FNV $variant _ $underlying_type >]::new(),
                    }
                }

                #[doc = "Creates new [`FNV" $variant "_" $underlying_type "HasherBuilder`] with a fixed seed,"]
                #[doc = "which will be used to initialize the hash function."]
                #[inline(always)]
                pub const fn with_seed(seed: [< u $underlying_type >]) -> Self {
                    Self {
                        inner: [< FNV $variant _ $underlying_type >]::with_seed(seed),
                    }
                }


                #[doc = "Creates new [`FNV" $variant "_" $underlying_type "`] instance."]
                #[inline(always)]
                pub const fn create_hasher(&self) -> [< FNV $variant _ $underlying_type >] {
                    self.inner.clone_const()
                }
            }

            impl Default for [< FNV $variant _ $underlying_type HasherBuilder >] {
                #[inline(always)]
                fn default() -> Self {
                    Self::new()
                }
            }

            impl Clone for [< FNV $variant _ $underlying_type HasherBuilder >] {
                #[inline(always)]
                fn clone(&self) -> Self {
                    Self {
                        inner: self.inner.clone_const(),
                    }
                }
            }

            impl BuildHasher for [< FNV $variant _ $underlying_type HasherBuilder >] {
                type Hasher = [< FNV $variant _ $underlying_type >];

                #[inline(always)]
                fn build_hasher(&self) -> Self::Hasher {
                    self.create_hasher()
                }
            }
        }
    };
}

build_fnv!(1a, 64);
build_fnv!(1, 64);
build_fnv!(1a, 128);
build_fnv!(1, 128);

impl FNV1a_64 {
    const INITIAL: u64 = 0xcbf29ce484222325;
    const MULTIPLIER: u64 = 0x00000100000001b3;

    /// Updates internal [`FNV1a_64`] state with a given bytes block.
    pub const fn result_const(&mut self, data: &[u8]) {
        let mut current = self.state;
        let mut index = 0;
        let len = data.len();
        while index < len {
            let byte = data[index];
            current = current ^ (byte as u64);
            current = current.wrapping_mul(Self::MULTIPLIER);
            index += 1;
        }
        self.state = current;
    }
}

impl FNV1_64 {
    const INITIAL: u64 = 0xcbf29ce484222325;
    const MULTIPLIER: u64 = 0x00000100000001b3;

    /// Updates internal [`FNV1_64`] state with a given bytes block.
    pub const fn result_const(&mut self, data: &[u8]) {
        let mut current = self.state;
        let mut index = 0;
        let len = data.len();
        while index < len {
            let byte = data[index];
            current = current.wrapping_mul(Self::MULTIPLIER);
            current = current ^ (byte as u64);
            index += 1;
        }
        self.state = current;
    }
}

impl FNV1a_128 {
    const INITIAL: u128 = 0x6c62272e07bb014262b821756295c58d;
    const MULTIPLIER: u128 = 0x0000000001000000000000000000013b;

    /// Updates internal [`FNV1a_128`] state with a given bytes block.
    pub const fn result_const(&mut self, data: &[u8]) {
        let mut current = self.state;
        let mut index = 0;
        let len = data.len();
        while index < len {
            let byte = data[index];
            current = current ^ (byte as u128);
            current = current.wrapping_mul(Self::MULTIPLIER);
            index += 1;
        }
        self.state = current;
    }
}

impl FNV1_128 {
    const INITIAL: u128 = 0x6c62272e07bb014262b821756295c58d;
    const MULTIPLIER: u128 = 0x0000000001000000000000000000013b;

    /// Updates internal [`FNV1_128`] state with a given bytes block.
    pub const fn result_const(&mut self, data: &[u8]) {
        let mut current = self.state;
        let mut index = 0;
        let len = data.len();
        while index < len {
            let byte = data[index];
            current = current.wrapping_mul(Self::MULTIPLIER);
            current = current ^ (byte as u128);
            index += 1;
        }
        self.state = current;
    }
}
