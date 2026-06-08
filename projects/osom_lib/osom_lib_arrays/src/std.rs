//! Holds aliases for the standard arrays.

use osom_lib_alloc::std_allocator::StdAllocator;

use crate::dynamic_array::{DynamicArray, InlineDynamicArray};
use crate::fixed_array::FixedArray;

/// The alias for [`DynamicArray`] with [`StdAllocator`].
pub type StdDynamicArray<T> = DynamicArray<T, StdAllocator>;

/// The alias for [`InlineDynamicArray`] with [`StdAllocator`].
pub type StdInlineDynamicArray<const TCAPACITY: usize, T> = InlineDynamicArray<TCAPACITY, T, StdAllocator>;

/// The alias for [`FixedArray`] with [`StdAllocator`].
pub type StdFixedArray<T> = FixedArray<T, StdAllocator>;
