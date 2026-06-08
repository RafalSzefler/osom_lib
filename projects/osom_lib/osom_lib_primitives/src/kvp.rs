//! Holds the [`KVP (Key-Value Pair)`][KVP] struct and its implementation.
use core::fmt::Display;

use osom_lib_reprc::traits::ReprC;
use osom_lib_try_clone::TryClone;

/// Represents possible errors when trying to clone a [`KVP`].
#[repr(C)]
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
#[must_use]
#[allow(clippy::upper_case_acronyms)]
pub enum KVPTryCloneError<TKeyError, TValueError> {
    /// Cloning the key failed.
    KeyCloneError(TKeyError) = 0,

    /// Cloning the value failed.
    ValueError(TValueError) = 1,
}

/// Represents the `(key, value)` pair, but with `#[repr(C)]` ABI.
#[repr(C)]
#[must_use]
#[derive(Debug, PartialEq, Eq, Hash)]
#[allow(clippy::upper_case_acronyms)]
pub struct KVP<TKey, TValue> {
    pub key: TKey,
    pub value: TValue,
}

unsafe impl<TKey: ReprC, TValue: ReprC> ReprC for KVP<TKey, TValue> {
    const CHECK: () = const {
        osom_lib_reprc::hidden::is_reprc::<TKey>();
        osom_lib_reprc::hidden::is_reprc::<TValue>();
    };
}

impl<TKey, TValue> KVP<TKey, TValue> {
    /// Unpacks the [`KVP`] and returns the key and value as a tuple.
    #[inline]
    #[must_use]
    pub const fn unpack(self) -> (TKey, TValue) {
        let key = unsafe { core::ptr::read(&raw const self.key) };
        let value = unsafe { core::ptr::read(&raw const self.value) };
        core::mem::forget(self);
        (key, value)
    }

    /// Unpacks the [`KVP`] from a raw pointer and returns the key and value as a tuple of raw pointers.
    ///
    /// # Safety
    ///
    /// It is up to the caller to ensure that `ptr` is a valid, non-null pointer to a [`KVP`].
    #[inline(always)]
    #[must_use]
    pub const unsafe fn unpack_ptr(ptr: *mut Self) -> (*mut TKey, *mut TValue) {
        unsafe {
            let mut_ref = ptr.as_mut_unchecked();
            (&raw mut mut_ref.key, &raw mut mut_ref.value)
        }
    }

    /// Converts reference to [`KVP`] into [`KVP`] of references.
    #[inline(always)]
    pub const fn as_ref_kvp(&self) -> KVP<&TKey, &TValue> {
        KVP {
            key: &self.key,
            value: &self.value,
        }
    }

    /// Converts mutable reference to [`KVP`] into [`KVP`] with
    /// key as a reference and value as a mutable reference.
    #[inline(always)]
    pub const fn as_mut_kvp(&mut self) -> KVP<&TKey, &mut TValue> {
        KVP {
            key: &self.key,
            value: &mut self.value,
        }
    }

    /// Returns references to the key and value as a tuple.
    #[inline]
    #[must_use]
    pub const fn as_tuple(&self) -> (&TKey, &TValue) {
        (&self.key, &self.value)
    }

    /// Returns mutable references to the key and value as a tuple.
    #[inline]
    #[must_use]
    pub const fn as_mut_tuple(&mut self) -> (&mut TKey, &mut TValue) {
        (&mut self.key, &mut self.value)
    }
}

impl<TKey: Display, TValue: Display> Display for KVP<TKey, TValue> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "KVP(key: {}, value: {})", self.key, self.value)
    }
}

impl<TKey, TValue> From<(TKey, TValue)> for KVP<TKey, TValue> {
    fn from((key, value): (TKey, TValue)) -> Self {
        Self { key, value }
    }
}

impl<TKey, TValue> From<KVP<TKey, TValue>> for (TKey, TValue) {
    fn from(kvp: KVP<TKey, TValue>) -> Self {
        kvp.unpack()
    }
}

impl<TKey: TryClone, TValue: TryClone> TryClone for KVP<TKey, TValue> {
    type Error = KVPTryCloneError<TKey::Error, TValue::Error>;

    fn try_clone(&self) -> Result<Self, Self::Error> {
        Ok(Self {
            key: self.key.try_clone().map_err(KVPTryCloneError::KeyCloneError)?,
            value: self.value.try_clone().map_err(KVPTryCloneError::ValueError)?,
        })
    }
}

impl<TKey: TryClone, TValue: TryClone> Clone for KVP<TKey, TValue> {
    fn clone(&self) -> Self {
        self.try_clone().expect("[KVP::clone] failure")
    }
}
