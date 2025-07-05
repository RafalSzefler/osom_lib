use core::{borrow::Borrow, hash::Hash};

/// A trait that allows two types to be considered equivalent,
/// in the sense of having the same hash values and being equal
/// in general sense. This applies to e.g. the standard `String`
/// and `&str` types.
///
/// By default the trait is implemented for all types that implement
/// [`Borrow`] in addition to [`Eq`] and [`Hash`].
pub trait Equivalent<B>: Eq + Hash
where
    B: Eq + Hash,
{
    fn is_equivalent_to(&self, other: &B) -> bool;
}

impl<A, B> Equivalent<B> for A
where
    A: Eq + Hash,
    B: Eq + Hash + Borrow<A>,
{
    #[inline(always)]
    fn is_equivalent_to(&self, other: &B) -> bool {
        self == other.borrow()
    }
}
