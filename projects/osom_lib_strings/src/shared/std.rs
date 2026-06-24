//! Holds aliases usable with std feature.
use osom_lib_alloc::std_allocator::StdAllocator;

use super::{SharedString, SharedStringBuilder};

/// The alias for [`StdSharedString`] with [`StdAllocator`].
pub type StdSharedString = SharedString<StdAllocator>;

/// The alias for [`StdSharedStringBuilder`] with [`StdAllocator`].
pub type StdSharedStringBuilder = SharedStringBuilder<StdAllocator>;
