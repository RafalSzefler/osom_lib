//! Holds aliases usable with std feature.
use osom_lib_alloc::std_allocator::StdAllocator;

use crate::owned::{OwnedString, OwnedStringBuilder};
use crate::shared::{SharedString, SharedStringBuilder};

/// The alias for [`SharedString`] with [`StdAllocator`].
pub type StdSharedString = SharedString<StdAllocator>;

/// The alias for [`SharedStringBuilder`] with [`StdAllocator`].
pub type StdSharedStringBuilder = SharedStringBuilder<StdAllocator>;

/// The alias for [`OwnedString`] with [`StdAllocator`].
pub type StdOwnedString = OwnedString<StdAllocator>;

/// The alias for [`OwnedStringBuilder`] with [`StdAllocator`].
pub type StdOwnedStringBuilder = OwnedStringBuilder<StdAllocator>;
