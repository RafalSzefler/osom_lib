#![cfg(feature = "std_alloc")]

use std::{
    ops::Deref,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

use osom_lib_arrays::InlineDynamicArray;

use osom_lib_primitives::Length;
use rstest::rstest;

#[rstest]
#[case(&[])]
#[case(&[1, 2])]
#[case(&[-1, 12, 0])]
#[case(&[-100, 2, -6, -7, 0, 12, 165, 111111])]
fn test_push_and_compare(#[case] data: &[i32]) {
    let mut inlined_vec = InlineDynamicArray::<5, _>::new();
    for value in data {
        inlined_vec.push(*value).unwrap();
    }

    assert_eq!(inlined_vec.as_slice(), data);
    assert_eq!(inlined_vec.deref(), data);

    let mut other = InlineDynamicArray::<15, _>::new();
    for value in data {
        other.push(*value).unwrap();
    }

    assert_eq!(inlined_vec, other);
}

#[test]
fn test_incremental_push() {
    const MAX: usize = 100;
    let mut vec = Vec::<u32>::with_capacity(MAX);
    let mut arr = InlineDynamicArray::<5, u32>::new();

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

    let mut arr = InlineDynamicArray::<N, Foo>::new();

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
    let mut inlined_vec = InlineDynamicArray::<5, _>::new();
    for value in data {
        inlined_vec.push(*value).unwrap();
    }

    assert_eq!(inlined_vec.as_slice(), data);
    assert_eq!(inlined_vec.deref(), data);

    for _ in 0..(data.len() - 2) {
        assert!(inlined_vec.pop().is_some());
    }

    assert_eq!(inlined_vec.as_slice(), &[first, second]);
    assert_eq!(inlined_vec.pop().unwrap(), second);
    assert_eq!(inlined_vec.as_slice(), &[first]);
    assert_eq!(inlined_vec.pop().unwrap(), first);
    assert_eq!(inlined_vec.as_slice(), &[]);
    assert!(inlined_vec.pop().is_none());

    for _ in 0..3 {
        assert!(inlined_vec.pop().is_none());
    }

    inlined_vec.push(12345).unwrap();
    assert_eq!(inlined_vec.as_slice(), &[12345]);
}

#[test]
fn test_fill() {
    let mut arr = InlineDynamicArray::<10, i32>::new();
    let result = arr.fill(|| 12345);
    assert_eq!(arr.as_slice(), &[12345; 10]);
    assert_eq!(*result, Length::try_from_i32(10).unwrap());
}

#[test]
fn test_fill_2() {
    let mut arr = InlineDynamicArray::<10, i32>::new();
    arr.push(-1).unwrap();
    arr.push(-2).unwrap();
    let result = arr.fill(|| 7);
    assert_eq!(arr.as_slice(), &[-1, -2, 7, 7, 7, 7, 7, 7, 7, 7]);
    assert_eq!(*result, Length::try_from_i32(8).unwrap());
}
