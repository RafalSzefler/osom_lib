mod common;

use osom_lib_trees::bplus_tree::StdBPlusTree;

#[test]
fn test_bplus_tree_int_string_big_capacity() {
    let tree = StdBPlusTree::<i32, String, 64>::new();
    common::test_tree_int_string(|| tree);
}

#[test]
#[ignore = "not yet implemented"]
fn test_bplus_tree_int_string_small_capacity() {
    let tree = StdBPlusTree::<i32, String, 8>::new();
    common::test_tree_int_string(|| tree);
}
