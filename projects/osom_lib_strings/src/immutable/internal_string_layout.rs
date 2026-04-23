#![allow(clippy::struct_field_names)]
use core::{alloc::Layout, sync::atomic::AtomicU32};

pub struct HeapObjectLayout {
    pub strong_offset: usize,
    pub weak_offset: usize,
    pub data_offset: usize,
}

#[inline(always)]
const fn max(left: usize, right: usize) -> usize {
    if left < right { right } else { left }
}

impl HeapObjectLayout {
    pub const fn calculate_for_data_size(&self, data_size: usize) -> Layout {
        let align = max(align_of::<u8>(), align_of::<AtomicU32>());
        unsafe { Layout::from_size_align_unchecked(self.data_offset + data_size, align) }
    }
}

#[inline(always)]
const fn layout_for<T>() -> Layout {
    unsafe { Layout::from_size_align_unchecked(size_of::<T>(), align_of::<T>()) }
}

pub const HEAP_OBJECT_LAYOUT: HeapObjectLayout = const {
    let layout = Layout::new::<()>();
    let Ok((layout, strong_offset)) = layout.extend(layout_for::<AtomicU32>()) else {
        panic!("Couldn't calcualte HeapObjectLayout")
    };

    let Ok((layout, weak_offset)) = layout.extend(layout_for::<AtomicU32>()) else {
        panic!("Couldn't calcualte HeapObjectLayout")
    };

    let data_layout = unsafe { Layout::from_size_align_unchecked(1, align_of::<u8>()) };
    let Ok((_, data_offset)) = layout.extend(data_layout) else {
        panic!("Couldn't calcualte HeapObjectLayout")
    };

    assert!(data_offset <= 1024);

    HeapObjectLayout {
        strong_offset,
        weak_offset,
        data_offset,
    }
};
