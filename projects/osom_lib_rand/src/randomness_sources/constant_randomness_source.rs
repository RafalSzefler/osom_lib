use crate::number::Number;
use crate::pseudo_random_number_generators::LinearCongruentialGenerator;
use crate::traits::RandomnessSource;

/// A trivial [`RandomnessSource`] that always returns the same sequence of
/// values based on a seed. Which by default is constant as well.
///
/// # Warning
///
/// This struct should only be used for testing purposes, not in production.
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use]
#[repr(transparent)]
pub struct ConstantRandomnessSource<ANumber: Number> {
    generator: LinearCongruentialGenerator<ANumber>,
}

impl<ANumber: Number> ConstantRandomnessSource<ANumber> {
    /// Creates a new [`ConstantRandomnessSource`] with the given seed.
    pub fn new(seed: ANumber) -> Self {
        Self {
            generator: LinearCongruentialGenerator::new(seed),
        }
    }
}

impl<ANumber: Number> RandomnessSource for ConstantRandomnessSource<ANumber> {
    type TNumber = ANumber;

    fn next_number(&mut self) -> Self::TNumber {
        self.generator.next_value()
    }
}

impl<ANumber: Number> Default for ConstantRandomnessSource<ANumber> {
    fn default() -> Self {
        Self::new(ANumber::ONE)
    }
}
