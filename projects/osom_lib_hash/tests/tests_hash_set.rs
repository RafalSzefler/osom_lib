#![cfg(feature = "std_alloc")]
// use osom_lib_hash::hash_set::HashSet;
// use osom_lib_hash::hash_set::operation_results::{TryInsertResult, TryRemoveResult};

// #[test]
// fn test_hash_set_insert_and_remove() {
//     let mut hash_set = HashSet::<8, _>::new();
//     assert!(matches!(hash_set.try_insert(1), Ok(TryInsertResult::Inserted)));
//     assert!(matches!(hash_set.try_insert(2), Ok(TryInsertResult::Inserted)));
//     assert!(matches!(hash_set.try_insert(1), Ok(TryInsertResult::AlreadyExists(_))));
//     assert!(matches!(hash_set.try_remove(&1), Ok(TryRemoveResult::Removed(_))));
//     assert!(matches!(hash_set.try_remove(&1), Ok(TryRemoveResult::NotFound)));
// }

// #[test]
// fn test_hash_set_resizing() {
//     let mut hash_set = HashSet::<8, _>::new();
//     let mut capacities = std::collections::HashSet::<i32>::new();
//     for i in 0..100 {
//         assert!(matches!(hash_set.try_insert(i), Ok(TryInsertResult::Inserted)));
//         assert!(matches!(hash_set.try_insert(i), Ok(TryInsertResult::AlreadyExists(_))));
//         capacities.insert(hash_set.capacity().value());
//     }

//     println!("capacities: {:?}", capacities);
//     assert!(capacities.len() > 1);

//     let mut capacities = std::collections::HashSet::<i32>::new();
//     for i in 0..100 {
//         assert!(matches!(hash_set.try_remove(&i), Ok(TryRemoveResult::Removed(_))));
//         capacities.insert(hash_set.capacity().value());
//     }
//     assert!(capacities.len() > 1);
// }
