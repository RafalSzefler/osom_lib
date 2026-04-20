//! Contains the Abseil hash table configuration.

use core::hash::BuildHasher;
use core::hash::Hash;

use osom_lib_alloc::traits::Allocator;

use crate::helpers::MaxLoadFactor;

/// The actual Abseil hash table configuration trait.
pub trait AbseilConfig: Default + Clone + Sized {
    type ConcreteBuildHasher: BuildHasher;
    type ConcreteAllocator: Allocator;

    fn build_hasher(&self) -> &Self::ConcreteBuildHasher;
    fn allocator(&self) -> &Self::ConcreteAllocator;
    fn load_factor(&self) -> MaxLoadFactor;
    fn calculate_partial_hashes<T: Hash>(&self, value: T) -> (u64, u8) {
        let hash_value = self.build_hasher().hash_one(value);
        let h1 = hash_value >> 7;
        let h2 = (hash_value & 0x7f) as u8;
        (h1, h2)
    }
}
