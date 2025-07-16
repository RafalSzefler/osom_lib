mod common;

use osom_lib_alloc::StdAllocator;
use osom_lib_trees::bplus_tree::BPlusTree;

#[test]
fn test_bplus_tree_int_string_big_capacity() {
    let tree = BPlusTree::<i32, String, StdAllocator, 64>::new();
    common::test_tree_int_string(|| tree);
}

#[test]
fn test_bplus_tree_int_string_small_capacity() {
    let tree = BPlusTree::<i32, String, StdAllocator, 8>::new();
    common::test_tree_int_string(|| tree);
}
