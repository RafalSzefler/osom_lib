//! Holds the [`CResult`] enum.

use core::{mem::forget, ptr};

use osom_lib_reprc::{macros::reprc, traits::ReprC};

/// This enum is essentially the same as the standard `Result`,
/// except it is `#[repr(C)]` and thus safe to use a cross the
/// ffi boundaries.
///
/// Note that the safety is guaranteed only for the enum itself,
/// not for the data it holds. Meaning you still have to manually
/// use `#[repr(C)]` on `TOk` and `TErr` for the inner data to be
/// ffi safe.
///
/// It additionally provides `const` variants of `Result` methods.
///
/// Also note that the layout is fixed, and unlike standard Result,
/// it doesn't depend on `TOk`, `TErr`.
#[reprc]
#[repr(u8)]
#[derive(Debug)]
#[must_use]
pub enum CResult<TOk, TErr>
where
    TOk: ReprC,
    TErr: ReprC,
{
    Ok(TOk) = 0,
    Err(TErr) = 1,
}

impl<TOk, TErr> CResult<TOk, TErr>
where
    TOk: ReprC,
    TErr: ReprC,
{
    /// Returns `true` if the enum holds [`CResult::Ok`],
    /// `false` otherwise.
    #[inline(always)]
    pub const fn is_ok(&self) -> bool {
        match self {
            CResult::Ok(_) => true,
            CResult::Err(_) => false,
        }
    }

    /// Returns `true` if the enum holds [`CResult::Err`],
    /// `false` otherwise.
    #[inline(always)]
    pub const fn is_err(&self) -> bool {
        !self.is_ok()
    }

    /// Unwraps currently stored [`CResult::Ok`] value.
    ///
    /// # Panics
    ///
    /// Only when `self` actually holds a [`CResult::Err`] value.
    #[inline(always)]
    pub const fn unwrap(self) -> TOk {
        match &self {
            CResult::Ok(ok) => {
                let data = unsafe { ptr::read(ok) };
                forget(self);
                data
            }
            CResult::Err(_) => {
                panic!("called `CResult::unwrap()` on an `Err` value.");
            }
        }
    }

    /// Unwraps currently stored [`CResult::Ok`] value.
    ///
    /// # Safety
    ///
    /// This function does not verify whether the stored value
    /// is actually [`CResult::Ok`]. The behaviour is undefined if it is not.
    #[inline(always)]
    pub const unsafe fn unwrap_unchecked(self) -> TOk {
        match &self {
            CResult::Ok(ok) => {
                let data = unsafe { ptr::read(ok) };
                forget(self);
                data
            }
            CResult::Err(_) => {
                unsafe { core::hint::unreachable_unchecked() };
            }
        }
    }

    /// Unwraps currently stored [`CResult::Err`] value.
    ///
    /// # Panics
    ///
    /// Only when `self` actually holds an [`CResult::Ok`] value.
    #[inline(always)]
    pub const fn unwrap_err(self) -> TErr {
        match &self {
            CResult::Ok(_) => {
                panic!("called `CResult::unwrap_err()` on an `Ok` value.");
            }
            CResult::Err(err) => {
                let data = unsafe { ptr::read(err) };
                forget(self);
                data
            }
        }
    }

    /// Unwraps currently stored [`CResult::Err`] value.
    ///
    /// # Safety
    ///
    /// This function does not verify whether the stored value
    /// is actually [`CResult::Err`]. The behaviour is undefined if it is not.
    #[inline(always)]
    pub const unsafe fn unwrap_err_unchecked(self) -> TErr {
        match &self {
            CResult::Ok(_) => {
                unsafe { core::hint::unreachable_unchecked() };
            }
            CResult::Err(err) => {
                let data = unsafe { ptr::read(err) };
                forget(self);
                data
            }
        }
    }

    #[allow(clippy::missing_errors_doc)]
    /// Converts [`CResult`] into standard `Result`.
    #[inline(always)]
    pub const fn into_result(self) -> Result<TOk, TErr> {
        match &self {
            CResult::Ok(ok) => {
                let data = unsafe { ptr::read(ok) };
                forget(self);
                Ok(data)
            }
            CResult::Err(err) => {
                let data = unsafe { ptr::read(err) };
                forget(self);
                Err(data)
            }
        }
    }

    /// Converts standard `Result` into [`CResult`].
    #[inline(always)]
    pub const fn from_result(result: Result<TOk, TErr>) -> Self {
        match &result {
            Ok(ok) => {
                let data = unsafe { ptr::read(ok) };
                forget(result);
                Self::Ok(data)
            }
            Err(err) => {
                let data = unsafe { ptr::read(err) };
                forget(result);
                Self::Err(data)
            }
        }
    }
}

impl<TOk, TErr> From<CResult<TOk, TErr>> for Result<TOk, TErr>
where
    TOk: ReprC,
    TErr: ReprC,
{
    fn from(value: CResult<TOk, TErr>) -> Self {
        value.into_result()
    }
}

impl<TOk, TErr> From<Result<TOk, TErr>> for CResult<TOk, TErr>
where
    TOk: ReprC,
    TErr: ReprC,
{
    fn from(value: Result<TOk, TErr>) -> Self {
        Self::from_result(value)
    }
}
