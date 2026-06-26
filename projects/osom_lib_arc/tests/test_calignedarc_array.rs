#![cfg(feature = "std")]

use std::sync::{Arc, atomic::AtomicU32};

use osom_lib_arc::std::{StdCAlignedArcArray, StdCAlignedArcArrayBuilder};

#[repr(align(128))]
struct HighAlign;

const _: () = const {
    assert!(align_of::<HighAlign>() == 128);
};

#[test]
fn test_caligned_arc_array() {
    let mut builder = StdCAlignedArcArrayBuilder::<HighAlign, _>::new().unwrap();
    builder.try_push_slice(&[1, 2, 3]).unwrap();
    let arc = builder.build();
    assert_eq!(StdCAlignedArcArray::strong_count(&arc), 1);
    assert_eq!(StdCAlignedArcArray::weak_count(&arc), 1);
    let weak = StdCAlignedArcArray::downgrade(&arc).unwrap();
    assert_eq!(StdCAlignedArcArray::strong_count(&arc), 1);
    assert_eq!(StdCAlignedArcArray::weak_count(&arc), 2);
    let up = weak.upgrade().unwrap();
    assert_eq!(StdCAlignedArcArray::strong_count(&arc), 2);
    assert_eq!(StdCAlignedArcArray::weak_count(&arc), 2);
    drop(weak);
    assert_eq!(StdCAlignedArcArray::strong_count(&arc), 2);
    assert_eq!(StdCAlignedArcArray::weak_count(&arc), 1);
    drop(up);
    assert_eq!(StdCAlignedArcArray::strong_count(&arc), 1);
    assert_eq!(StdCAlignedArcArray::weak_count(&arc), 1);
    let arc_clone = arc.clone();
    assert_eq!(StdCAlignedArcArray::strong_count(&arc), 2);
    assert_eq!(StdCAlignedArcArray::weak_count(&arc), 1);
    assert_eq!(StdCAlignedArcArray::strong_count(&arc_clone), 2);
    assert_eq!(StdCAlignedArcArray::weak_count(&arc_clone), 1);
    assert_eq!(StdCAlignedArcArray::data(&arc), &[1, 2, 3]);
    let data = StdCAlignedArcArray::data(&arc_clone);
    assert_eq!(data, &[1, 2, 3]);
    assert!(data.as_ptr().align_offset(align_of::<HighAlign>()) == 0);
}

#[test]
fn test_caligned_drop() {
    struct Data {
        pub counter: Arc<AtomicU32>,
    }

    impl Drop for Data {
        fn drop(&mut self) {
            self.counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        }
    }

    let counter = Arc::new(AtomicU32::new(0));
    let get = || counter.load(std::sync::atomic::Ordering::SeqCst);

    let mut builder = StdCAlignedArcArrayBuilder::<HighAlign, _>::new().unwrap();
    for _ in 0..3 {
        builder
            .try_push_array([Data {
                counter: counter.clone(),
            }])
            .unwrap();
    }
    let data = builder.build();

    assert!(align_of::<Data>() < align_of::<HighAlign>());
    assert!(data.as_ref().as_ptr().align_offset(align_of::<HighAlign>()) == 0);

    assert_eq!(get(), 0);
    let data2 = data.clone();
    assert_eq!(get(), 0);
    drop(data2);
    assert_eq!(get(), 0);
    drop(data);
    assert_eq!(get(), 3);
}
