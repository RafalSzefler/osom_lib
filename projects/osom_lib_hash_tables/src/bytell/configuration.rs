//! Contains the configuration for the bytell hash table.

use core::hash::BuildHasher;

use osom_lib_alloc::traits::Allocator;

use crate::helpers::MaxLoadFactor;

use super::hash_to_index::HashToIndex;

/// The actual bytell hash table configuration trait.
pub trait BytellConfig: Default + Clone + Sized {
    type ConcreteHashToIndex: HashToIndex;
    type ConcreteBuildHasher: BuildHasher;
    type ConcreteAllocator: Allocator;

    fn build_hasher(&self) -> &Self::ConcreteBuildHasher;
    fn hash_to_index(&self) -> &Self::ConcreteHashToIndex;
    fn hash_to_index_mut(&mut self) -> &mut Self::ConcreteHashToIndex;
    fn allocator(&self) -> &Self::ConcreteAllocator;
    fn load_factor(&self) -> MaxLoadFactor;
}
