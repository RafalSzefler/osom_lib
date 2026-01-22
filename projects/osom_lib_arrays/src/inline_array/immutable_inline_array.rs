use core::ops::Index;

use osom_lib_alloc::traits::Allocator;
use osom_lib_primitives::length::Length;

use crate::traits::ImmutableArray;

use super::InlineArray;

impl<const TCAPACITY: usize, T, TAllocator> Default for InlineArray<TCAPACITY, T, TAllocator>
where
    T: Sized,
    TAllocator: Allocator,
{
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl<const TCAPACITY: usize, T, TAllocator> Index<Length> for InlineArray<TCAPACITY, T, TAllocator>
where
    T: Sized,
    TAllocator: Allocator,
{
    type Output = T;

    fn index(&self, index: Length) -> &Self::Output {
        &self.as_slice_internal()[index.as_usize()]
    }
}

impl<const TCAPACITY: usize, T, TAllocator> ImmutableArray<T> for InlineArray<TCAPACITY, T, TAllocator>
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

    #[inline(always)]
    fn as_slice(&self) -> &[T] {
        self.as_slice_internal()
    }
}
