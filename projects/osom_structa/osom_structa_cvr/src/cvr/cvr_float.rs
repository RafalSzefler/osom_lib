#![allow(clippy::cast_sign_loss)]
use core::{cmp::Ordering, fmt::Display};

use osom_lib_reprc::macros::reprc;
use osom_lib_try_clone::TryClone;

/// A thin wrapper around `f64` value. The biggest difference is
/// that implements total ordering via `.tolal_cmp()`.
#[reprc]
#[derive(Debug, Clone, Copy)]
#[must_use]
pub struct CVRFloat {
    inner: f64,
}

impl CVRFloat {
    /// Creates a new [`CVRFloat`] instance.
    pub const fn new(inner: f64) -> Self {
        Self { inner }
    }

    /// Returns the underlying `f64` value.
    #[inline]
    #[must_use]
    pub const fn inner(self) -> f64 {
        self.inner
    }
}

impl PartialEq for CVRFloat {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for CVRFloat {}

impl PartialOrd for CVRFloat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CVRFloat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.inner.total_cmp(&other.inner)
    }
}

impl core::hash::Hash for CVRFloat {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.inner.to_le_bytes().hash(state);
    }
}

osom_lib_macros::unreachable_from_infallible!(CVRFloat);

impl From<f64> for CVRFloat {
    fn from(value: f64) -> Self {
        Self { inner: value }
    }
}

impl From<CVRFloat> for f64 {
    fn from(value: CVRFloat) -> Self {
        value.inner
    }
}

impl Display for CVRFloat {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl TryClone for CVRFloat {
    type Error = core::convert::Infallible;

    fn try_clone(&self) -> Result<Self, Self::Error> {
        Ok(*self)
    }
}
