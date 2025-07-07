use core::marker::PhantomData;

use crate::{number::Number, traits::RandomnessSource};

trait NextOsNumber {
    fn next_number() -> Self;
}

impl NextOsNumber for u32 {
    fn next_number() -> Self {
        getrandom::u32().expect("Failed to get u32 random number from OS")
    }
}

impl NextOsNumber for u64 {
    fn next_number() -> Self {
        getrandom::u64().expect("Failed to get u64 random number from OS")
    }
}

impl NextOsNumber for u128 {
    fn next_number() -> Self {
        let mut array = [0u8; size_of::<Self>()];
        getrandom::fill(&mut array).expect("Failed to get u128 random number from OS");
        unsafe { core::mem::transmute::<[u8; size_of::<Self>()], Self>(array) }
    }
}

/// Randomness source that retrieves randomness from the operating system.
/// 
/// At the moment implemented using [`getrandom`] crate.
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use]
#[repr(transparent)]
#[allow(private_bounds)]
pub struct OsRandomnessSource<ANumber: Number + NextOsNumber> {
    _phantom: PhantomData<ANumber>,
}

impl<ANumber: Number + NextOsNumber> Default for OsRandomnessSource<ANumber> {
    fn default() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<ANumber: Number + NextOsNumber> RandomnessSource for OsRandomnessSource<ANumber> {
    type TNumber = ANumber;

    fn next_number(&mut self) -> Self::TNumber {
        Self::TNumber::next_number()
    }

    fn fill_bytes(&mut self, bytes: &mut [u8]) {
        getrandom::fill(bytes).expect("Failed to fill bytes from OS");
    }
}
