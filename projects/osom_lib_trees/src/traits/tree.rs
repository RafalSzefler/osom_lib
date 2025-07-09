use core::ops::RangeBounds;

use super::{Compare, TreeQueryExactMutResult, TreeQueryExactResult, TreeQueryMutResult, TreeQueryResult};

/// Abstract trait for tree-like data structures.
pub trait Tree {
    type TKey: Ord;
    type TValue;

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
    /// # Notes
    /// 
    /// * If `range` has both lower and upper bounds, then the returned iterator will be in ascending order starting from the lower bound.
    /// * If `range` has only a lower bound, then the returned iterator will be in ascending order starting from the lower bound.
    /// * If `range` has only an upper bound, then the returned iterator will be in descending order starting from the upper bound.
    /// * If `range` has no bounds, then the returned iterator will be in ascending order starting from the smallest key in the tree.
    fn query_range<K>(
        &self,
        range: impl RangeBounds<K>,
    ) -> impl TreeQueryResult<'_, Self::TKey, Self::TValue>
    where
        Self::TKey: Compare<K>;
    
    /// The mutable version of [`Tree::query_range`].
    fn query_range_mut<K>(
        &mut self,
        range: impl RangeBounds<K>,
    ) -> impl TreeQueryMutResult<'_, Self::TKey, Self::TValue>
    where
        Self::TKey: Compare<K>;
}
