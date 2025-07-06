use crate::number::Number;
use crate::traits::RandomnessSource;

/// A trivial [`RandomnessSource`] that always returns the same value.
///
/// # Warning
///
/// This struct should only be used for testing purposes, not in production.
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use]
#[repr(transparent)]
pub struct ConstantRandomnessSource<ANumber: Number> {
    value: ANumber,
}

impl<ANumber: Number> ConstantRandomnessSource<ANumber> {
    pub fn new(value: ANumber) -> Self {
        Self { value }
    }
}

impl<ANumber: Number> RandomnessSource for ConstantRandomnessSource<ANumber> {
    type TNumber = ANumber;

    fn next_number(&mut self) -> Self::TNumber {
        self.value
    }
}

impl<ANumber: Number> Default for ConstantRandomnessSource<ANumber> {
    fn default() -> Self {
        Self::new(ANumber::ONE)
    }
}
