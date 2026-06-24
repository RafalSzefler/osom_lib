//! This module defines the std aliases.
use osom_lib_alloc::std_allocator::StdAllocator;

use crate::carc::{CArc, CWeak};
use crate::carc_array::{CArcArray, CArcArrayBuilder, CWeakArray};

/// The alias for [`CArc`] with [`StdAllocator`].
pub type StdCArc<T> = CArc<T, StdAllocator>;

/// The alias for [`CWeak`] with [`StdAllocator`].
pub type StdCWeak<T> = CWeak<T, StdAllocator>;

/// The alias for [`CArcArrayBuilder`] with [`StdAllocator`].
pub type StdCArcArrayBuilder<T> = CArcArrayBuilder<T, StdAllocator>;

/// The alias for [`CArcArray`] with [`StdAllocator`].
pub type StdCArcArray<T> = CArcArray<T, StdAllocator>;

/// The alias for [`CWeakArray`] with [`StdAllocator`].
pub type StdCWeakArray<T> = CWeakArray<T, StdAllocator>;
