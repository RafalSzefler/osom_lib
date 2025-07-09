use core::num::NonZeroU8;

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

    /// Searches the tree for the key-value pairs less than the passed key.
    /// These will be returned in descending order starting from the gratest found element.
    ///
    /// If `max_count` is `None`, all key-value pairs less than the passed key will be returned.
    /// If `max_count` is `Some(n)`, at most `n` key-value pairs will be returned.
    fn query_less_than<K>(
        &self,
        key: &K,
        max_count: Option<NonZeroU8>,
    ) -> impl TreeQueryResult<'_, Self::TKey, Self::TValue>
    where
        Self::TKey: Compare<K>;

    /// The mutable version of [`Tree::query_less_than`].
    fn query_less_than_mut<K>(
        &mut self,
        key: &K,
        max_count: Option<NonZeroU8>,
    ) -> impl TreeQueryMutResult<'_, Self::TKey, Self::TValue>
    where
        Self::TKey: Compare<K>;

    /// Searches the tree for the key-value pairs less than or equal to the passed key.
    /// These will be returned in descending order starting from the gratest found element.
    ///
    /// If `max_count` is `None`, all key-value pairs less than or equal to the passed key will be returned.
    /// If `max_count` is `Some(n)`, at most `n` key-value pairs will be returned.
    fn query_less_than_or_equal<K>(
        &self,
        key: &K,
        max_count: Option<NonZeroU8>,
    ) -> impl TreeQueryResult<'_, Self::TKey, Self::TValue>
    where
        Self::TKey: Compare<K>;

    /// The mutable version of [`Tree::query_less_than_or_equal`].
    fn query_less_than_or_equal_mut<K>(
        &mut self,
        key: &K,
        max_count: Option<NonZeroU8>,
    ) -> impl TreeQueryMutResult<'_, Self::TKey, Self::TValue>
    where
        Self::TKey: Compare<K>;

    /// Searches the tree for the key-value pairs greater than the passed key.
    /// These will be returned in ascending order starting from the smallest found element.
    ///
    /// If `max_count` is `None`, all key-value pairs greater than the passed key will be returned.
    /// If `max_count` is `Some(n)`, at most `n` key-value pairs will be returned.
    fn query_greater_than<K>(
        &self,
        key: &K,
        max_count: Option<NonZeroU8>,
    ) -> impl TreeQueryResult<'_, Self::TKey, Self::TValue>
    where
        Self::TKey: Compare<K>;

    /// The mutable version of [`Tree::query_greater_than`].
    fn query_greater_than_mut<K>(
        &mut self,
        key: &K,
        max_count: Option<NonZeroU8>,
    ) -> impl TreeQueryMutResult<'_, Self::TKey, Self::TValue>
    where
        Self::TKey: Compare<K>;

    /// Searches the tree for the key-value pairs greater than or equal to the passed key.
    /// These will be returned in ascending order starting from the smallest found element.
    ///
    /// If `max_count` is `None`, all key-value pairs greater than or equal to the passed key will be returned.
    /// If `max_count` is `Some(n)`, at most `n` key-value pairs will be returned.
    fn query_greater_than_or_equal<K>(
        &self,
        key: &K,
        max_count: Option<NonZeroU8>,
    ) -> impl TreeQueryResult<'_, Self::TKey, Self::TValue>
    where
        Self::TKey: Compare<K>;

    /// The mutable version of [`Tree::query_greater_than_or_equal`].
    fn query_greater_than_or_equal_mut<K>(
        &mut self,
        key: &K,
        max_count: Option<NonZeroU8>,
    ) -> impl TreeQueryMutResult<'_, Self::TKey, Self::TValue>
    where
        Self::TKey: Compare<K>;

    /// Searches the tree for the key-value pairs in the range `(left, right)`,
    /// i.e. with endpoints excluded.
    fn query_range_exclusive<K>(&self, left: &K, right: &K) -> impl TreeQueryResult<'_, Self::TKey, Self::TValue>
    where
        Self::TKey: Compare<K>;

    /// The mutable version of [`Tree::query_range_exclusive`].
    fn query_range_exclusive_mut<K>(
        &mut self,
        left: &K,
        right: &K,
    ) -> impl TreeQueryMutResult<'_, Self::TKey, Self::TValue>
    where
        Self::TKey: Compare<K>;

    /// Searches the tree for the key-value pairs in the range `[left, right]`,
    /// i.e. with endpoints included.
    fn query_range_inclusive<K>(&self, left: &K, right: &K) -> impl TreeQueryResult<'_, Self::TKey, Self::TValue>
    where
        Self::TKey: Compare<K>;

    /// The mutable version of [`Tree::query_range_inclusive`].
    fn query_range_inclusive_mut<K>(
        &mut self,
        left: &K,
        right: &K,
    ) -> impl TreeQueryMutResult<'_, Self::TKey, Self::TValue>
    where
        Self::TKey: Compare<K>;

    /// Searches the tree for the key-value pairs in the range `[left, right)`,
    /// i.e. with the left endpoint included and the right endpoint excluded.
    fn query_range_left_inclusive<K>(&self, left: &K, right: &K) -> impl TreeQueryResult<'_, Self::TKey, Self::TValue>
    where
        Self::TKey: Compare<K>;

    /// The mutable version of [`Tree::query_range_left_inclusive`].
    fn query_range_left_inclusive_mut<K>(
        &mut self,
        left: &K,
        right: &K,
    ) -> impl TreeQueryMutResult<'_, Self::TKey, Self::TValue>
    where
        Self::TKey: Compare<K>;

    /// Searches the tree for the key-value pairs in the range `(left, right]`,
    /// i.e. with the right endpoint included and the left endpoint excluded.
    fn query_range_right_inclusive<K>(&self, left: &K, right: &K) -> impl TreeQueryResult<'_, Self::TKey, Self::TValue>
    where
        Self::TKey: Compare<K>;

    /// The mutable version of [`Tree::query_range_right_inclusive`].
    fn query_range_right_inclusive_mut<K>(
        &mut self,
        left: &K,
        right: &K,
    ) -> impl TreeQueryMutResult<'_, Self::TKey, Self::TValue>
    where
        Self::TKey: Compare<K>;
}
