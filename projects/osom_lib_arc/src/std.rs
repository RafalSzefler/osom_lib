//! This module defines the std aliases.
use osom_lib_alloc::std_allocator::StdAllocator;

use crate::caligned_arc_array::{CAlignedArcArray, CAlignedArcArrayBuilder, CAlignedWeakArray};
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

/// The alias for [`CAlignedArcArrayBuilder`] with [`StdAllocator`].
pub type StdCAlignedArcArrayBuilder<TAlign, T> = CAlignedArcArrayBuilder<TAlign, T, StdAllocator>;

/// The alias for [`CAlignedArcArray`] with [`StdAllocator`].
pub type StdCAlignedArcArray<TAlign, T> = CAlignedArcArray<TAlign, T, StdAllocator>;

/// The alias for [`CAlignedWeakArray`] with [`StdAllocator`].
pub type StdCAlignedWeakArray<TAlign, T> = CAlignedWeakArray<TAlign, T, StdAllocator>;
