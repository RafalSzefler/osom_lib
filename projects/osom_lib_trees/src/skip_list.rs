#![allow(dead_code)]
//! Holds the implementation of [`SkipList`] data structure.

use core::ptr::null_mut;

#[cfg(feature = "std_alloc")]
use osom_lib_alloc::StdAllocator;

use osom_lib_alloc::Allocator;
use osom_lib_rand::{
    pseudo_random_number_generators::LinearCongruentialGenerator, traits::PseudoRandomNumberGenerator,
};

struct SkipListNode<TKey, TValue, const MAX_LEVEL: usize> {
    key: TKey,
    value: TValue,
    nexts: [*mut SkipListNode<TKey, TValue, MAX_LEVEL>; MAX_LEVEL],
}

/// The classical [`SkipList`] data structure. And yes, we know it
/// has *List* in name. Yet, it is closer to tree than a list.
#[must_use]
pub struct SkipList<
    TKey,
    TValue,
    #[cfg(feature = "std_alloc")] TAllocator = StdAllocator,
    #[cfg(not(feature = "std_alloc"))] TAllocator,
    TPRNG = LinearCongruentialGenerator<u64>,
    const MAX_LEVEL: usize = 32,
> where
    TKey: PartialEq + Eq + PartialOrd + Ord,
    TAllocator: Allocator,
    TPRNG: PseudoRandomNumberGenerator<TNumber = u64>,
{
    head: *mut SkipListNode<TKey, TValue, MAX_LEVEL>,
    prng: TPRNG,
    allocator: TAllocator,
}

impl<TKey, TValue, TAllocator, TPRNG, const MAX_LEVEL: usize> SkipList<TKey, TValue, TAllocator, TPRNG, MAX_LEVEL>
where
    TKey: PartialEq + Eq + PartialOrd + Ord,
    TAllocator: Allocator,
    TPRNG: PseudoRandomNumberGenerator<TNumber = u64>,
{
    /// Constructs a new empty [`SkipList`] with the given [`PseudoRandomNumberGenerator`]
    /// and [`Allocator`].
    #[inline(always)]
    pub const fn with_prng_and_allocator(prng: TPRNG, allocator: TAllocator) -> Self {
        Self {
            head: null_mut(),
            prng,
            allocator,
        }
    }
}

impl<TKey, TValue, TAllocator, TPRNG, const MAX_LEVEL: usize> SkipList<TKey, TValue, TAllocator, TPRNG, MAX_LEVEL>
where
    TKey: PartialEq + Eq + PartialOrd + Ord,
    TAllocator: Allocator,
    TPRNG: PseudoRandomNumberGenerator<TNumber = u64> + Default,
{
    /// Constructs a new empty [`SkipList`] with default [`PseudoRandomNumberGenerator`]
    /// and default [`Allocator`].
    #[inline(always)]
    pub fn new() -> Self {
        Self::with_prng_and_allocator(TPRNG::default(), TAllocator::default())
    }
}

impl<TKey, TValue, TAllocator, TPRNG, const MAX_LEVEL: usize> Default
    for SkipList<TKey, TValue, TAllocator, TPRNG, MAX_LEVEL>
where
    TKey: PartialEq + Eq + PartialOrd + Ord,
    TAllocator: Allocator,
    TPRNG: PseudoRandomNumberGenerator<TNumber = u64> + Default,
{
    fn default() -> Self {
        Self::new()
    }
}
