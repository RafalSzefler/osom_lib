use osom_lib_trees::traits::{Ordering, Tree, TreeQueryExactResult, TreeTryInsertResult};

pub fn test_tree_int_string<TTree: Tree<TKey = i32, TValue = String>, TTreeBuilder: FnOnce() -> TTree>(
    builder: TTreeBuilder,
) {
    let mut tree = builder();
    for i in -10..10 {
        match tree.try_insert(i, i.to_string() + "@V").unwrap() {
            TreeTryInsertResult::Inserted => {}
            TreeTryInsertResult::AlreadyExists { .. } => panic!("key {} already exists", i),
        };

        match tree.try_insert(i, i.to_string()).unwrap() {
            TreeTryInsertResult::Inserted => {
                panic!("key {} should already exist", i)
            }
            TreeTryInsertResult::AlreadyExists { .. } => {}
        };
    }

    let expect = |passed_key: i32, expected: &str| {
        let result = tree.query_exact(&passed_key);
        match result {
            TreeQueryExactResult::Found { key, value } => {
                assert_eq!(key, &passed_key);
                assert_eq!(value, expected);
            }
            TreeQueryExactResult::NotFound => panic!("key {} not found", passed_key),
        }
    };

    for i in -10..10 {
        expect(i, &(i.to_string() + "@V"));
    }

    let ascending_result = tree.query_range(5..7, Ordering::Ascending).collect::<Vec<_>>();
    assert_eq!(ascending_result.len(), 2);
    assert_eq!(**ascending_result[0].key(), 5);
    assert_eq!(*ascending_result[0].value(), "5@V");
    assert_eq!(**ascending_result[1].key(), 6);
    assert_eq!(*ascending_result[1].value(), "6@V");

    let descending_result = tree.query_range(-3..=-1, Ordering::Descending).collect::<Vec<_>>();
    assert_eq!(descending_result.len(), 3);
    assert_eq!(**descending_result[0].key(), -1);
    assert_eq!(*descending_result[0].value(), "-1@V");
    assert_eq!(**descending_result[1].key(), -2);
    assert_eq!(*descending_result[1].value(), "-2@V");
    assert_eq!(**descending_result[2].key(), -3);
    assert_eq!(*descending_result[2].value(), "-3@V");
}
