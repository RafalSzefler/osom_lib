//! # Notes
//!
//! All multipliers are chosen based on "Computationally Easy, Spectrally Good Multipliers for
//! Congruential Pseudorandom Number Generators" paper by Guy Steele and Sebastiano Vigna.
//!
//! On the other hand, `increment` produces full period (over modulus being power of two)
//! if and only if `increment % 4 == 1`. This result can be found in
//! "Notes on a New Pseudo-Random Number Generator" paper by Martin Greenberger.
//! Thus we fix an appropriate prime increment for all generators.
use crate::number::{Number, NumberType};
use crate::traits::{PseudoRandomNumberGenerator, RandomnessSource};

const PRIME_INCREMENT: u32 = const {
    let value = 3326489;
    assert!(value % 4 == 1);
    value
};

struct LcgConstants<ANumber: Number> {
    multiplier: ANumber,
    prime_increment: ANumber,
}

impl<ANumber: Number> LcgConstants<ANumber> {
    #[inline(always)]
    fn new() -> Self {
        let multiplier = unsafe {
            match ANumber::NUMBER_TYPE {
                NumberType::U32 => ANumber::from_u32(0x915F77F5),
                NumberType::U64 => ANumber::from_u64_unchecked(0xFC0072FA0B15F4FD),
                NumberType::U128 => ANumber::from_u128_unchecked(0xAADEC8C3186345282B4E141F3A1232D5),
            }
        };

        Self {
            multiplier,
            prime_increment: ANumber::from_u32(PRIME_INCREMENT),
        }
    }

    #[inline(always)]
    const fn multiplier(&self) -> ANumber {
        self.multiplier
    }

    #[inline(always)]
    const fn prime_increment(&self) -> ANumber {
        self.prime_increment
    }
}

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

impl<ANumber: Number> LinearCongruentialGenerator<ANumber> {
    /// Creates a new LCG with the given initial value. The remaining parameters
    /// are carefuly chosen to maximize generator's quality.
    pub fn new(initial: ANumber) -> Self {
        let constants = LcgConstants::<ANumber>::new();
        Self::with_params(constants.multiplier(), constants.prime_increment(), initial)
    }
}

impl<ANumber: Number> PseudoRandomNumberGenerator for LinearCongruentialGenerator<ANumber> {
    type TNumber = ANumber;

    fn next_number(&mut self) -> Self::TNumber {
        self.next_value()
    }

    fn from_randomness_source(source: &mut impl RandomnessSource<TNumber = Self::TNumber>) -> Self {
        Self::new(source.next_number())
    }
}
