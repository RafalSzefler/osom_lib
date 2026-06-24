//! Holds the [`COption`] enum.
#![allow(clippy::derivable_impls)]

use core::fmt::Display;
use core::hash::Hash;
use core::{mem::forget, ptr};

use osom_lib_reprc::{macros::reprc, traits::ReprC};
use osom_lib_try_clone::TryClone;

/// This enum is essentially the same as the standard `Option`,
/// except it is `#[repr(C)]` and thus safe to use a across the
/// ffi boundaries.
///
/// It additionally provides `const` variants of `Option` methods.
#[reprc]
#[repr(u8)]
#[derive(Debug)]
#[must_use]
pub enum COption<TValue>
where
    TValue: ReprC,
{
    None = 0,
    Some(TValue) = 1,
}

impl<TValue> COption<TValue>
where
    TValue: ReprC,
{
    /// Returns `true` if the enum holds [`COption::Some`],
    /// `false` otherwise.
    #[inline]
    pub const fn is_some(&self) -> bool {
        match self {
            COption::Some(_) => true,
            COption::None => false,
        }
    }

    /// Returns `true` if the enum holds [`COption::None`],
    /// `false` otherwise.
    #[inline]
    pub const fn is_none(&self) -> bool {
        !self.is_some()
    }

    /// Unwraps currently stored [`COption::Some`] value.
    ///
    /// # Panics
    ///
    /// Only when `self` actually holds a [`COption::None`] value.
    #[inline]
    pub fn expect(self, message: &str) -> TValue {
        match &self {
            COption::Some(value) => {
                let data = unsafe { ptr::read(value) };
                forget(self);
                data
            }
            COption::None => {
                panic!("`COption::expect()`: {}.", message);
            }
        }
    }

    /// Unwraps currently stored [`COption::Some`] value.
    ///
    /// # Panics
    ///
    /// Only when `self` actually holds a [`COption::None`] value.
    #[inline]
    pub const fn unwrap(self) -> TValue {
        match &self {
            COption::Some(value) => {
                let data = unsafe { ptr::read(value) };
                forget(self);
                data
            }
            COption::None => {
                panic!("called `COption::unwrap()` on a `None` value.");
            }
        }
    }

    /// Unwraps currently stored [`COption::Some`] value.
    ///
    /// # Safety
    ///
    /// This function does not verify whether the stored value
    /// is actually [`COption::Some`]. The behaviour is undefined if it is not.
    #[inline]
    pub const unsafe fn unwrap_unchecked(self) -> TValue {
        match &self {
            COption::Some(value) => {
                let data = unsafe { ptr::read(value) };
                forget(self);
                data
            }
            COption::None => {
                unsafe { core::hint::unreachable_unchecked() };
            }
        }
    }

    /// Converts [`COption`] into standard `Option`.
    #[inline]
    pub const fn into_option(self) -> Option<TValue> {
        match &self {
            COption::Some(value) => {
                let data = unsafe { ptr::read(value) };
                forget(self);
                Some(data)
            }
            COption::None => {
                forget(self);
                None
            }
        }
    }

    /// Converts standard `Option` into [`COption`].
    #[inline]
    #[allow(clippy::single_match_else)]
    pub const fn from_option(option: Option<TValue>) -> Self {
        match &option {
            Some(value) => {
                let data = unsafe { ptr::read(value) };
                forget(option);
                COption::Some(data)
            }
            None => {
                forget(option);
                COption::None
            }
        }
    }

    /// Converts [`COption`] into a reference to the stored value.
    #[inline]
    pub const fn as_ref(&self) -> COption<&TValue> {
        match *self {
            COption::Some(ref value) => COption::Some(value),
            COption::None => COption::None,
        }
    }

    /// Converts [`COption`] into a mutable reference to the stored value.
    #[inline]
    pub const fn as_mut(&mut self) -> COption<&mut TValue> {
        match *self {
            COption::Some(ref mut value) => COption::Some(value),
            COption::None => COption::None,
        }
    }
}

impl<TValue: ReprC> From<Option<TValue>> for COption<TValue> {
    fn from(option: Option<TValue>) -> Self {
        COption::from_option(option)
    }
}

impl<TValue: ReprC> From<COption<TValue>> for Option<TValue> {
    fn from(option: COption<TValue>) -> Self {
        option.into_option()
    }
}

impl<TValue: ReprC> Default for COption<TValue> {
    fn default() -> Self {
        COption::None
    }
}

impl<TValue: ReprC + Clone> Clone for COption<TValue> {
    fn clone(&self) -> Self {
        match self {
            COption::Some(value) => COption::Some(value.clone()),
            COption::None => COption::None,
        }
    }
}

impl<TValue: ReprC + TryClone> TryClone for COption<TValue> {
    type Error = <TValue as TryClone>::Error;
    fn try_clone(&self) -> Result<Self, Self::Error> {
        match self {
            COption::Some(value) => Ok(COption::Some(value.try_clone()?)),
            COption::None => Ok(COption::None),
        }
    }
}

impl<TValue: ReprC + PartialEq> PartialEq for COption<TValue> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Some(l0), Self::Some(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl<TValue: ReprC + Eq> Eq for COption<TValue> {}

impl<TValue: ReprC + Hash> Hash for COption<TValue> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
        if let COption::Some(value) = self {
            value.hash(state);
        }
    }
}

impl<TValue: ReprC + Display> Display for COption<TValue> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            COption::Some(value) => {
                f.write_str("Some(")?;
                value.fmt(f)?;
                f.write_str(")")?;
                Ok(())
            }
            COption::None => write!(f, "None"),
        }
    }
}
