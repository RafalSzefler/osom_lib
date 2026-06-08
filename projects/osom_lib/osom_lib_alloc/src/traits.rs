//! Defines allocator traits.

use core::{alloc::Layout, fmt::Debug, ptr::NonNull};

use osom_lib_reprc::traits::ReprC;

/// Represents a trait for types that can provide a message.
pub trait WithMessage {
    fn message(&self) -> &str;
}

/// Represents an Allocator.
///
/// # Safety
///
/// This trait is inherently unsafe, since it depends on well managed
/// raw pointers. For example it is possible to call [`Allocator::deallocate`]
/// twice on the same pointer, which is an Undefined Behaviour.
pub unsafe trait Allocator: Debug + ReprC {
    /// A specific Allocator error. Implementor can add additional
    /// information to the error, not only generic "allocation failed".
    type SpecificAllocationError: Debug + ReprC + WithMessage;

    /// Allocates new piece of memory. This is the only safe
    /// function here. The returned `ptr` is guaranteed to satisfy
    /// `layout` requirements (although the implementation is free
    /// to overallocate and strengthen alignment).
    ///
    /// # Errors
    ///
    /// An error typically means out-of-memory error. But the implementation
    /// is allowed to provide additional info.
    fn allocate(&mut self, layout: Layout) -> Result<NonNull<u8>, Self::SpecificAllocationError>;

    /// Deallocates memory.
    ///
    /// # Safety
    ///
    /// Passed `ptr` has to be created with previous call to [`Allocator::allocate`]
    /// or [`Allocator::resize`]. Layouts have to match. Using the passed `ptr`
    /// after the call to this function is an Undefined Behaviour.
    unsafe fn deallocate(&mut self, ptr: NonNull<u8>, layout: Layout);

    /// Resizes given `ptr` to the new layout.
    ///
    /// # Errors
    ///  
    /// An error typically means out-of-memory error. But the implementation
    /// is allowed to provide additional info.
    ///
    /// # Safety
    ///
    /// `ptr` has to be a valid pointer created with previous call to
    /// [`Allocator::allocate`] or [`Allocator::resize`]. `old_layout`
    /// has to match the pointers layout. Using the passed ptr after
    /// the call to this function is an Undefined Behaviour. Use the
    /// return value instead.
    unsafe fn resize(
        &mut self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<u8>, Self::SpecificAllocationError> {
        unsafe {
            let new_ptr = self.allocate(new_layout)?;
            let new_size = core::cmp::min(new_layout.size(), old_layout.size());
            new_ptr.copy_from_nonoverlapping(ptr, new_size);
            self.deallocate(ptr, old_layout);
            Ok(new_ptr)
        }
    }
}
