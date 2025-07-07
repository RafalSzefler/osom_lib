use core::marker::PhantomData;

use crate::{number::{Number, NumberType}, traits::RandomnessSource};

#[inline(always)]
fn next_os_number<ANumber: Number>() -> ANumber {
    unsafe {
        match ANumber::NUMBER_TYPE {
            NumberType::U32 => {
                let no = getrandom::u32().expect("Failed to get u32 random number from OS");
                ANumber::from_u32(no)
            }
            NumberType::U64 => {
                let no = getrandom::u64().expect("Failed to get u64 random number from OS");
                ANumber::from_u64_unchecked(no)
            }
            NumberType::U128 => {
                let mut array = [0u8; size_of::<u128>()];
                getrandom::fill(&mut array).expect("Failed to get u128 random number from OS");
                let no = u128::from_ne_bytes(array);
                ANumber::from_u128_unchecked(no)
            }
        }
    }
}

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
        next_os_number::<Self::TNumber>()
    }

    fn fill_bytes(&mut self, bytes: &mut [u8]) {
        getrandom::fill(bytes).expect("Failed to fill bytes from OS");
    }
}
