#![allow(dead_code)]

use std::sync::{Arc, atomic::AtomicI32};

use osom_lib_arrays::traits::MutableArray;
use osom_lib_primitives::{length::Length, macros::make_length};
use osom_lib_reprc::traits::ReprC;

pub fn test_mutable_array<TArr: MutableArray<i32>, Builder: FnOnce() -> TArr>(array_builder: Builder) {
    let mut array = array_builder();
    assert_eq!(array.as_ref(), &[]);
    assert_eq!(array.as_mut(), &[]);
    assert_eq!(array.length(), Length::ZERO);
    assert!(array.is_empty());
    assert!(array.try_pop().is_err());

    array.push(1);
    assert_eq!(array.as_ref(), &[1]);
    assert_eq!(array.as_mut(), &[1]);
    assert_eq!(array.length(), Length::ONE);
    assert!(!array.is_empty());

    array.push_array([5, -1, 3]);
    assert_eq!(array.as_ref(), &[1, 5, -1, 3]);
    assert_eq!(array.as_mut(), &[1, 5, -1, 3]);
    assert_eq!(array.length(), make_length!(4));
    assert!(!array.is_empty());

    array.push_slice(&[1, 1, 1, 2, 2, 2]);
    assert_eq!(array.as_ref(), &[1, 5, -1, 3, 1, 1, 1, 2, 2, 2]);
    assert_eq!(array.as_mut(), &[1, 5, -1, 3, 1, 1, 1, 2, 2, 2]);
    assert_eq!(array.length(), make_length!(10));
    assert!(!array.is_empty());

    assert_eq!(array.pop(), 2);
    assert_eq!(array.as_ref(), &[1, 5, -1, 3, 1, 1, 1, 2, 2]);
    assert_eq!(array.as_mut(), &[1, 5, -1, 3, 1, 1, 1, 2, 2]);
    assert_eq!(array.length(), make_length!(9));
    assert!(!array.is_empty());

    assert_eq!(array.pop(), 2);
    assert_eq!(array.as_ref(), &[1, 5, -1, 3, 1, 1, 1, 2]);
    assert_eq!(array.as_mut(), &[1, 5, -1, 3, 1, 1, 1, 2]);
    assert_eq!(array.length(), make_length!(8));
    assert!(!array.is_empty());

    let _ = array.pop();
    let _ = array.pop();
    let _ = array.pop();
    let _ = array.pop();
    let _ = array.pop();
    assert_eq!(array.as_ref(), &[1, 5, -1]);
    assert_eq!(array.as_mut(), &[1, 5, -1]);
    assert_eq!(array.length(), make_length!(3));
    assert!(!array.is_empty());

    assert_eq!(array.pop(), -1);
    assert_eq!(array.as_ref(), &[1, 5]);
    assert_eq!(array.as_mut(), &[1, 5]);
    assert_eq!(array.length(), make_length!(2));
    assert!(!array.is_empty());

    assert_eq!(array.pop(), 5);
    assert_eq!(array.as_ref(), &[1]);
    assert_eq!(array.as_mut(), &[1]);
    assert_eq!(array.length(), make_length!(1));
    assert!(!array.is_empty());

    assert_eq!(array.pop(), 1);
    assert_eq!(array.as_ref(), &[]);
    assert_eq!(array.as_mut(), &[]);
    assert_eq!(array.length(), make_length!(0));
    assert!(array.is_empty());

    assert!(array.try_pop().is_err());
}

#[repr(transparent)]
pub struct DropCounter {
    counter: Arc<AtomicI32>,
}

unsafe impl ReprC for DropCounter {
    const CHECK: () = ();
}

impl DropCounter {
    #[inline(always)]
    pub fn new(counter: Arc<AtomicI32>) -> Self {
        counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        Self { counter }
    }
}

impl Drop for DropCounter {
    fn drop(&mut self) {
        self.counter.fetch_add(-1, std::sync::atomic::Ordering::SeqCst);
    }
}

pub fn test_array_destruction<'a, TArr: MutableArray<DropCounter>, Builder: FnOnce() -> TArr>(array_builder: Builder) {
    let counter = Arc::new(AtomicI32::new(0));
    let mut array = array_builder();
    for _ in 0..10 {
        array.push(DropCounter::new(counter.clone()));
    }

    assert_eq!(counter.load(std::sync::atomic::Ordering::SeqCst), 10);

    for _ in 0..3 {
        let item = array.pop();
        drop(item);
    }

    assert_eq!(counter.load(std::sync::atomic::Ordering::SeqCst), 7);

    drop(array);

    assert_eq!(counter.load(std::sync::atomic::Ordering::SeqCst), 0);
}

pub fn test_array_clone<'a, TArr: MutableArray<i32> + Clone, Builder: FnOnce() -> TArr>(array_builder: Builder) {
    let mut array = array_builder();
    assert_eq!(array.length(), Length::ZERO);
    for idx in 0..10 {
        array.push(2 * idx - 1);
    }
    assert_eq!(array.length().as_u32(), 10);
    let mut clone = array.clone();
    assert_eq!(clone.length().as_u32(), 10);
    assert_eq!(array.as_ref(), clone.as_ref());

    clone.push(124);

    assert_eq!(array.length().as_u32(), 10);
    assert_eq!(clone.length().as_u32(), 11);
    assert_ne!(array.as_ref(), clone.as_ref());
}

pub fn test_array_back_and_forth<'a, TArr: MutableArray<i32> + Clone, Builder: FnOnce() -> TArr>(
    array_builder: Builder,
) {
    let mut array = array_builder();

    for _ in 0..10 {
        assert_eq!(array.length(), Length::ZERO);
        for idx in 0..500 {
            array.push(2 * idx - 1);
        }

        let mut length = 500;
        assert_eq!(array.length().as_u32(), length);
        assert_eq!(array.pop(), 2 * 499 - 1);
        length -= 1;
        assert_eq!(array.length().as_u32(), length);

        for idx in (0..499).rev() {
            assert_eq!(array.pop(), 2 * idx - 1);
            length -= 1;
            assert_eq!(array.length().as_u32(), length);
        }

        assert!(array.try_pop().is_err());
    }
}
