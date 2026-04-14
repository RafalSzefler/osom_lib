//! Holds aliases for the standard arrays.

use osom_lib_alloc::std_allocator::StdAllocator;

use crate::{dynamic_array::DynamicArray, fixed_array::FixedArray, inline_array::InlineArray};

/// The alias for [`DynamicArray`] with [`StdAllocator`].
pub type StdDynamicArray<T> = DynamicArray<T, StdAllocator>;

/// The alias for [`InlineArray`] with [`StdAllocator`].
pub type StdInlineArray<const TCAPACITY: usize, T> = InlineArray<TCAPACITY, T, StdAllocator>;

/// The alias for [`FixedArray`] with [`StdAllocator`].
pub type StdFixedArray<T> = FixedArray<T, StdAllocator>;
