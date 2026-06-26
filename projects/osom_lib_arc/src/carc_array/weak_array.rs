//! Holds the definition of [`CWeak`].
use osom_lib_alloc::traits::Allocator;
use osom_lib_primitives::align::Align;
use osom_lib_reprc::traits::ReprC;
use osom_lib_try_clone::TryClone;

use crate::{
    caligned_arc_array::CAlignedWeakArray,
    carc_array::CArcArray,
    errors::{MaxReferencesExceededError, WeakUpgradeError},
};

/// A weak reference to the underlying [`CArcArray`].
///
/// This object cannot inspect the underlying value (unless `T` is `Copy`).
/// But it does track weak references, and each weak reference can build
/// a strong reference, assuming any other strong reference is alive.
#[repr(transparent)]
#[must_use]
#[derive(Debug)]
pub struct CWeakArray<T, TAllocator: Allocator> {
    internal: CAlignedWeakArray<Align<1>, T, TAllocator>,
}

unsafe impl<T: ReprC, TAllocator: Allocator> ReprC for CWeakArray<T, TAllocator> {
    const CHECK: () = const {
        osom_lib_reprc::hidden::is_reprc::<T>();
        osom_lib_reprc::hidden::is_reprc::<CAlignedWeakArray<Align<1>, T, TAllocator>>();
    };
}

impl<T, TAllocator: Allocator> CWeakArray<T, TAllocator> {
    /// Returns the number of strong references to the [`CWeakArray`].
    #[inline(always)]
    #[must_use]
    pub fn strong_count(&self) -> u32 {
        CAlignedWeakArray::strong_count(&self.internal)
    }

    /// Returns the number of weak references to the [`CWeakArray`].
    #[inline(always)]
    #[must_use]
    pub fn weak_count(&self) -> u32 {
        CAlignedWeakArray::weak_count(&self.internal)
    }

    /// Upgrades current weak reference to the strong [`CArcArray`].
    ///
    /// # Errors
    ///
    /// For details see [`WeakUpgradeError`].
    pub fn upgrade(&self) -> Result<CArcArray<T, TAllocator>, WeakUpgradeError> {
        let strong = CAlignedWeakArray::upgrade(&self.internal)?;
        Ok(CArcArray::from_internal(strong))
    }

    /// Returns a reference to the underlying slice.
    ///
    /// This function is only available if `T` implements `Copy`.
    /// That is because being `Copy` means it is not `Drop`. And
    /// so we don't need strong references to keep the data alive.
    #[inline]
    #[must_use]
    pub fn data(&self) -> &[T]
    where
        T: Copy,
    {
        self.internal.data()
    }

    /// Abandons current weak reference.
    ///
    /// If the internal weak counter is positive it returns false.
    ///
    /// Otherwise it deallocates the underlying memory and returns true.
    /// In particular only single (the last) [`CWeakArray`] returns true
    /// by calling this.
    #[inline(always)]
    #[must_use]
    pub fn abandon(self) -> bool {
        let result = unsafe { core::ptr::read(&raw const self.internal) }.abandon();
        core::mem::forget(self);
        result
    }

    #[inline]
    pub(super) fn from_internal(internal: CAlignedWeakArray<Align<1>, T, TAllocator>) -> Self {
        Self { internal }
    }
}

impl<T, TAllocator: Allocator> Clone for CWeakArray<T, TAllocator> {
    fn clone(&self) -> Self {
        Self {
            internal: self.internal.clone(),
        }
    }
}

impl<T, TAllocator: Allocator> TryClone for CWeakArray<T, TAllocator> {
    type Error = MaxReferencesExceededError;

    fn try_clone(&self) -> Result<Self, Self::Error> {
        Ok(Self {
            internal: self.internal.try_clone()?,
        })
    }
}
