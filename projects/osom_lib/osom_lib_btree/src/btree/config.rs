use osom_lib_alloc::traits::Allocator;
use osom_lib_reprc::traits::ReprC;

/// The configuration trait for the [`BTree`][crate::btree::BTree].
#[must_use]
pub trait BTreeConfig: ReprC + Sized {
    /// The concrete allocator type.
    type ConcreteAllocator: Allocator;

    /// The number of children that a node can have. This has
    /// to be an even number.
    const CHILDREN_COUNT: usize;

    /// Returns the allocator for the [`BTree`][crate::btree::BTree].
    fn allocator_mut(&mut self) -> &mut Self::ConcreteAllocator;
}
