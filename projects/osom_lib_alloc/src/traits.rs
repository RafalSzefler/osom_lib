use core::alloc::Layout;
use core::fmt::Debug;
use core::hash::Hash;

/// Represents an error that occurs when allocating memory.
/// Most likely due to out of memory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[must_use]
pub struct AllocationError;

/// Represents a newly allocated piece of memory. Typically a thin wrapper
/// around raw `*mut u8` pointer, but type safe.
///
/// # Safety
///
/// Any type implementing this trait must ensure that the memory
/// is ultimately deallocated by calling [`AllocatedMemory::deallocate`].
#[must_use]
pub unsafe trait AllocatedMemory: Sized + PartialEq + Eq + Clone + Hash + Debug {
    /// Converts the [`AllocatedMemory`] into a raw pointer.
    ///
    /// # Safety
    ///
    /// This function is unsafe since it doesn't move ownership, and so
    /// multiple copies of the same memory can exist. Moreover it does not
    /// validate the pointer, in particular it does not check whether it is
    /// properly aligned for the type `T`.
    unsafe fn as_ptr<T: Sized>(&self) -> *mut T;

    /// Resizes the [`AllocatedMemory`] to a new layout.
    ///
    /// # Errors
    ///
    /// Returns an [`AllocationError`] if the memory cannot be resized.
    fn resize(self, old_layout: Layout, new_layout: Layout) -> Result<Self, AllocationError>;

    /// Deallocates the [`AllocatedMemory`].
    fn deallocate(self, layout: Layout);
}

/// Represents a memory allocator.
///
/// # Safety
///
/// This trait is inherently unsafe, because it deals with raw pointers
/// and memory management.
#[must_use]
pub unsafe trait Allocator: Default + Clone + Debug + Send + Sync {
    type TAllocatedMemory: AllocatedMemory;

    /// Allocates a new [`AllocatedMemory`] with the given layout.
    ///
    /// # Errors
    ///
    /// Returns an [`AllocationError`] if the memory cannot be allocated.
    fn allocate(&self, layout: Layout) -> Result<Self::TAllocatedMemory, AllocationError>;

    /// Creates an [`AllocatedMemory`] from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must originally arise from calling [`Allocator::allocate`].
    /// Otherwise the behaviour is undefined.
    unsafe fn convert_raw_ptr<T: Sized>(&self, ptr: *mut T) -> Self::TAllocatedMemory;

    /// Creates a new dangling [`AllocatedMemory`]. This pointer
    /// is non-zero, not valid but well-aligned. Note that it should
    /// not be deallocated, nor dereferenced. It does however represent
    /// a valid pointer to the type `T`.
    ///
    /// It is useful for example for creating slices of length 0
    /// and other lazily-initialized things.
    ///
    /// # Safety
    ///
    /// This function is unsafe, because the pointer is not valid.
    /// It is up to the caller to ensure that it is not used directly,
    /// in particular it should never be dereferenced and deallocated.
    unsafe fn dangling<T: Sized>(&self) -> Self::TAllocatedMemory;
}
