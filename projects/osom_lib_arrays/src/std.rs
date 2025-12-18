//! Holds aliases for the standard arrays.

use osom_lib_alloc::std_allocator::StdAllocator;

use crate::dynamic_array::DynamicArray;

/// The alias for [`DynamicArray`] with [`StdAllocator`].
pub type StdDynamicArray<T> = DynamicArray<T, StdAllocator>;
