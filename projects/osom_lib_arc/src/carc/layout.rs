use core::{alloc::Layout, marker::PhantomData, sync::atomic::AtomicU32};

use osom_lib_alloc::traits::Allocator;

pub struct CArcLayout<T, TAllocator: Allocator> {
    pub allocator_offset: usize,
    pub strong_offset: usize,
    pub weak_offset: usize,
    pub data_offset: usize,
    pub total_layout: Layout,
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
            strong_offset,
            weak_offset,
            data_offset,
            total_layout: layout,
            _phantom: PhantomData,
        }
    }
}
