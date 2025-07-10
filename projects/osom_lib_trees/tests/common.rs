use osom_lib_trees::traits::{Tree, TreeQueryExactResult, TreeTryInsertResult};

pub fn test_tree_int_string<TTree: Tree<TKey = i32, TValue = String>, TTreeBuilder: FnOnce() -> TTree>(
    builder: TTreeBuilder,
) {
    let mut tree = builder();
    for i in -10..10 {
        match tree.try_insert(i, i.to_string()) {
            TreeTryInsertResult::Inserted => {}
            TreeTryInsertResult::AlreadyExists { .. } => panic!("key {} already exists", i),
        };

        match tree.try_insert(i, i.to_string()) {
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
        expect(i, i.to_string().as_str());
    }
}
