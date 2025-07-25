use core::ptr::NonNull;

use osom_lib_alloc::Allocator;

use crate::{bplus_tree::nodes::NodeTaggedPtr, traits::Compare};

#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
pub fn scan_node<TKey, K>(key: &K, array: impl AsRef<[TKey]>) -> i32
where
    TKey: Compare<K>,
{
    let array = array.as_ref();
    let (start, end) = {
        let middle = array.len() / 2;
        if array[middle].is_less_or_equal(key) {
            (middle, array.len() - 1)
        } else {
            (0, middle - 1)
        }
    };

    let mut idx = start;
    while idx <= end {
        if array[idx].is_greater_or_equal(key) {
            return idx as i32;
        }
        idx += 1;
    }

    idx as i32
}

pub fn deallocate_recursive<TKey, TValue, TAllocator, const NODE_CAPACITY: usize>(
    node_tagged_ptr: &mut NodeTaggedPtr<NODE_CAPACITY, TKey, TValue>,
    allocator: &mut TAllocator,
) where
    TAllocator: Allocator,
{
    if node_tagged_ptr.is_null() {
        return;
    }

    if node_tagged_ptr.is_leaf() {
        let leaf = unsafe { node_tagged_ptr.as_leaf_mut() };
        unsafe { core::ptr::drop_in_place(leaf) };
        let ptr = unsafe { NonNull::new_unchecked(core::ptr::from_mut(leaf)) };
        unsafe { allocator.deallocate_for_type(ptr) };
    } else {
        let internal = unsafe { node_tagged_ptr.as_internal_mut() };
        for edge in internal.edges_mut().as_mut_slice() {
            deallocate_recursive(edge, allocator);
        }
        unsafe { core::ptr::drop_in_place(internal) };
        let ptr = unsafe { NonNull::new_unchecked(core::ptr::from_mut(internal)) };
        unsafe { allocator.deallocate_for_type(ptr) };
    }
}

pub fn move_last_into_order<T>(arr: &mut [T]) -> usize
where
    T: Ord,
{
    let len = arr.len();
    if len <= 1 {
        return 0;
    }

    let insert_idx = {
        let (left, right) = arr.split_at_mut(len - 1);
        let last = &right[0];
        let mut insert_idx = 0;
        for item in left {
            if last <= item {
                break;
            }
            insert_idx += 1;
        }
        insert_idx
    };

    move_last_into_position(arr, insert_idx);
    insert_idx
}

pub fn move_last_into_position<T>(arr: &mut [T], position: usize) {
    let len = arr.len();
    if len <= 1 || position == len - 1 {
        return;
    }

    let raw_ptr = arr.as_mut_ptr();
    unsafe {
        let data = core::ptr::read(raw_ptr.add(len - 1));
        raw_ptr
            .add(position + 1)
            .copy_from(raw_ptr.add(position), len - position - 1);
        raw_ptr.add(position).write(data);
    }
}
