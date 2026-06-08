use crate::traits::ReprC;

/// Provides a const time check to see if the type is `ReprC`.
#[inline(always)]
pub const fn is_reprc<T: ReprC>() {}
