//! Holds [`ImmutableString`] struct and related tools.

use core::mem::ManuallyDrop;

use osom_lib_alloc::Allocator;
use osom_lib_arrays::{ImmutableArray, ImmutableArrayConstructionError, ImmutableWeakArray};
use osom_lib_primitives::Length;

#[cfg(feature = "std_alloc")]
use osom_lib_alloc::StdAllocator;

/// Represents an error that occurs when constructing new [`ImmutableString`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[must_use]
#[repr(u8)]
pub enum ImmutableStringConstructionError {
    /// The allocator failed to allocate memory.
    AllocationError,

    /// The passed array is too long, it exceeds [`MAX_LENGTH`][`ImmutableString::MAX_LENGTH`].
    StringTooLong,
}

impl From<ImmutableArrayConstructionError> for ImmutableStringConstructionError {
    fn from(error: ImmutableArrayConstructionError) -> Self {
        match error {
            ImmutableArrayConstructionError::AllocationError => ImmutableStringConstructionError::AllocationError,
            ImmutableArrayConstructionError::ArrayTooLong => ImmutableStringConstructionError::StringTooLong,
        }
    }
}

/// Represents an immutable string, which is stored behind ref counters.
/// This is a thin wrapper around [`ImmutableArray<u8>`], and is thread
/// safe as well.
///
/// Cloning and moving around are very cheap.
///
/// # Notes
///
/// In order to build [`ImmutableString`] incrementally, use
/// [`ImmutableArrayBuilder<u8>`][`osom_lib_arrays::ImmutableArrayBuilder<u8>`]
/// and convert the final [`ImmutableArray<u8>`] to [`ImmutableString`] either safely
/// (with UTF-8 validation) or unsafely (without validation).
#[derive(Clone)]
#[repr(transparent)]
#[must_use]
pub struct ImmutableString<
    #[cfg(feature = "std_alloc")] TAllocator = StdAllocator,
    #[cfg(not(feature = "std_alloc"))] TAllocator,
> where
    TAllocator: Allocator,
{
    internal: ManuallyDrop<ImmutableArray<u8, TAllocator>>,
}

unsafe impl<TAllocator: Allocator> Send for ImmutableString<TAllocator> {}
unsafe impl<TAllocator: Allocator> Sync for ImmutableString<TAllocator> {}

/// Represents a weak reference to an [`ImmutableString`].
/// Analogously to how [`ImmutableWeakArray`] is a weak
/// reference to [`ImmutableArray`].
#[derive(Clone)]
#[repr(transparent)]
pub struct ImmutableWeakString<
    #[cfg(feature = "std_alloc")] TAllocator = StdAllocator,
    #[cfg(not(feature = "std_alloc"))] TAllocator,
> where
    TAllocator: Allocator,
{
    internal: ManuallyDrop<ImmutableWeakArray<u8, TAllocator>>,
}

impl<TAllocator: Allocator> ImmutableWeakString<TAllocator> {
    fn from_internal(internal: ImmutableWeakArray<u8, TAllocator>) -> Self {
        Self {
            internal: ManuallyDrop::new(internal),
        }
    }

    /// Upgrades the [`ImmutableWeakString`] to an [`ImmutableString`].
    ///
    /// Returns `None` if the [`ImmutableWeakString`] is no longer valid, i.e.
    /// the underlying memory has been deallocated. Otherwise returns
    /// a new strong
    #[inline(always)]
    pub fn upgrade(&self) -> Option<ImmutableString<TAllocator>> {
        let internal = self.internal.upgrade()?;
        Some(ImmutableString::from_internal(internal))
    }

    /// Returns the number of strong references to the [`ImmutableWeakString`].
    #[inline(always)]
    #[must_use]
    pub fn strong_count(instance: &Self) -> usize {
        instance.internal.strong_count()
    }

    /// Returns the number of weak references to the [`ImmutableWeakString`].
    #[inline(always)]
    #[must_use]
    pub fn weak_count(instance: &Self) -> usize {
        instance.internal.weak_count()
    }

    /// Releases the [`ImmutableWeakString`]. If this was the last weak reference,
    /// it will deallocated the underlying memory and return `true`.
    /// Otherwise it will just return `false`.
    #[inline(always)]
    pub fn release(mut self) -> bool {
        let internal = unsafe { ManuallyDrop::take(&mut self.internal) };
        let val = internal.release();
        core::mem::forget(self);
        val
    }
}

impl<TAllocator: Allocator> Drop for ImmutableWeakString<TAllocator> {
    fn drop(&mut self) {
        unsafe { ManuallyDrop::drop(&mut self.internal) };
    }
}

impl<TAllocator: Allocator> ImmutableString<TAllocator> {
    fn from_internal(internal: ImmutableArray<u8, TAllocator>) -> Self {
        Self {
            internal: ManuallyDrop::new(internal),
        }
    }

    pub const MAX_LENGTH: usize = ImmutableArray::<u8, TAllocator>::MAX_LENGTH;

