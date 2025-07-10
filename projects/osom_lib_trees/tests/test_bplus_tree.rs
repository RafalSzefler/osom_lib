mod common;

use osom_lib_trees::bplus_tree::BPlusTree;

#[test]
fn test_bplus_tree_int_string() {
    let tree = BPlusTree::<i32, String>::new().unwrap();
    common::test_tree_int_string(|| tree);
}
