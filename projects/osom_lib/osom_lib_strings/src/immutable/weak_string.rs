use osom_lib_alloc::traits::Allocator;
use osom_lib_arc::carc_array::CWeakArray;
use osom_lib_reprc::macros::reprc;
use osom_lib_try_clone::TryClone;

use crate::immutable::{ImmutableString, MaxReferencesExceededError};

/// Represents possible errors when upgrading a [`WeakImmutableString`] to a [`ImmutableString`].
#[reprc]
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[must_use]
pub enum WeakUpgradeError {
    /// The maximum number of references is exceeded.
    MaxReferencesExceeded = 0,

    /// There are no strong references alive.
    NoStrongReferencesAlive = 1,
}

impl From<osom_lib_arc::errors::WeakUpgradeError> for WeakUpgradeError {
    fn from(err: osom_lib_arc::errors::WeakUpgradeError) -> Self {
        match err {
            osom_lib_arc::errors::WeakUpgradeError::MaxReferencesExceeded => Self::MaxReferencesExceeded,
            osom_lib_arc::errors::WeakUpgradeError::NoStrongReferencesAlive => Self::NoStrongReferencesAlive,
        }
    }
}

/// A weak reference to the underlying [`ImmutableString`].
///
/// This object cannot inspect the underlying string. But it does track
/// weak references, and each weak reference can build a strong reference,
/// assuming any other strong reference is alive.
///
/// # Examples
///
/// ```rust
/// # cfg_select! {
/// #    feature="std" => {
/// use osom_lib_strings::immutable::{ImmutableStringError, WeakUpgradeError};
/// use osom_lib_strings::immutable::std::StdImmutableString;
///
/// fn main() -> Result<(), ImmutableStringError> {
///     // The first instance counts as both strong and weak reference.
///     let strong1 = StdImmutableString::from_str_slice("foo")?;
///     assert_eq!(strong1.strong_count(), 1);
///     assert_eq!(strong1.weak_count(), 1);
///     assert_eq!(strong1.as_str(), "foo");
///
///     let weak = strong1.downgrade().unwrap();  // Cheap operation
///     assert_eq!(strong1.strong_count(), 1);
///     assert_eq!(strong1.weak_count(), 2);
///     assert_eq!(weak.strong_count(), 1);
///     assert_eq!(weak.weak_count(), 2);
///
///     let strong2 = weak.upgrade().unwrap();  // Cheap operation
///     assert_eq!(strong1.strong_count(), 2);
///     assert_eq!(strong1.weak_count(), 2);
///     assert_eq!(weak.strong_count(), 2);
///     assert_eq!(weak.weak_count(), 2);
///     assert_eq!(strong2.as_str(), strong1.as_str());
///
///     drop(strong2);
///     assert_eq!(strong1.strong_count(), 1);
///     assert_eq!(strong1.weak_count(), 2);
///     assert_eq!(weak.strong_count(), 1);
///     assert_eq!(weak.weak_count(), 2);
///
///     drop(strong1);
///     // Both counters are decreased, since there are no more strong references.
///     assert_eq!(weak.strong_count(), 0);
///     assert_eq!(weak.weak_count(), 1);
///
///     // No more strong reference, .upgrade() won't work.
///     assert!(matches!(weak.upgrade(), Err(WeakUpgradeError::NoStrongReferencesAlive)));
///
///     drop(weak);  // The actual deallocation happens here.
///     Ok(())
/// }
/// #   },
/// #   _ => { }
/// # }
/// ```
#[reprc]
#[repr(transparent)]
#[derive(Debug)]
#[must_use]
pub struct WeakImmutableString<TAllocator: Allocator> {
    internal: CWeakArray<u8, TAllocator>,
}

impl<TAllocator: Allocator> WeakImmutableString<TAllocator> {
    #[inline(always)]
    pub(crate) fn from_internal(internal: CWeakArray<u8, TAllocator>) -> Self {
        Self { internal }
    }

    /// Returns the number of strong references to the [`WeakImmutableString`].
    #[inline(always)]
    #[must_use]
    pub fn strong_count(&self) -> u32 {
        CWeakArray::strong_count(&self.internal)
    }

    /// Returns the number of weak references to the [`WeakImmutableString`].
    #[inline(always)]
    #[must_use]
    pub fn weak_count(&self) -> u32 {
        CWeakArray::weak_count(&self.internal)
    }

    /// Returns the underlying string.
    ///
    /// # Notes
    ///
    /// Even though this is a weak reference, the string is still valid,
    /// because a sequence of chars is not `Drop`.
    #[inline]
    #[must_use]
    pub fn as_str(&self) -> &str {
        unsafe {
            let slice = self.internal.data();
            core::str::from_utf8_unchecked(&slice[..slice.len() - 1])
        }
    }

    /// Returns the underlying string as C-string.
    ///
    /// # Notes
    ///
    /// Even though this is a weak reference, the string is still valid,
    /// because a sequence of chars is not `Drop`.
    #[inline]
    #[must_use]
    pub fn as_c_str(&self) -> &str {
        unsafe {
            let slice = self.internal.data();
            core::str::from_utf8_unchecked(slice)
        }
    }

    /// Upgrades current weak reference to the strong [`ImmutableString`].
    ///
    ///
    /// # Errors
    ///
    /// For details see [`WeakUpgradeError`].
    pub fn upgrade(&self) -> Result<ImmutableString<TAllocator>, WeakUpgradeError> {
        let strong = CWeakArray::upgrade(&self.internal)?;
        Ok(ImmutableString::from_internal(strong))
    }

    /// Abandons current weak reference.
    ///
    /// If the internal weak counter is positive it returns false.
    ///
    /// Otherwise it deallocates the underlying memory and returns true.
    /// In particular only single (the last) [`WeakImmutableString`] returns true
    /// by calling this.
    #[inline(always)]
    #[must_use]
    pub fn abandon(self) -> bool {
        CWeakArray::<u8, TAllocator>::abandon(self.internal)
    }
}

impl<TAllocator: Allocator> TryClone for WeakImmutableString<TAllocator> {
    type Error = MaxReferencesExceededError;

    fn try_clone(&self) -> Result<Self, Self::Error> {
        let internal = self.internal.try_clone()?;
        Ok(Self { internal })
    }
}

impl<TAllocator: Allocator> Clone for WeakImmutableString<TAllocator> {
    fn clone(&self) -> Self {
        self.try_clone()
            .expect("WeakImmutableString weak reference count is too high.")
    }
}
