use core::alloc::Layout;
use core::fmt::Debug;
use core::ptr::NonNull;

/// Represents an error that occurs when allocating memory.
/// Most likely due to out of memory. Concrete allocators can
/// extend this error with more information.
#[derive(Debug)]
#[must_use]
pub struct DetailedAllocationError<T: Sized> {
    pub details: T,
}

/// Represents a generic allocation error. It discards any additional
/// data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[must_use]
pub struct AllocationError;

impl<T: Sized> From<DetailedAllocationError<T>> for AllocationError {
    fn from(_: DetailedAllocationError<T>) -> Self {
        AllocationError
    }
}

/// Represents a memory allocator.
///
/// # Safety
///
/// This trait is inherently unsafe, because it deals with raw pointers
/// and memory management.
#[must_use]
pub unsafe trait Allocator: Default + Clone + Debug + Send + Sync {
    type ErrorDetails: Sized;

    /// Allocates a new piece of memory with the given layout.
    ///
    /// # Errors
    ///
    /// Returns an [`AllocationError`] if the memory cannot be allocated.
    fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, DetailedAllocationError<Self::ErrorDetails>>;

    /// Allocates a new piece of memory with the layout of the type `T`.
    ///
    /// # Errors
    ///
    /// Returns an [`AllocationError`] if the memory cannot be allocated.
    fn allocate_for_type<T: Sized>(&self) -> Result<NonNull<T>, DetailedAllocationError<Self::ErrorDetails>> {
        let layout = Layout::new::<T>();
        let result = self.allocate(layout)?;
        Ok(unsafe { NonNull::new_unchecked(result.as_ptr().cast()) })
    }

    /// Resizes the memory block pointed to by `ptr` to a new layout.
    ///
    /// # Errors
    ///
    /// Returns an [`AllocationError`] if the memory cannot be resized.
    ///
    /// # Safety
    ///
    /// The passed pointer must not be used after the call.
    unsafe fn resize(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<u8>, DetailedAllocationError<Self::ErrorDetails>>;

    /// Deallocates the memory block pointed to by `ptr`.
    ///
    /// # Safety
    ///
    /// The passed pointer must not be used after the call.
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout);

    /// Deallocates the memory block pointed to by `ptr`, with the layout of the type `T`.
    ///
    /// # Safety
    ///
    /// The passed pointer must not be used after the call.
    unsafe fn deallocate_for_type<T: Sized>(&self, ptr: NonNull<T>) {
        let layout = Layout::new::<T>();
        unsafe { self.deallocate(ptr.cast(), layout) };
    }

    /// Creates a new dangling pointer. This pointer
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
    unsafe fn dangling<T: Sized>(&self) -> NonNull<T>;
}
