//! Holds the definition of results of various tree operations.
use osom_lib_primitives::KeyValuePair;

#[must_use]
pub enum TreeQueryExactResult<'a, TKey: 'a, TValue: 'a> {
    NotFound,
    Found { key: &'a TKey, value: &'a TValue },
}

#[must_use]
pub enum TreeQueryExactMutResult<'a, TKey: 'a, TValue: 'a> {
    NotFound,
    Found { key: &'a TKey, value: &'a mut TValue },
}

#[must_use]
pub trait TreeQueryResult<'a, TKey: 'a, TValue: 'a>:
    Iterator<Item = KeyValuePair<&'a TKey, &'a TValue>>
{ }

#[must_use]
pub trait TreeQueryMutResult<'a, TKey: 'a, TValue: 'a>:
    Iterator<Item = KeyValuePair<&'a TKey, &'a mut TValue>>
{ }
