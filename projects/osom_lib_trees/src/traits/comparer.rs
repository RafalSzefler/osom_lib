/// Represents a comparer for two values.
pub trait Compare<TOther> {
    /// Checks if `self < other`.
    fn is_less(&self, other: &TOther) -> bool;

    /// Checks if `self == other`.
    fn is_equal(&self, other: &TOther) -> bool;

    /// Checks if `self <= other`. This should be equivalent to `self < other || self == other`.
    fn is_less_or_equal(&self, other: &TOther) -> bool {
        self.is_less(other) || self.is_equal(other)
    }

    /// Checks if `self > other`. This should be equivalent to `!(self <= other)`.
    fn is_greater(&self, other: &TOther) -> bool {
        !self.is_less_or_equal(other)
    }

    /// Checks if `self >= other`. This should be equivalent to `self > other || self == other`.
    fn is_greater_or_equal(&self, other: &TOther) -> bool {
        !self.is_less(other)
    }

    /// Checks if `self != other`. This should be equivalent to `!(self == other)`.
    fn is_not_equal(&self, other: &TOther) -> bool {
        !self.is_equal(other)
    }
}

impl<T, K> Compare<K> for T
where
    T: Ord,
    K: core::borrow::Borrow<T>,
{
    fn is_less(&self, other: &K) -> bool {
        self < other.borrow()
    }

    fn is_equal(&self, other: &K) -> bool {
        self == other.borrow()
    }
}
