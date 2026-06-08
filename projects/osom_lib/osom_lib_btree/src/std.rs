//! This module defines the std aliases.
use osom_lib_alloc::std_allocator::StdAllocator;

use crate::defaults::DefaultBTree;

/// An alias for [`DefaultBTree`] with [`StdAllocator`]. Requires `std` feature.
pub type StdBTree<TKey, TValue> = DefaultBTree<TKey, TValue, StdAllocator>;
