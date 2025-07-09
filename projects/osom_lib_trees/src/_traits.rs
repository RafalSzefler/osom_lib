use core::borrow::Borrow;

use osom_lib_primitives::KeyValuePair;

use crate::op_results::{TreeInsertOrUpdateResult, TreeInsertWithResult};

use super::errors::TreeError;
use super::op_results::{TreeGetMutResult, TreeGetResult, TreeInsertResult};

/// Trait that checks whether `Self` is equivalent to `T`.
/// 
/// This has the default implementation for all types
/// implementing [`Borrow<T>`].
pub trait Cmp<T>
{
    fn is_less(&self, other: &T) -> bool;
    fn is_equal(&self, other: &T) -> bool;
    fn is_less_or_equal(&self, other: &T) -> bool {
        self.is_less(other) || self.is_equal(other)
    }
}

impl<T, K> Cmp<T> for K
where
    T: PartialEq + Eq + PartialOrd + Ord,
    K: Borrow<T>,
{
    fn is_less(&self, other: &T) -> bool {
        self.borrow() < other
    }

    fn is_equal(&self, other: &T) -> bool {
        self.borrow() == other
    }
}

/// Abstract trait for tree-like data structures.
pub trait Tree {
    type TKey: PartialEq + Eq + PartialOrd + Ord;
    type TValue;

    /// Inserts a key-value pair into the tree.
    /// 
    /// If the key already exists, it does nothing and returns
    /// passed key-value pair.
    /// 
    /// # Errors
    /// 
    /// For details see [`TreeError`].
    fn insert(&mut self, key: Self::TKey, value: Self::TValue)
        -> Result<TreeInsertResult<Self::TKey, Self::TValue>, TreeError>;

    /// Inserts a key-value pair into the tree. The value is generated
    /// only if the key does not exist.
    /// 
    /// If the key already exists, it does nothing and returns
    /// passed key only.
    /// 
    /// # Errors
    /// 
    /// For details see [`TreeError`].
    fn insert_with(&mut self, key: Self::TKey, value: impl FnOnce() -> Self::TValue)
        -> Result<TreeInsertWithResult<Self::TKey>, TreeError>;

    /// Inserts a key-value pair into the tree. Overwrites the existing value.
    /// 
    /// If the key already exists, the value is updated and the old value is returned.
    /// 
    /// # Errors
    /// 
    /// For details see [`TreeError`].
    fn insert_or_update_with(&mut self, key: Self::TKey, value: impl FnOnce() -> Self::TValue)
        -> Result<TreeInsertOrUpdateResult<Self::TKey, Self::TValue>, TreeError>;

    /// Retrieves key-value pair from the tree, matching passed key. If
    /// the value does not exist, it is generated and inserted into the tree.
    /// 
    /// # Errors
    /// 
    /// For details see [`TreeError`].
    fn get_or_insert_with(&mut self, key: Self::TKey, value: impl FnOnce() -> Self::TValue)
        -> Result<KeyValuePair<&Self::TKey, &Self::TValue>, TreeError>;

    /// The mutable version of [`Tree::get_or_insert_with`].
    fn get_or_insert_with_mut(&mut self, key: Self::TKey, value: impl FnOnce() -> Self::TValue)
        -> Result<KeyValuePair<&Self::TKey, &mut Self::TValue>, TreeError>;

    /// Retrieves key-value pair from the tree, matching passed key exactly.
    /// 
    /// # Errors
    /// 
    /// For details see [`TreeError`].
    fn get_exact<K>(&self, key: &K) -> TreeGetResult<'_, Self::TKey, Self::TValue>
    where
        Self::TKey: Cmp<K>;

    /// The mutable version of [`Tree::get_exact`].
    fn get_exact_mut<K>(&mut self, key: &K) -> TreeGetMutResult<'_, Self::TKey,  Self::TValue>
    where
        Self::TKey: Cmp<K>;

    
    /// Retrieves the key-value pair with the smallest key
    /// greater than the passed key. Note that it doesn't return
    /// exact match, even if the key exists.
    /// 
    /// It can return [`TreeGetResult::NotFound`] only if the passed key
    /// already is the greatest.
    /// 
    /// # Errors
    /// 
    /// For details see [`TreeError`].
    fn get_next<K>(&self, key: &K) -> TreeGetResult<'_, Self::TKey, Self::TValue>
    where
        Self::TKey: Cmp<K>;
    
    /// The mutable version of [`Tree::get_next`].
    fn get_next_mut<K>(&mut self, key: &K) -> TreeGetMutResult<'_, Self::TKey, Self::TValue>
    where
        Self::TKey: Cmp<K>;
    
    /// Retrieves the key-value pair with the greatest key
    /// less than the passed key. Note that it doesn't return
    /// exact match, even if the key exists.
    /// 
    /// It can return [`TreeGetResult::NotFound`] only if the passed key
    /// already is the smallest.
    /// 
    /// # Errors
    /// 
    /// For details see [`TreeError`].
    fn get_prev<K>(&self, key: &K) -> TreeGetResult<'_, Self::TKey, Self::TValue>
    where
        Self::TKey: Cmp<K>;
    
    /// The mutable version of [`Tree::get_prev`].
    fn get_prev_mut<K>(&mut self, key: &K) -> TreeGetMutResult<'_, Self::TKey, Self::TValue>
    where
        Self::TKey: Cmp<K>;

    // Default implementations

    /// Similar to [`Tree::insert_or_update_with`], but without a fixed value.
    /// 
    /// # Errors
    /// 
    /// For details see [`TreeError`].
    fn insert_or_update(&mut self, key: Self::TKey, value: Self::TValue)
        -> Result<TreeInsertOrUpdateResult<Self::TKey, Self::TValue>, TreeError> {
            self.insert_or_update_with(key, || value)
        }
    
    /// Similar to [`Tree::get_or_insert_with`], but without a fixed value.
    /// 
    /// # Errors
    /// 
    /// For details see [`TreeError`].
    fn get_or_insert(&mut self, key: Self::TKey, value: Self::TValue)
        -> Result<KeyValuePair<&Self::TKey, &Self::TValue>, TreeError>
    {
        self.get_or_insert_with(key, || value)
    }

    /// Similar to [`Tree::get_or_insert_with_mut`], but without a fixed value.
    /// 
    /// # Errors
    /// 
    /// For details see [`TreeError`].
    fn get_or_insert_mut(&mut self, key: Self::TKey, value: Self::TValue)
        -> Result<KeyValuePair<&Self::TKey, &mut Self::TValue>, TreeError>
    {
        self.get_or_insert_with_mut(key, || value)
    }
}
