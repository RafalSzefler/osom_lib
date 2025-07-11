use std::{
    ops::Deref,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

use osom_lib_arrays::FixedArray;

use osom_lib_primitives::Length;
use rstest::rstest;

#[rstest]
#[case(&[])]
#[case(&[1, 2])]
#[case(&[-1, 12, 0])]
#[case(&[-100, 2, -6, -7, 0, 12, 165, 111111])]
fn test_push_and_compare(#[case] data: &[i32]) {
    let mut inlined_vec = FixedArray::<_, 15>::new();
    for value in data {
        inlined_vec.push(*value).unwrap();
    }

    assert_eq!(inlined_vec.as_slice(), data);
    assert_eq!(inlined_vec.deref(), data);

    let mut other = FixedArray::<_, 15>::new();
    for value in data {
        other.push(*value).unwrap();
    }

    assert_eq!(inlined_vec, other);
}

#[test]
fn test_incremental_push() {
    const MAX: usize = 100;
    let mut vec = Vec::<u32>::with_capacity(MAX);
    let mut arr = FixedArray::<_, 150>::new();

    for i in 0..MAX {
        let no = i as u32;
        vec.push(no);
        arr.push(no).unwrap();
        let arr_len: usize = arr.len().into();
        assert_eq!(arr_len, vec.len());
        assert_eq!(arr.as_slice(), vec.as_slice());
    }
}

#[test]
fn test_various_drops() {
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

    let mut arr = FixedArray::<_, MAX>::new();

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

#[rstest]
#[case(&[1, 2])]
#[case(&[-1, 12, 0])]
#[case(&[-100, 2, -6, -7, 0, 12, 165, 111111])]
fn test_push_and_pop(#[case] data: &[i32]) {
    let first = data[0];
    let second = data[1];
    let mut inlined_vec = FixedArray::<_, 15>::new();
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
    let mut arr = FixedArray::<_, 15>::new();
    arr.extend_from_array([12345; 10]).unwrap();
    assert_eq!(arr.as_slice(), &[12345; 10]);
    assert_eq!(arr.len(), Length::try_from_i32(10).unwrap());
}

#[test]
fn test_fill_2() {
    let mut arr = FixedArray::<_, 15>::new();
    arr.push(-1).unwrap();
    arr.push(-2).unwrap();
    arr.extend_from_array([7; 8]).unwrap();
    assert_eq!(arr.as_slice(), &[-1, -2, 7, 7, 7, 7, 7, 7, 7, 7]);
    assert_eq!(arr.len(), Length::try_from_i32(10).unwrap());
}

#[test]
fn test_out_of_range() {
    let mut arr = FixedArray::<_, 3>::new();
    arr.push(1).unwrap();
    arr.push(2).unwrap();
    arr.push(3).unwrap();
    assert!(arr.push(4).is_err());
    assert_eq!(arr.pop().unwrap(), 3);
    assert_eq!(arr.pop().unwrap(), 2);
    assert_eq!(arr.pop().unwrap(), 1);
    assert!(arr.pop().is_none());
}

#[rstest]
#[case(&[])]
#[case(&[5])]
#[case(&[5, 17])]
#[case(&[5, 17, 3, -10])]
#[case(&[1, -1, 2, -2, 3, -3, 4, -4])]
#[case(&[1, -1, 2, -2, 3, -3, 4, -4, 5, 5])]
fn test_clone(#[case] input: &[i32]) {
    let mut arr = FixedArray::<_, 15>::new();
    arr.extend_from_slice(input).unwrap();
    let arr2 = arr.clone();
    assert_eq!(arr, arr2);
    assert_eq!(arr.as_slice(), arr2.as_slice());
    assert_eq!(arr.len(), arr2.len());
    assert_eq!(arr.is_full(), arr2.is_full());
    assert_eq!(arr.is_empty(), arr2.is_empty());
}
