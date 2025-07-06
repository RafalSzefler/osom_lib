use core::marker::PhantomData;

use crate::{number::{Number, MAX_NUMBER_SIZE}, traits::RandomnessSource};

/// Randomness source that retrieves randomness from the operating system.
/// 
/// At the moment implemented using [`getrandom`] crate.
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use]
#[repr(transparent)]
pub struct OsRandomnessSource<ANumber: Number> {
    _phantom: PhantomData<ANumber>,
}

impl<ANumber: Number> Default for OsRandomnessSource<ANumber> {
    fn default() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<ANumber: Number> RandomnessSource for OsRandomnessSource<ANumber> {
    type TNumber = ANumber;

    fn next_number(&mut self) -> Self::TNumber {
        let mut result = [0u8; MAX_NUMBER_SIZE];
        let slice = &mut result[..ANumber::size()];
        getrandom::fill(slice).unwrap();
        Self::TNumber::from_le_bytes(slice)
    }

    fn fill_bytes(&mut self, bytes: &mut [u8]) {
        getrandom::fill(bytes).unwrap();
    }
}
