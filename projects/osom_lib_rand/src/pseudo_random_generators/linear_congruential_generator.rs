//! # Notes
//!
//! All multipliers are chosen based on "Computationally Easy, Spectrally Good Multipliers for
//! Congruential Pseudorandom Number Generators" paper by Guy Steele and Sebastiano Vigna.
//!
//! On the other hand, `increment` produces full period (over modulus being power of two)
//! if and only if `increment % 4 == 1`. This result can be found in
//! "Notes on a New Pseudo-Random Number Generator" paper by Martin Greenberger.
//! Thus we fix an appropriate prime increment for all generators.
use crate::number::Number;
use crate::traits::PseudoRandomGenerator;

const PRIME_INCREMENT: u32 = const {
    let value = 3326489;
    assert!(value % 4 == 1);
    value
};
const MULTIPLIER_32: u32 = 0x915F77F5;
const MULTIPLIER_64: u64 = 0xFC0072FA0B15F4FD;
const MULTIPLIER_128: u128 = 0xAADEC8C3186345282B4E141F3A1232D5;

/// The classical LCG algorithm. In the most general form it does:
/// `X_{n+1} = (X_n * multiplier + increment) % modulus`.
/// 
/// Of course, we use `32`, `64` and `128` as the modulus, and so we completely
/// skip it and do wrapping arithmetic.
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use]
pub struct LinearCongruentialGenerator<ANumber: Number> {
    current: ANumber,
    multiplier: ANumber,
    increment: ANumber,
}

impl<ANumber: Number> LinearCongruentialGenerator<ANumber> {
    /// Creates a new LCG with the given parameters.
    /// 
    /// # Notes
    /// 
    /// Both `multiplier` and `increment` have to be odd. Moreover, `increment`
    /// should satisfy `increment % 4 == 1` condition, otherwise the generator
    /// won't produce full period.
    #[inline(always)]
    pub const fn with_params(multiplier: ANumber, increment: ANumber, initial: ANumber) -> Self {
        Self {
            current: initial,
            multiplier,
            increment,
        }
    }

    /// Returns the next value of the LCG.
    #[inline(always)]
    pub fn next_value(&mut self) -> ANumber {
        self.current = self.current.wrapping_mul(self.multiplier).wrapping_add(self.increment);
        self.current
    }
}

impl LinearCongruentialGenerator<u32> {
    /// Creates a new LCG with the given initial value. The remaining parameters
    /// are carefuly chosen to maximize generator's quality.
    pub const fn new(initial: u32) -> Self {
        Self::with_params(MULTIPLIER_32, PRIME_INCREMENT, initial)
    }
}

impl PseudoRandomGenerator for LinearCongruentialGenerator<u32> {
    type TNumber = u32;

    fn next_number(&mut self) -> Self::TNumber {
        self.next_value()
    }

    fn from_randomness_source(source: &mut impl crate::traits::RandomnessSource<TNumber = Self::TNumber>) -> Self {
        Self::new(source.next_number())
    }
}

impl LinearCongruentialGenerator<u64> {
    /// Creates a new LCG with the given initial value. The remaining parameters
    /// are carefuly chosen to maximize generator's quality.
    pub const fn new(initial: u64) -> Self {
        Self::with_params(MULTIPLIER_64, PRIME_INCREMENT as u64, initial)
    }
}

impl PseudoRandomGenerator for LinearCongruentialGenerator<u64> {
    type TNumber = u64;

    fn next_number(&mut self) -> Self::TNumber {
        self.next_value()
    }

    fn from_randomness_source(source: &mut impl crate::traits::RandomnessSource<TNumber = Self::TNumber>) -> Self {
        Self::new(source.next_number())
    }
}

impl LinearCongruentialGenerator<u128> {
    /// Creates a new LCG with the given initial value. The remaining parameters
    /// are carefuly chosen to maximize generator's quality.
    pub const fn new(initial: u128) -> Self {
        Self::with_params(MULTIPLIER_128, PRIME_INCREMENT as u128, initial)
    }
}

impl PseudoRandomGenerator for LinearCongruentialGenerator<u128> {
    type TNumber = u64;

    fn next_number(&mut self) -> Self::TNumber {
        (self.next_value() >> 64) as u64
    }

    fn from_randomness_source(source: &mut impl crate::traits::RandomnessSource<TNumber = Self::TNumber>) -> Self {
        Self::new(u128::from(source.next_number()))
    }
}
