#![allow(unused)]

use std::{
    convert::Infallible,
    sync::{Arc, atomic::AtomicU32},
};

use osom_lib_hash_tables::traits::MutableHashTable;
use osom_lib_primitives::length::Length;

fn make_length(value: u32) -> Length {
    Length::try_from_u32(value).unwrap()
}

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
    assert_eq!(table.length(), make_length(2));
    assert!(table.insert(3, 2).is_none());
    assert_eq!(table.length(), make_length(3));

    assert!(table.insert(1, 2).is_some());
    assert_eq!(table.length(), make_length(3));
    assert!(table.insert(2, 2).is_some());
    assert_eq!(table.length(), make_length(3));
    assert!(table.insert(3, 2).is_some());
    assert_eq!(table.length(), make_length(3));

    for i in 1..=3 {
        let value = table.get_mut(&i).unwrap();
        *value = 5;
    }

    for i in 1..=3 {
        assert_eq!(table.get(&i).unwrap(), &5);
    }
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
        for kvp in table.iter() {
            keys.push(kvp.key.clone());
            sum |= kvp.value;
        }
        assert_eq!(sum, 0b1111);
        keys.sort();
        assert_eq!(keys, vec!["a", "b", "c", "d"]);
    }

    {
        for mut kvp in table.iter_mut() {
            *kvp.value <<= 1;
        }

        assert_eq!(table.get(&"a".to_string()).unwrap(), &0b00010);
        assert_eq!(table.get(&"b".to_string()).unwrap(), &0b00100);
        assert_eq!(table.get(&"c".to_string()).unwrap(), &0b01000);
        assert_eq!(table.get(&"d".to_string()).unwrap(), &0b10000);

        let mut sum = 0;
        for kvp in table.iter() {
            sum |= *kvp.value;
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

#[derive(PartialEq, Eq, Hash)]
pub struct StringWrapper(String);

impl TryClone for StringWrapper {
    type Error = Infallible;

    fn try_clone(&self) -> Result<Self, Self::Error> {
        Ok(Self(self.0.clone()))
    }
}

impl AsRef<String> for StringWrapper {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

impl From<&str> for StringWrapper {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

#[inline(never)]
pub fn test_hash_table_clone<T: MutableHashTable<i32, StringWrapper> + TryClone + PartialEq>(
    builder: impl FnOnce() -> T,
) {
    let mut table = builder();
    assert_eq!(table.length().as_u32(), 0);
    table.insert(5, "test".into());
    assert_eq!(table.length().as_u32(), 1);
    assert_eq!(table.get(&5).unwrap().as_ref(), "test");
    let mut clone = table.try_clone().unwrap();
    assert_eq!(clone.length().as_u32(), 1);
    if table != clone {
        panic!("clone not equal to table");
    }
    clone.insert(5, "foo".into());
    if table == clone {
        panic!("clone after modification still equal to table");
    }
    clone.insert(5, "test".into());
    if table != clone {
        panic!("clone not equal to table");
    }
    clone.insert(1, "test".into());
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
use osom_lib_try_clone::TryClone;
