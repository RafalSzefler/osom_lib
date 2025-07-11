use core::ops::RangeBounds;

use super::{
    Compare, Ordering, TreeQueryExactMutResult, TreeQueryExactResult, TreeQueryMutResult, TreeQueryResult,
    TreeTryInsertResult, TreeError,
};


/// Abstract trait for tree-like data structures.
pub trait Tree {
    type TKey: Ord;
    type TValue;

    fn try_insert(&mut self, key: Self::TKey, value: Self::TValue) -> Result<TreeTryInsertResult, TreeError>;

    /// Searches the tree for the exact match.
    fn query_exact<K>(&self, key: &K) -> TreeQueryExactResult<'_, Self::TKey, Self::TValue>
    where
        Self::TKey: Compare<K>;

    /// The mutable version of [`Tree::query_exact`].
    fn query_exact_mut<K>(&mut self, key: &K) -> TreeQueryExactMutResult<'_, Self::TKey, Self::TValue>
    where
        Self::TKey: Compare<K>;

    /// Searches the tree for the key-value pairs contained in the passed range.
    ///
    /// The resulting iterator will be in the order specified by `ordering`.
    fn query_range<K>(
        &self,
        range: impl RangeBounds<K>,
        ordering: Ordering,
    ) -> impl TreeQueryResult<'_, Self::TKey, Self::TValue>
    where
        Self::TKey: Compare<K>;

    /// The mutable version of [`Tree::query_range`].
    fn query_range_mut<K>(
        &mut self,
        range: impl RangeBounds<K>,
        ordering: Ordering,
    ) -> impl TreeQueryMutResult<'_, Self::TKey, Self::TValue>
    where
        Self::TKey: Compare<K>;
}
