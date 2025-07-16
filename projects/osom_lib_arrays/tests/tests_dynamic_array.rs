#![cfg(feature = "std_alloc")]

use std::{
    ops::Deref,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

use osom_lib_arrays::StdDynamicArray;

use osom_lib_primitives::Length;
use rstest::rstest;

#[rstest]
#[case(&[])]
#[case(&[1, 2])]
#[case(&[-1, 12, 0])]
#[case(&[-100, 2, -6, -7, 0, 12, 165, 111111])]
fn test_push_and_compare(#[case] data: &[i32]) {
    let mut dynamic_vec = StdDynamicArray::<i32>::new();
    for value in data {
        dynamic_vec.push(*value).unwrap();
    }

    assert_eq!(dynamic_vec.as_slice(), data);
    assert_eq!(dynamic_vec.deref(), data);

    let mut other = StdDynamicArray::<i32>::new();
    for value in data {
        other.push(*value).unwrap();
    }

    assert_eq!(dynamic_vec, other);
}

#[test]
fn test_incremental_push() {
    const MAX: usize = 100;
    let mut vec = Vec::<u32>::with_capacity(MAX);
    let mut arr = StdDynamicArray::<u32>::new();

    for i in 0..MAX {
        let no = i as u32;
        vec.push(no);
        arr.push(no).unwrap();
        let arr_len: usize = arr.len().into();
        assert_eq!(arr_len, vec.len());
        assert_eq!(arr.as_slice(), vec.as_slice());
    }
}

fn test_drop<const N: usize>() {
    const MAX: usize = 100;

    #[derive(Debug, Clone)]
    struct Foo {
        counter: Arc<AtomicUsize>,
    }

    impl Drop for Foo {
        fn drop(&mut self) {
            self.counter.fetch_add(1, Ordering::SeqCst);
        }
    }

    let counter = Arc::new(AtomicUsize::new(0));

    let mut arr = StdDynamicArray::<Foo>::new();

    for _ in 0..MAX {
        let foo = Foo {
            counter: counter.clone(),
        };
        arr.push(foo).unwrap();
    }

    assert_eq!(counter.load(Ordering::SeqCst), 0);

    drop(arr);

    assert_eq!(counter.load(Ordering::SeqCst), MAX);
}

#[test]
fn test_various_drops() {
    test_drop::<1>();
    test_drop::<5>();
    test_drop::<25>();
    test_drop::<100>();
    test_drop::<250>();
    test_drop::<1000>();
}

#[rstest]
#[case(&[1, 2])]
#[case(&[-1, 12, 0])]
#[case(&[-100, 2, -6, -7, 0, 12, 165, 111111])]
fn test_push_and_pop(#[case] data: &[i32]) {
    let first = data[0];
    let second = data[1];
    let mut dynamic_vec = StdDynamicArray::<i32>::new();
    for value in data {
        dynamic_vec.push(*value).unwrap();
    }

    assert_eq!(dynamic_vec.as_slice(), data);
    assert_eq!(dynamic_vec.deref(), data);

    for _ in 0..(data.len() - 2) {
        assert!(dynamic_vec.pop().is_some());
    }

    assert_eq!(dynamic_vec.as_slice(), &[first, second]);
    assert_eq!(dynamic_vec.pop().unwrap(), second);
    assert_eq!(dynamic_vec.as_slice(), &[first]);
    assert_eq!(dynamic_vec.pop().unwrap(), first);
    assert_eq!(dynamic_vec.as_slice(), &[]);
    assert!(dynamic_vec.pop().is_none());

    for _ in 0..3 {
        assert!(dynamic_vec.pop().is_none());
    }

    dynamic_vec.push(12345).unwrap();
    assert_eq!(dynamic_vec.as_slice(), &[12345]);
}

#[test]
fn test_shrink_to_fit() {
    let length = Length::try_from_i32(10).unwrap();
    let mut dynamic_array = StdDynamicArray::<i32>::with_capacity(length).unwrap();
    assert_eq!(dynamic_array.capacity(), length);
    assert_eq!(dynamic_array.len(), Length::ZERO);

    dynamic_array.push(1).unwrap();
    assert_eq!(dynamic_array.capacity(), length);
    assert_eq!(dynamic_array.len(), Length::try_from_i32(1).unwrap());

    dynamic_array.shrink_to_fit().unwrap();
    assert_eq!(dynamic_array.capacity(), Length::try_from_i32(1).unwrap());
    assert_eq!(dynamic_array.len(), Length::try_from_i32(1).unwrap());

    dynamic_array.push(2).unwrap();
    assert!(dynamic_array.capacity() >= Length::try_from_i32(2).unwrap());
    assert_eq!(dynamic_array.len(), Length::try_from_i32(2).unwrap());
}
