use core::{borrow::Borrow, convert::Infallible, fmt::Write};

use osom_lib_alloc::traits::Allocator;
use osom_lib_btree::{btree::BTree, defaults::DefaultBTreeConfig};
use osom_lib_primitives::{kvp::KVP, length::Length};
use osom_lib_reprc::traits::ReprC;
use osom_lib_try_clone::TryClone;

use crate::errors::{CVRObjectInsertError, TryCloneCVRError};

use super::{CVR, CVRString};

/// Represents a key-value object with [`CVRString`] keys and [`CVR`] values.
/// This object is deterministic, meaning internally stores keys ordered.
///
/// Internally it is a [`BTree`] of [`CVRString`] keys and [`CVR`] values.
#[repr(transparent)]
#[derive(Debug)]
#[must_use]
pub struct CVRObject<TAllocator: Allocator> {
    value: BTree<CVRString<TAllocator>, CVR<TAllocator>, DefaultBTreeConfig<TAllocator>>,
}

impl<TAllocator: Allocator> PartialEq for CVRObject<TAllocator> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<TAllocator: Allocator> Eq for CVRObject<TAllocator> {}

impl<TAllocator: Allocator> core::hash::Hash for CVRObject<TAllocator> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

unsafe impl<TAllocator: Allocator> ReprC for CVRObject<TAllocator> {
    const CHECK: () = const {
        osom_lib_reprc::hidden::is_reprc::<CVRString<TAllocator>>();
        osom_lib_reprc::hidden::is_reprc::<CVR<TAllocator>>();
    };
}

impl<TAllocator: Allocator> CVRObject<TAllocator> {
    /// Creates a new empty [`CVRObject`] instance.
    #[inline]
    pub fn new() -> Self
    where
        TAllocator: Default,
    {
        Self { value: BTree::new() }
    }

    /// Creates a new [`CVRObject`] instance with the given allocator.
    #[inline]
    pub fn with_allocator(allocator: TAllocator) -> Self {
        Self {
            value: BTree::with_config(DefaultBTreeConfig::with_allocator(allocator)),
        }
    }

    /// Returns `true` if the object is empty, `false` otherwise.
    #[inline(always)]
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len().as_u32() == 0
    }

    /// Returns the number of key-value pairs in the [`CVRObject`].
    #[inline(always)]
    pub const fn len(&self) -> Length {
        self.value.len()
    }

    /// Inserts a `(key, value)` pair into the object.
    ///
    /// Returns `None` if the key was not already present, or the previous value if it was.
    ///
    /// # Errors
    ///
    /// See [`CVRObjectInsertError`] for details.
    #[inline]
    pub fn try_insert(
        &mut self,
        key: CVRString<TAllocator>,
        value: CVR<TAllocator>,
    ) -> Result<Option<CVR<TAllocator>>, CVRObjectInsertError> {
        let value = self.value.try_insert(key, value)?;
        Ok(value)
    }

    /// Inserts a `(key, adder())` pair into the object if it doesn't exist,
    /// or updates the value with `updater(value)` call.
    ///
    /// Note: the method guarantees that either `adder` or `updater` will be called, but not both.
    ///
    /// # Errors
    ///
    /// See [`CVRObjectInsertError`] for details.
    #[inline]
    pub fn try_insert_or_update(
        &mut self,
        key: CVRString<TAllocator>,
        adder: impl FnOnce() -> CVR<TAllocator>,
        updater: impl FnOnce(&mut CVR<TAllocator>),
    ) -> Result<(), CVRObjectInsertError> {
        let _ = self.value.try_insert_or_update(key, adder, updater)?;
        Ok(())
    }

    /// Checks if the object contains the key.
    #[inline]
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        CVRString<TAllocator>: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.get(key).is_some()
    }

    /// Returns the value with key matching the given `key`.
    /// Return `None` if the key is not present in the [`CVRObject`].
    #[inline]
    pub fn get<Q>(&self, key: &Q) -> Option<&CVR<TAllocator>>
    where
        CVRString<TAllocator>: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.get_key_value(key).map(|(_, value)| value)
    }

    /// Returns the key-value pair with key matching the given `key`.
    /// Return `None` if the key is not present in the [`CVRObject`].
    #[inline]
    pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&CVRString<TAllocator>, &CVR<TAllocator>)>
    where
        CVRString<TAllocator>: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.value.get(key).map(KVP::unpack)
    }

    /// Removes the key-value pair with key matching the given `key`.
    /// Returns the removed value if the key was present, otherwise `None`.
    #[inline]
    pub fn remove<Q>(&mut self, key: &Q) -> Option<(CVRString<TAllocator>, CVR<TAllocator>)>
    where
        CVRString<TAllocator>: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.value.remove(key).map(KVP::unpack)
    }

    /// Iterates over the object in ascending order.
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (&CVRString<TAllocator>, &CVR<TAllocator>)> {
        self.value.iter().map(KVP::unpack)
    }

    /// Iterates over the object in ascending order.
    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&CVRString<TAllocator>, &mut CVR<TAllocator>)> {
        self.value.iter_mut().map(KVP::unpack)
    }
}

impl<TAllocator: Allocator + Default> Default for CVRObject<TAllocator> {
    fn default() -> Self {
        Self::new()
    }
}

impl<TAllocator: Allocator> From<Infallible> for CVRObject<TAllocator> {
    #[inline]
    fn from(_: Infallible) -> Self {
        unreachable!("From<Infallible> for CVRObject<TAllocator> is not possible");
    }
}

impl<TAllocator: Allocator> core::fmt::Display for CVRObject<TAllocator> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_char('{')?;

        let mut iterator = self.iter();
        if let Some(item) = iterator.next() {
            write!(f, "{}: {}", item.0, item.1)?;
            for item in iterator {
                write!(f, ", {}: {}", item.0, item.1)?;
            }
        }

        f.write_char('}')
    }
}

impl<TAllocator: Allocator + TryClone> TryClone for CVRObject<TAllocator> {
    type Error = TryCloneCVRError;

    fn try_clone(&self) -> Result<Self, Self::Error> {
        let inner = self.value.try_clone().map_err(|_| TryCloneCVRError)?;
        Ok(Self { value: inner })
    }
}

impl<TAllocator: Allocator + TryClone> Clone for CVRObject<TAllocator> {
    fn clone(&self) -> Self {
        self.try_clone().expect("Failed to clone CVRObject")
    }
}
