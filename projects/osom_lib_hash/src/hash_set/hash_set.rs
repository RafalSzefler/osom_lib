#[cfg(feature = "std_alloc")]
use osom_lib_alloc::StdAllocator;

use osom_lib_alloc::Allocator;
use osom_lib_arrays::InlineDynamicArray;
use osom_lib_primitives::Length;

use core::hash::{BuildHasher, Hash, Hasher};

use crate::{hash_set::Equivalent, hashers::Fnv1aHasherBuilder};

use super::errors::HashSetError;
use super::operation_results::{TryInsertResult, TryRemoveResult};

use super::quadratic_index_sequence::QuadraticIndexSequence;

use super::hash_set_bucket::Bucket;

#[must_use]
pub struct HashSet<
    const INLINE_SIZE: usize,
    T,
    TBuildHasher = Fnv1aHasherBuilder,
    #[cfg(feature = "std_alloc")] TAllocator = StdAllocator,
    #[cfg(not(feature = "std_alloc"))] TAllocator,
> where
    T: PartialEq + Eq + Hash,
    TBuildHasher: BuildHasher,
    TAllocator: Allocator,
{
    data: InlineDynamicArray<INLINE_SIZE, Bucket<T>, TAllocator>,
    deleted_count: i32,
    occupied_count: i32,
    hash_builder: TBuildHasher,
}

impl<const INLINE_SIZE: usize, T, TBuildHasher, TAllocator> HashSet<INLINE_SIZE, T, TBuildHasher, TAllocator>
where
    T: PartialEq + Eq + Hash,
    TBuildHasher: BuildHasher,
    TAllocator: Allocator,
{
    pub fn with_hash_builder(hash_builder: TBuildHasher) -> Self {
        Self::with_hash_builder_and_allocator(hash_builder, TAllocator::default())
    }

    pub fn with_hash_builder_and_allocator(hash_builder: TBuildHasher, allocator: TAllocator) -> Self {
        const {
            assert!(INLINE_SIZE >= 4, "Inline size must be at least 4");
        }

        let mut data = InlineDynamicArray::with_allocator(allocator);
        let _ = data.try_fill(|| Bucket::Empty).unwrap();

        Self {
            data: data,
            deleted_count: 0,
            occupied_count: 0,
            hash_builder,
        }
    }

    #[inline(always)]
    pub const fn capacity(&self) -> Length {
        self.data.capacity()
    }

    #[inline(always)]
    pub const fn len(&self) -> Length {
        unsafe { Length::new_unchecked(self.occupied_count as i32) }
    }

    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        self.occupied_count == 0
    }

    #[cfg(test)]
    pub(self) const fn data(&self) -> &InlineDynamicArray<INLINE_SIZE, Bucket<T>, TAllocator> {
        &self.data
    }
}

impl<const INLINE_SIZE: usize, T, TBuildHasher, TAllocator> HashSet<INLINE_SIZE, T, TBuildHasher, TAllocator>
where
    T: PartialEq + Eq + Hash,
    TBuildHasher: BuildHasher + Default,
    TAllocator: Allocator,
{
    #[inline(always)]
    pub fn new() -> Self {
        Self::with_hash_builder_and_allocator(TBuildHasher::default(), TAllocator::default())
    }

    #[inline(always)]
    pub fn with_allocator(allocator: TAllocator) -> Self {
        Self::with_hash_builder_and_allocator(TBuildHasher::default(), allocator)
    }
}

impl<const INLINE_SIZE: usize, T, TBuildHasher, TAllocator> Default
    for HashSet<INLINE_SIZE, T, TBuildHasher, TAllocator>
where
    T: PartialEq + Eq + Hash,
    TBuildHasher: BuildHasher + Default,
    TAllocator: Allocator,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(all(feature = "std_alloc", test))]
mod tests {
    use super::*;

    #[test]
    fn test_empty_hash_set() {
        let hash_set = HashSet::<4, i32>::new();
        let data = hash_set.data().as_slice();
        assert_eq!(data, &[Bucket::Empty, Bucket::Empty, Bucket::Empty, Bucket::Empty]);
    }
}