    /// Constructs a new [`ImmutableString`] from a string with default allocator.
    /// It copies the string into the new [`ImmutableString`].
    ///
    /// # Errors
    ///
    /// For details see [`ImmutableStringConstructionError`].
    pub fn new(text: &str) -> Result<Self, ImmutableStringConstructionError> {
        Self::with_allocator(text, TAllocator::default())
    }

    /// Constructs a new [`ImmutableString`] from a string and an allocator.
    /// It copies the string into the new [`ImmutableString`].
    ///
    /// # Errors
    ///
    /// For details see [`ImmutableStringConstructionError`].
    pub fn with_allocator(text: &str, allocator: TAllocator) -> Result<Self, ImmutableStringConstructionError> {
        let array = ImmutableArray::from_slice_with_allocator(text.as_bytes(), allocator)?;
        Ok(Self {
            internal: ManuallyDrop::new(array),
        })
    }

    #[inline(always)]
    #[must_use]
    pub fn as_str(&self) -> &str {
        let slice = self.internal.as_slice();
        unsafe { core::str::from_utf8_unchecked(slice) }
    }

    /// Downgrades the [`ImmutableString`] to a [`ImmutableWeakString`] and increments the internal weak counter.
    #[inline(always)]
    pub fn downgrade(instance: &Self) -> ImmutableWeakString<TAllocator> {
        ImmutableWeakString::from_internal(ImmutableArray::downgrade(&instance.internal))
    }

    /// Returns the number of strong references to the [`ImmutableString`].
    #[inline(always)]
    #[must_use]
    pub fn strong_count(instance: &Self) -> usize {
        ImmutableArray::strong_count(&instance.internal)
    }

    /// Returns the number of weak references to the [`ImmutableString`].
    #[inline(always)]
    #[must_use]
    pub fn weak_count(instance: &Self) -> usize {
        ImmutableArray::weak_count(&instance.internal)
    }

    /// Returns the length of the [`ImmutableString`].
    #[inline(always)]
    pub fn len(&self) -> Length {
        self.internal.len()
    }

    /// Returns `true` if the [`ImmutableString`] is empty, `false` otherwise.
    #[inline(always)]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len().value() == 0
    }

    /// Releases the [`ImmutableString`]. If this was the last strong reference,
    /// it will return a [`ImmutableWeakString`]. Otherwise it will return `None`.
    pub fn release(mut instance: Self) -> Option<ImmutableWeakString<TAllocator>> {
        let internal = unsafe { ManuallyDrop::take(&mut instance.internal) };
        let weak = ImmutableArray::release(internal)?;
        core::mem::forget(instance);
        Some(ImmutableWeakString::from_internal(weak))
    }

    /// Constructs a new [`ImmutableString`] from an [`ImmutableArray<u8>`].
    ///
    /// # Safety
    ///
    /// This method does not check if the array is a valid UTF-8 string.
    #[inline(always)]
    pub unsafe fn from_unchecked(value: ImmutableArray<u8, TAllocator>) -> Self {
        Self {
            internal: ManuallyDrop::new(value),
        }
    }
}

impl<TAllocator: Allocator> Drop for ImmutableString<TAllocator> {
    fn drop(&mut self) {
        unsafe { ManuallyDrop::drop(&mut self.internal) };
    }
}

impl<TAllocator: Allocator> core::ops::Deref for ImmutableString<TAllocator> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl<TAllocator: Allocator> core::fmt::Debug for ImmutableString<TAllocator> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ImmutableWeakString")
            .field("strong_count", &Self::strong_count(self))
            .field("weak_count", &Self::weak_count(self))
            .field("len", &self.len())
            .field("capacity", &self.internal.capacity())
            .finish()
    }
}

impl<TAllocator1: Allocator, TAllocator2: Allocator> core::cmp::PartialEq<ImmutableString<TAllocator1>>
    for ImmutableString<TAllocator2>
{
    fn eq(&self, other: &ImmutableString<TAllocator1>) -> bool {
        self.as_str() == other.as_str()
    }
}

impl<TAllocator: Allocator> core::cmp::Eq for ImmutableString<TAllocator> {}

impl<TAllocator: Allocator> core::hash::Hash for ImmutableString<TAllocator> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.internal.hash(state);
    }
}

impl<TAllocator: Allocator> TryFrom<ImmutableArray<u8, TAllocator>> for ImmutableString<TAllocator> {
    type Error = core::str::Utf8Error;

    fn try_from(value: ImmutableArray<u8, TAllocator>) -> Result<Self, Self::Error> {
        let _ = core::str::from_utf8(value.as_slice())?;
        Ok(Self {
            internal: ManuallyDrop::new(value),
        })
    }
}

impl<TAllocator: Allocator> From<ImmutableString<TAllocator>> for ImmutableArray<u8, TAllocator> {
    fn from(mut value: ImmutableString<TAllocator>) -> Self {
        let internal = unsafe { ManuallyDrop::take(&mut value.internal) };
        core::mem::forget(value);
        internal
    }
}
