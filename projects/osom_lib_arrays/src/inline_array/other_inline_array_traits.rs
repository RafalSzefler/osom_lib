use core::borrow::{Borrow, BorrowMut};
use core::hash::Hash;

use osom_lib_alloc::traits::Allocator;

use crate::traits::MutableArray;

use super::InlineArray;

impl<const TCAPACITY: usize, T: Clone, TAllocator> Clone for InlineArray<TCAPACITY, T, TAllocator>
where
    TAllocator: Allocator,
{
    fn clone(&self) -> Self {
        let mut new_array = InlineArray::with_capacity_and_allocator(self.size, self.allocator.clone())
            .expect("Couldn't create a new InlineArray during clone() call");

        new_array.push_slice(self.as_slice_internal());
        new_array
    }
}

impl<const TCAPACITY: usize, T, TAllocator, Rhs> PartialEq<Rhs> for InlineArray<TCAPACITY, T, TAllocator>
where
    T: PartialEq,
    TAllocator: Allocator,
    Rhs: AsRef<[T]>,
{
    fn eq(&self, other: &Rhs) -> bool {
        self.as_slice_internal() == other.as_ref()
    }
}

impl<const TCAPACITY: usize, T, TAllocator> Eq for InlineArray<TCAPACITY, T, TAllocator>
where
    T: Eq,
    TAllocator: Allocator,
{
}

impl<const TCAPACITY: usize, T, TAllocator> Hash for InlineArray<TCAPACITY, T, TAllocator>
where
    T: Hash,
    TAllocator: Allocator,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_slice_internal().hash(state);
    }
}

impl<const TCAPACITY: usize, T, TAllocator> AsRef<[T]> for InlineArray<TCAPACITY, T, TAllocator>
where
    TAllocator: Allocator,
{
    fn as_ref(&self) -> &[T] {
        self.as_slice_internal()
    }
}

impl<const TCAPACITY: usize, T, TAllocator> AsMut<[T]> for InlineArray<TCAPACITY, T, TAllocator>
where
    TAllocator: Allocator,
{
    fn as_mut(&mut self) -> &mut [T] {
        self.as_slice_mut()
    }
}

impl<const TCAPACITY: usize, T, TAllocator> Borrow<[T]> for InlineArray<TCAPACITY, T, TAllocator>
where
    TAllocator: Allocator,
{
    fn borrow(&self) -> &[T] {
        self.as_slice_internal()
    }
}

impl<const TCAPACITY: usize, T, TAllocator> BorrowMut<[T]> for InlineArray<TCAPACITY, T, TAllocator>
where
    TAllocator: Allocator,
{
    fn borrow_mut(&mut self) -> &mut [T] {
        self.as_slice_mut_internal()
    }
}
