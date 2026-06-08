#![cfg(feature = "std")]

use core::ops::Deref;
use std::sync::{Arc, atomic::AtomicU32};

use osom_lib_boxed::std::StdCBox;
use osom_lib_try_clone::TryClone;

#[test]
fn test_box() {
    let mut box_ = StdCBox::new(0).unwrap();
    assert_eq!(box_.deref(), &0);
    *box_ = 1;
    assert_eq!(box_.deref(), &1);
    let value = StdCBox::unpack(box_);
    assert_eq!(value, 1);
}

#[test]
fn test_drop() {
    #[derive(Clone)]
    struct Data {
        pub counter: Arc<AtomicU32>,
    }

    impl TryClone for Data {
        type Error = core::convert::Infallible;

        fn try_clone(&self) -> Result<Self, Self::Error> {
            Ok(self.clone())
        }
    }

    impl Drop for Data {
        fn drop(&mut self) {
            self.counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        }
    }

    let counter = Arc::new(AtomicU32::new(0));
    let get = || counter.load(std::sync::atomic::Ordering::SeqCst);

    let box1 = StdCBox::new(Data {
        counter: counter.clone(),
    })
    .unwrap();

    assert_eq!(get(), 0);
    let box2 = box1.clone();
    assert_eq!(get(), 0);

    drop(box1);
    assert_eq!(get(), 1);

    drop(box2);
    assert_eq!(get(), 2);
}

#[test]
fn test_raw_ptr() {
    let box_ = StdCBox::<i32>::new(13).unwrap();
    assert_eq!(*box_, 13);
    let raw_ptr = StdCBox::into_raw_ptr(box_);
    let box_ = unsafe { StdCBox::<i32>::from_raw_ptr(raw_ptr) };
    assert_eq!(*box_, 13);
}
