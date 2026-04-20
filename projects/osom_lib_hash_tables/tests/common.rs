#![allow(unused)]

use std::sync::{Arc, atomic::AtomicU32};

use osom_lib_hash_tables::traits::MutableHashTable;
use osom_lib_primitives::{length::Length, macros::make_length};

#[inline(never)]
pub fn test_hash_table_biggg<T: MutableHashTable<i32, u64>>(builder: impl FnOnce() -> T) {
    let mut table = builder();
    assert_eq!(table.length(), Length::ZERO);

    const A: u128 = 0xdb36357734e34abb0050d0761fcdfc15;
    const C: u128 = 0x86e9;
    const ITERATIONS: usize = 50001;
    let mut state = 0u128;

    for idx in 0..ITERATIONS as i32 {
        state = state.wrapping_mul(A).wrapping_add(C);
        let value = state as u32 as i32;
        let result = table.insert(idx, value as u64);
        assert!(result.is_none());
    }

    let table_len = table.length().as_usize();
    assert_eq!(table_len, ITERATIONS);
}

#[inline(never)]
pub fn test_hash_table_simple<T: MutableHashTable<i32, u8>>(builder: impl FnOnce() -> T) {
    let mut table = builder();
    assert_eq!(table.length(), Length::ZERO);
    assert!(table.get(&1).is_none());
    assert!(table.insert(1, 0).is_none());
    assert_eq!(table.length(), Length::ONE);
    assert_eq!(table.get(&1).unwrap(), &0);
    assert!(table.insert(1, 2).is_some());
    assert_eq!(table.length(), Length::ONE);
    assert_eq!(table.get(&1).unwrap(), &2);
    assert!(table.remove(&1).is_some());
    assert_eq!(table.length(), Length::ZERO);
    assert!(table.get(&1).is_none());

    assert!(table.insert(1, 2).is_none());
    assert_eq!(table.length(), Length::ONE);
    assert!(table.insert(2, 2).is_none());
    assert_eq!(table.length(), make_length!(2));
    assert!(table.insert(3, 2).is_none());
    assert_eq!(table.length(), make_length!(3));

    assert!(table.insert(1, 2).is_some());
    assert_eq!(table.length(), make_length!(3));
    assert!(table.insert(2, 2).is_some());
    assert_eq!(table.length(), make_length!(3));
    assert!(table.insert(3, 2).is_some());
    assert_eq!(table.length(), make_length!(3));
}

#[inline(never)]
pub fn test_hash_table_iter<T: MutableHashTable<String, u32>>(builder: impl FnOnce() -> T) {
    let mut table = builder();
    table.insert("d".to_string(), 0b1000);
    table.insert("c".to_string(), 0b0100);
    table.insert("b".to_string(), 0b0010);
    table.insert("a".to_string(), 0b0001);

    assert_eq!(table.get(&"a".to_string()).unwrap(), &0b0001);
    assert_eq!(table.get(&"b".to_string()).unwrap(), &0b0010);
    assert_eq!(table.get(&"c".to_string()).unwrap(), &0b0100);
    assert_eq!(table.get(&"d".to_string()).unwrap(), &0b1000);

    {
        let mut keys = Vec::new();
        let mut sum = 0;
        for (key, value) in table.iter() {
            keys.push(key.clone());
            sum |= value;
        }
        assert_eq!(sum, 0b1111);
        keys.sort();
        assert_eq!(keys, vec!["a", "b", "c", "d"]);
    }

    {
        for (_, value) in table.iter_mut() {
            *value <<= 1;
        }

        assert_eq!(table.get(&"a".to_string()).unwrap(), &0b00010);
        assert_eq!(table.get(&"b".to_string()).unwrap(), &0b00100);
        assert_eq!(table.get(&"c".to_string()).unwrap(), &0b01000);
        assert_eq!(table.get(&"d".to_string()).unwrap(), &0b10000);

        let mut sum = 0;
        for (_, value) in table.iter() {
            sum |= value;
        }
        assert_eq!(sum, 0b11110);
    }
}

static ID: AtomicU32 = AtomicU32::new(0);

#[repr(C)]
pub struct TestStruct {
    id: u32,
    counter: Arc<AtomicU32>,
}

impl TestStruct {
    pub fn new(counter: Arc<AtomicU32>) -> Self {
        counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let id = ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        Self { id, counter }
    }
}

