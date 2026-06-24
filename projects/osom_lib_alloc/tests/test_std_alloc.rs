#![cfg(feature = "std")]

use std::{
    alloc::Layout,
    sync::{Arc, atomic::AtomicU32},
};

use osom_lib_alloc::{std_allocator::StdAllocator, traits::Allocator};

#[test]
fn test_alloc() {
    #[repr(C)]
    struct Data {
        pub val1: u32,
        pub val2: u64,
        pub counter: Arc<AtomicU32>,
    }

    impl Drop for Data {
        fn drop(&mut self) {
            self.counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        }
    }

    const SIZE: usize = 7;

    let counter = Arc::new(AtomicU32::new(0));
    let mut allocator = StdAllocator;
    let layout = unsafe { Layout::from_size_align_unchecked(size_of::<Data>() * SIZE, align_of::<Data>()) };
    let ptr = allocator.allocate(layout).unwrap();
    let data = ptr.cast::<Data>();

    let mut current = data;
    for idx in 0..SIZE {
        unsafe {
            let data = Data {
                val1: idx as u32,
                val2: (idx as u64) * 2,
                counter: counter.clone(),
            };
            current.write(data);
            current = current.add(1);
        }
    }

    let mut current = data;
    for _ in 0..SIZE {
        unsafe {
            std::ptr::drop_in_place(current.as_ptr());
            current = current.add(1);
        }
    }

    unsafe { allocator.deallocate(ptr, layout) };

    let value = counter.load(std::sync::atomic::Ordering::SeqCst);
    assert_eq!(value, SIZE as u32);
}
