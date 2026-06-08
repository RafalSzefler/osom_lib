use core::borrow::{Borrow, BorrowMut};
use core::hash::Hash;

use osom_lib_alloc::traits::Allocator;
use osom_lib_try_clone::TryClone;

use crate::errors::{ArrayError, ArrayTryCloneError};
use crate::traits::MutableArray as _;

use super::InlineDynamicArray;

impl<const TCAPACITY: usize, T, TAllocator> TryClone for InlineDynamicArray<TCAPACITY, T, TAllocator>
where
    T: TryClone,
    TAllocator: Allocator + TryClone,
{
    type Error = ArrayTryCloneError;

    /// Tries to clone the array. This is a perfect clone, with exactly the same capacity.
    ///
    /// # Errors
    ///
    /// For details see [`ArrayError`].
    fn try_clone(&self) -> Result<Self, Self::Error> {
        let allocator = self.allocator.try_clone().map_err(|_| ArrayError::AllocationError)?;
        let mut new_array = Self::with_capacity_and_allocator(self.capacity, allocator)?;
        new_array.try_push_slice(self.as_slice_internal())?;
        Ok(new_array)
    }
}

impl<const TCAPACITY: usize, T, TAllocator> Clone for InlineDynamicArray<TCAPACITY, T, TAllocator>
where
    T: TryClone,
    TAllocator: Allocator + TryClone,
{
    fn clone(&self) -> Self {
        self.try_clone().expect("Failed to clone inline dynamic array")
    }
}

impl<const TCAPACITY: usize, T, TAllocator, Rhs> PartialEq<Rhs> for InlineDynamicArray<TCAPACITY, T, TAllocator>
where
    T: PartialEq,
    TAllocator: Allocator,
    Rhs: AsRef<[T]>,
{
    fn eq(&self, other: &Rhs) -> bool {
        self.as_slice_internal() == other.as_ref()
    }
}

impl<const TCAPACITY: usize, T, TAllocator> Eq for InlineDynamicArray<TCAPACITY, T, TAllocator>
where
    T: Eq,
    TAllocator: Allocator,
{
}

impl<const TCAPACITY: usize, T, TAllocator> Hash for InlineDynamicArray<TCAPACITY, T, TAllocator>
where
    T: Hash,
    TAllocator: Allocator,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_slice_internal().hash(state);
    }
}

impl<const TCAPACITY: usize, T, TAllocator> AsRef<[T]> for InlineDynamicArray<TCAPACITY, T, TAllocator>
where
    TAllocator: Allocator,
{
    fn as_ref(&self) -> &[T] {
        self.as_slice_internal()
    }
}

impl<const TCAPACITY: usize, T, TAllocator> AsMut<[T]> for InlineDynamicArray<TCAPACITY, T, TAllocator>
where
    TAllocator: Allocator,
{
    fn as_mut(&mut self) -> &mut [T] {
        self.as_slice_mut_internal()
    }
}

impl<const TCAPACITY: usize, T, TAllocator> Borrow<[T]> for InlineDynamicArray<TCAPACITY, T, TAllocator>
where
    TAllocator: Allocator,
{
    fn borrow(&self) -> &[T] {
        self.as_ref()
    }
}

impl<const TCAPACITY: usize, T, TAllocator> BorrowMut<[T]> for InlineDynamicArray<TCAPACITY, T, TAllocator>
where
    TAllocator: Allocator,
{
    fn borrow_mut(&mut self) -> &mut [T] {
        self.as_mut()
    }
}
