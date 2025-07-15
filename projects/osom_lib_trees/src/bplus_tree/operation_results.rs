use core::marker::PhantomData;

use osom_lib_primitives::KeyValuePair;

use crate::{
    bplus_tree::nodes::{LeafItem, LeafItemRange},
    traits::{Ordering, TreeQueryMutResult, TreeQueryResult},
};

struct InternalTreeQueryResult<const N: usize, TKey, TValue> {
    leaf_item_current: LeafItem<N, TKey, TValue>,
    leaf_item_end: LeafItem<N, TKey, TValue>,
    ordering: Ordering,
}

impl<const N: usize, TKey, TValue> InternalTreeQueryResult<N, TKey, TValue> {
    pub fn new(leaf_item_range: LeafItemRange<N, TKey, TValue>, ordering: Ordering) -> Self {
        if leaf_item_range.is_null() {
            return Self {
                leaf_item_current: LeafItem::null(),
                leaf_item_end: LeafItem::null(),
                ordering,
            };
        }

        match ordering {
            Ordering::Unspecified | Ordering::Ascending => Self {
                leaf_item_current: leaf_item_range.start,
                leaf_item_end: leaf_item_range.end,
                ordering,
            },
            Ordering::Descending => Self {
                leaf_item_current: leaf_item_range.end,
                leaf_item_end: leaf_item_range.start,
                ordering,
            },
        }
    }
}

impl<const N: usize, TKey, TValue> Iterator for InternalTreeQueryResult<N, TKey, TValue> {
    type Item = (*const TKey, *mut TValue);

    fn next(&mut self) -> Option<Self::Item> {
        if self.leaf_item_current.is_null() {
            return None;
        }

        let current = self.leaf_item_current.clone();

        #[allow(clippy::collapsible_else_if)]
        let new_current = if self.ordering == Ordering::Descending {
            if current.is_equal(&self.leaf_item_end) {
                LeafItem::null()
            } else {
                current.prev()
            }
        } else {
            if current.is_equal(&self.leaf_item_end) {
                LeafItem::null()
            } else {
                current.next()
            }
        };

        self.leaf_item_current = new_current;
        let key = unsafe { current.key_ptr() };
        let value = unsafe { current.value_ptr() };
        Some((key, value))
    }
}

#[repr(transparent)]
pub struct BPlusTreeQueryResult<'a, const N: usize, TKey: 'a, TValue: 'a> {
    internal_result: InternalTreeQueryResult<N, TKey, TValue>,
    phantom: PhantomData<(&'a TKey, &'a TValue)>,
}

impl<'a, const N: usize, TKey: 'a, TValue: 'a> BPlusTreeQueryResult<'a, N, TKey, TValue> {
    pub fn new(leaf_item_range: LeafItemRange<N, TKey, TValue>, ordering: Ordering) -> Self {
        Self {
            internal_result: InternalTreeQueryResult::new(leaf_item_range, ordering),
            phantom: PhantomData,
        }
    }
}

impl<'a, const N: usize, TKey: 'a, TValue: 'a> Iterator for BPlusTreeQueryResult<'a, N, TKey, TValue> {
    type Item = KeyValuePair<&'a TKey, &'a TValue>;

    fn next(&mut self) -> Option<Self::Item> {
        let (key, value) = self.internal_result.next()?;
        Some(KeyValuePair::new(unsafe { &*key }, unsafe { &*value }))
    }
}

impl<'a, const N: usize, TKey: 'a, TValue: 'a> TreeQueryResult<'a, TKey, TValue>
    for BPlusTreeQueryResult<'a, N, TKey, TValue>
{
}

#[repr(transparent)]
pub struct BPlusTreeQueryMutResult<'a, const N: usize, TKey: 'a, TValue: 'a> {
    internal_result: InternalTreeQueryResult<N, TKey, TValue>,
    phantom: PhantomData<(&'a TKey, &'a TValue)>,
}

impl<'a, const N: usize, TKey: 'a, TValue: 'a> BPlusTreeQueryMutResult<'a, N, TKey, TValue> {
    pub fn new(leaf_item_range: LeafItemRange<N, TKey, TValue>, ordering: Ordering) -> Self {
        Self {
            internal_result: InternalTreeQueryResult::new(leaf_item_range, ordering),
            phantom: PhantomData,
        }
    }
}

impl<'a, const N: usize, TKey: 'a, TValue: 'a> Iterator for BPlusTreeQueryMutResult<'a, N, TKey, TValue> {
    type Item = KeyValuePair<&'a TKey, &'a mut TValue>;

    fn next(&mut self) -> Option<Self::Item> {
        let (key, value) = self.internal_result.next()?;
        Some(KeyValuePair::new(unsafe { &*key }, unsafe { &mut *value }))
    }
}

impl<'a, const N: usize, TKey: 'a, TValue: 'a> TreeQueryMutResult<'a, TKey, TValue>
    for BPlusTreeQueryMutResult<'a, N, TKey, TValue>
{
}
