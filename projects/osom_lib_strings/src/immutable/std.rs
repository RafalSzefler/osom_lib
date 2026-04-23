use osom_lib_alloc::std_allocator::StdAllocator;

use crate::immutable::{ImmutableString, ImmutableStringBuilder};

/// The alias for [`StdImmutableString`] with [`StdAllocator`].
pub type StdImmutableString = ImmutableString<StdAllocator>;

/// The alias for [`StdImmutableStringBuilder`] with [`StdAllocator`].
pub type StdImmutableStringBuilder = ImmutableStringBuilder<StdAllocator>;
