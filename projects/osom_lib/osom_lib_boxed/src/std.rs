//! This module defines the std aliases.
use osom_lib_alloc::std_allocator::StdAllocator;

use super::cbox::CBox;

/// The alias for [`CBox`] with [`StdAllocator`].
pub type StdCBox<T> = CBox<T, StdAllocator>;
