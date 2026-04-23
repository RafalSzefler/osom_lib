use osom_lib_alloc::traits::AllocationError;
use osom_lib_reprc::macros::reprc;

#[reprc]
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum ImmutableStringError {
    AllocationError = 0,
    MaxLengthExceeded = 1,
}

impl From<AllocationError> for ImmutableStringError {
    fn from(_: AllocationError) -> Self {
        Self::AllocationError
    }
}
