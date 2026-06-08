use core::ops::Index;

use osom_lib_alloc::traits::Allocator;
use osom_lib_primitives::length::Length;

use crate::traits::ImmutableArray;

use super::InlineDynamicArray;

impl<const TCAPACITY: usize, T, TAllocator> Default for InlineDynamicArray<TCAPACITY, T, TAllocator>
where
    T: Sized,
    TAllocator: Allocator + Default,
{
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl<const TCAPACITY: usize, T, TAllocator> Index<Length> for InlineDynamicArray<TCAPACITY, T, TAllocator>
where
    T: Sized,
    TAllocator: Allocator,
{
    type Output = T;

    fn index(&self, index: Length) -> &Self::Output {
        &self.as_slice_internal()[index.as_usize()]
    }
}

impl<const TCAPACITY: usize, T, TAllocator> ImmutableArray<T> for InlineDynamicArray<TCAPACITY, T, TAllocator>
where
    T: Sized,
    TAllocator: Allocator,
{
    #[inline(always)]
    fn length(&self) -> Length {
        self.size
    }

    #[inline(always)]
    fn capacity(&self) -> Length {
        self.capacity
    }
}
