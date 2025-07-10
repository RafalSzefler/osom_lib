#![allow(dead_code, unused_variables)]

use core::marker::PhantomData;

use osom_lib_primitives::KeyValuePair;

use crate::{
    bplus_tree::node::LeafRef,
    traits::{Ordering, TreeQueryExactMutResult, TreeQueryExactResult, TreeQueryMutResult, TreeQueryResult},
};

pub enum BPlusTreeQueryExactResult<TKey, TValue> {
    NotFound,
    Found { key: *const TKey, value: *mut TValue },
}

impl<TKey, TValue> From<BPlusTreeQueryExactResult<TKey, TValue>> for TreeQueryExactResult<'_, TKey, TValue> {
    fn from(result: BPlusTreeQueryExactResult<TKey, TValue>) -> Self {
        match result {
            BPlusTreeQueryExactResult::NotFound => TreeQueryExactResult::NotFound,
            BPlusTreeQueryExactResult::Found { key, value } => TreeQueryExactResult::Found {
                key: unsafe { &*key },
                value: unsafe { &mut *value },
            },
        }
    }
}

impl<TKey, TValue> From<BPlusTreeQueryExactResult<TKey, TValue>> for TreeQueryExactMutResult<'_, TKey, TValue> {
    fn from(result: BPlusTreeQueryExactResult<TKey, TValue>) -> Self {
        match result {
            BPlusTreeQueryExactResult::NotFound => TreeQueryExactMutResult::NotFound,
            BPlusTreeQueryExactResult::Found { key, value } => TreeQueryExactMutResult::Found {
                key: unsafe { &*key },
                value: unsafe { &mut *value },
            },
        }
    }
}

pub struct BPlusTreeQueryResult<const N: usize, TKey, TValue> {
    current_ref: LeafRef<N, TKey, TValue>,
    last_ref: LeafRef<N, TKey, TValue>,
    ordering: Ordering,
}

impl<const N: usize, TKey, TValue> BPlusTreeQueryResult<N, TKey, TValue> {
    #[inline(always)]
    pub const fn new(start: LeafRef<N, TKey, TValue>, last_ref: LeafRef<N, TKey, TValue>, ordering: Ordering) -> Self {
        Self {
            current_ref: start,
            last_ref: last_ref,
            ordering: ordering,
        }
    }

    pub fn next_ref(&mut self) -> LeafRef<N, TKey, TValue> {
        let current_ref = self.current_ref.clone();
        if current_ref.is_null() {
            return current_ref;
        }

        if current_ref.equals(&self.last_ref) {
            self.current_ref = LeafRef::null();
            return current_ref;
        }

        match self.ordering {
            Ordering::Unspecified | Ordering::Ascending => {
                self.current_ref = current_ref.next();
            }
            Ordering::Descending => {
                self.current_ref = current_ref.prev();
            }
        }

        current_ref
    }
}

pub struct BPlusTreeQueryResultIterator<'a, const N: usize, TKey: 'a, TValue: 'a> {
    result: BPlusTreeQueryResult<N, TKey, TValue>,
    phantom: PhantomData<(&'a TKey, &'a mut TValue)>,
}

impl<'a, const N: usize, TKey: 'a, TValue: 'a> From<BPlusTreeQueryResult<N, TKey, TValue>>
    for BPlusTreeQueryResultIterator<'a, N, TKey, TValue>
{
    fn from(result: BPlusTreeQueryResult<N, TKey, TValue>) -> Self {
        Self {
            result,
            phantom: PhantomData,
        }
    }
}

impl<'a, const N: usize, TKey: 'a, TValue: 'a> Iterator for BPlusTreeQueryResultIterator<'a, N, TKey, TValue> {
    type Item = KeyValuePair<&'a TKey, &'a TValue>;

    fn next(&mut self) -> Option<Self::Item> {
        let next_ref = self.result.next_ref();
        if next_ref.is_null() {
            return None;
        }

        let leaf = unsafe { &mut *next_ref.leaf };
        let key = unsafe { leaf.data.keys[next_ref.idx as usize].assume_init_ref() };
        let value = unsafe { leaf.values[next_ref.idx as usize].assume_init_mut() };

        Some(KeyValuePair::new(key, value))
    }
}

impl<'a, const N: usize, TKey: 'a, TValue: 'a> TreeQueryResult<'a, TKey, TValue>
    for BPlusTreeQueryResultIterator<'a, N, TKey, TValue>
{
}

pub struct BPlusTreeQueryMutResultIterator<'a, const N: usize, TKey: 'a, TValue: 'a> {
    result: BPlusTreeQueryResult<N, TKey, TValue>,
    phantom: PhantomData<(&'a TKey, &'a mut TValue)>,
}

impl<'a, const N: usize, TKey: 'a, TValue: 'a> From<BPlusTreeQueryResult<N, TKey, TValue>>
    for BPlusTreeQueryMutResultIterator<'a, N, TKey, TValue>
{
    fn from(result: BPlusTreeQueryResult<N, TKey, TValue>) -> Self {
        Self {
            result,
            phantom: PhantomData,
        }
    }
}

impl<'a, const N: usize, TKey: 'a, TValue: 'a> Iterator for BPlusTreeQueryMutResultIterator<'a, N, TKey, TValue> {
    type Item = KeyValuePair<&'a TKey, &'a mut TValue>;

    fn next(&mut self) -> Option<Self::Item> {
        let next_ref = self.result.next_ref();
        if next_ref.is_null() {
            return None;
        }

        let leaf = unsafe { &mut *next_ref.leaf };
        let key = unsafe { leaf.data.keys[next_ref.idx as usize].assume_init_ref() };
        let value = unsafe { leaf.values[next_ref.idx as usize].assume_init_mut() };

        Some(KeyValuePair::new(key, value))
    }
}

impl<'a, const N: usize, TKey: 'a, TValue: 'a> TreeQueryMutResult<'a, TKey, TValue>
    for BPlusTreeQueryMutResultIterator<'a, N, TKey, TValue>
{
}