impl Drop for TestStruct {
    fn drop(&mut self) {
        self.counter.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
    }
}

impl core::fmt::Debug for TestStruct {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "TS({})", self.id)
    }
}

#[inline(never)]
pub fn test_hash_table_ownership<T: MutableHashTable<i32, TestStruct>>(builder: impl FnOnce() -> T) {
    let counter = Arc::new(AtomicU32::new(0));

    macro_rules! load {
        () => {
            counter.load(std::sync::atomic::Ordering::SeqCst)
        };
    }

    let mut table = builder();

    assert_eq!(load!(), 0);

    assert!(table.insert(1, TestStruct::new(counter.clone())).is_none());
    assert!(table.insert(2, TestStruct::new(counter.clone())).is_none());
    assert!(table.insert(3, TestStruct::new(counter.clone())).is_none());

    assert_eq!(load!(), 3);

    table.remove(&1);

    assert_eq!(load!(), 2);

    assert!(table.insert(2, TestStruct::new(counter.clone())).is_some());

    assert_eq!(load!(), 2);

    assert!(table.insert(5, TestStruct::new(counter.clone())).is_none());

    assert_eq!(load!(), 3);

    assert!(table.insert(1, TestStruct::new(counter.clone())).is_none());
    assert!(table.insert(2, TestStruct::new(counter.clone())).is_some());
    assert!(table.insert(3, TestStruct::new(counter.clone())).is_some());

    assert_eq!(load!(), 4);

    assert!(table.insert(1, TestStruct::new(counter.clone())).is_some());
    assert!(table.insert(2, TestStruct::new(counter.clone())).is_some());
    assert!(table.insert(37, TestStruct::new(counter.clone())).is_none());

    assert_eq!(load!(), 5);

    let range = 17u32;
    for idx in 0..range {
        assert!(
            table
                .insert(idx as i32 + 137, TestStruct::new(counter.clone()))
                .is_none()
        );
        assert!(
            table
                .insert(idx as i32 + 137, TestStruct::new(counter.clone()))
                .is_some()
        );
    }

    assert_eq!(load!(), 5 + range);

    table.remove(&137);

    assert_eq!(load!(), 4 + range);

    drop(table);

    assert_eq!(load!(), 0);
}

#[inline(never)]
pub fn test_hash_table_clone<T: MutableHashTable<i32, String> + Clone + PartialEq>(builder: impl FnOnce() -> T) {
    let mut table = builder();
    assert_eq!(table.length().as_u32(), 0);
    table.insert(5, "test".to_string());
    assert_eq!(table.length().as_u32(), 1);
    assert_eq!(table.get(&5).unwrap(), "test");
    let mut clone = table.clone();
    assert_eq!(clone.length().as_u32(), 1);
    if table != clone {
        panic!("clone not equal to table");
    }
    clone.insert(5, "foo".to_string());
    if table == clone {
        panic!("clone after modification still equal to table");
    }
    clone.insert(5, "test".to_string());
    if table != clone {
        panic!("clone not equal to table");
    }
    clone.insert(1, "test".to_string());
    if table == clone {
        panic!("clone after modification still equal to table");
    }
}

#[allow(unused_macros)]
macro_rules! build_tests {
    ( $array_name: ident ) => {
        paste::paste! {
            #[allow(non_snake_case)]
            #[test]
            fn [< test_ $array_name _simple >]() {
                crate::common::test_hash_table_simple($array_name::default);
            }

            #[allow(non_snake_case)]
            #[test]
            fn [< test_ $array_name _iter >]() {
                crate::common::test_hash_table_iter($array_name::default);
            }

            #[allow(non_snake_case)]
            #[test]
            fn [< test_ $array_name _ownership >]() {
                crate::common::test_hash_table_ownership($array_name::default);
            }

            #[allow(non_snake_case)]
            #[test]
            fn [< test_ $array_name _clone >]() {
                crate::common::test_hash_table_clone($array_name::default);
            }

            #[cfg(not(miri))]
            #[allow(non_snake_case)]
            #[test]
            fn [< test_ $array_name _biggg >]() {
                crate::common::test_hash_table_biggg($array_name::default);
            }
        }
    };
}

#[allow(unused_imports)]
pub(crate) use build_tests;
