#![cfg(feature = "std")]

use std::sync::{Arc, atomic::AtomicU32};

use osom_lib_arc::std::{StdCArcArray, StdCArcArrayBuilder};

#[test]
fn test_carc() {
    let mut builder = StdCArcArrayBuilder::new().unwrap();
    builder.try_push_slice(&[1, 2, 3]).unwrap();
    let arc = builder.build();
    assert_eq!(StdCArcArray::strong_count(&arc), 1);
    assert_eq!(StdCArcArray::weak_count(&arc), 1);
    let weak = StdCArcArray::downgrade(&arc).unwrap();
    assert_eq!(StdCArcArray::strong_count(&arc), 1);
    assert_eq!(StdCArcArray::weak_count(&arc), 2);
    let up = weak.upgrade().unwrap();
    assert_eq!(StdCArcArray::strong_count(&arc), 2);
    assert_eq!(StdCArcArray::weak_count(&arc), 2);
    drop(weak);
    assert_eq!(StdCArcArray::strong_count(&arc), 2);
    assert_eq!(StdCArcArray::weak_count(&arc), 1);
    drop(up);
    assert_eq!(StdCArcArray::strong_count(&arc), 1);
    assert_eq!(StdCArcArray::weak_count(&arc), 1);

    let arc_clone = arc.clone();
    assert_eq!(StdCArcArray::strong_count(&arc), 2);
    assert_eq!(StdCArcArray::weak_count(&arc), 1);
    assert_eq!(StdCArcArray::strong_count(&arc_clone), 2);
    assert_eq!(StdCArcArray::weak_count(&arc_clone), 1);
    assert_eq!(StdCArcArray::data(&arc), &[1, 2, 3]);
    assert_eq!(StdCArcArray::data(&arc_clone), &[1, 2, 3]);
}

#[test]
fn test_drop() {
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

    let mut builder = StdCArcArrayBuilder::new().unwrap();
    for _ in 0..3 {
        builder
            .try_push_array([Data {
                counter: counter.clone(),
            }])
            .unwrap();
    }
    let data = builder.build();

    assert_eq!(get(), 0);
    let data2 = data.clone();
    assert_eq!(get(), 0);
    drop(data2);
    assert_eq!(get(), 0);
    drop(data);
    assert_eq!(get(), 3);
}
