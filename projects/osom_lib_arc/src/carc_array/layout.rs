use core::{alloc::Layout, marker::PhantomData, sync::atomic::AtomicU32};

use osom_lib_alloc::traits::Allocator;

use crate::errors::CArcArrayError;

#[must_use]
pub struct CArcLayout<T, TAllocator: Allocator> {
    pub allocator_offset: usize,
    pub size_offset: usize,
    pub capacity_offset: usize,
    pub strong_offset: usize,
    pub weak_offset: usize,
    pub data_offset: usize,
    pub alignment: usize,
    _phantom: PhantomData<(T, TAllocator)>,
}

const fn layout_for<T>() -> Layout {
    unsafe { Layout::from_size_align_unchecked(size_of::<T>(), align_of::<T>()) }
}

impl<T, TAllocator: Allocator> CArcLayout<T, TAllocator> {
    pub const fn new() -> Self {
        let layout = unsafe { Layout::from_size_align_unchecked(0, 1) };
        let Ok((layout, allocator_offset)) = layout.extend(layout_for::<TAllocator>()) else {
            panic!("Couldn't calculate CArcLayout for TAllocator.");
        };

        let Ok((layout, size_offset)) = layout.extend(layout_for::<u32>()) else {
            panic!("Couldn't calculate CArcLayout for u32 for size.");
        };

        let Ok((layout, capacity_offset)) = layout.extend(layout_for::<u32>()) else {
            panic!("Couldn't calculate CArcLayout for u32 for capacity.");
        };

        let Ok((layout, strong_offset)) = layout.extend(layout_for::<AtomicU32>()) else {
            panic!("Couldn't calculate CArcLayout for AtomicU32.");
        };

        let Ok((layout, weak_offset)) = layout.extend(layout_for::<AtomicU32>()) else {
            panic!("Couldn't calculate CArcLayout for AtomicU32.");
        };

        let Ok((layout, data_offset)) = layout.extend(layout_for::<T>()) else {
            panic!("Couldn't calculate CArcLayout for AtomicU32.");
        };

        Self {
            allocator_offset,
            size_offset,
            capacity_offset,
            strong_offset,
            weak_offset,
            data_offset,
            alignment: layout.align(),
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub const fn calculate_for_capacity(&self, capacity: u32) -> Result<Layout, CArcArrayError> {
        const {
            assert!(
                size_of::<usize>() >= size_of::<u32>(),
                "size_of::<usize>() must be greater than size_of::<u32>()"
            );
        };
        let Some(total_size) = size_of::<T>().checked_mul(capacity as usize) else {
            return Err(CArcArrayError::ArraySizeOutOfRange);
        };
        let Some(total_size) = total_size.checked_add(self.data_offset) else {
            return Err(CArcArrayError::ArraySizeOutOfRange);
        };
        Ok(unsafe { Layout::from_size_align_unchecked(total_size, self.alignment) })
    }
}
