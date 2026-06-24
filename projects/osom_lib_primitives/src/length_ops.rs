#![allow(clippy::cast_possible_wrap)]

use crate::{as_i32::AsI32, length::Length};

impl<T: AsI32> core::ops::Add<T> for Length {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Self::try_from_i32(self.as_u32() as i32 + rhs.as_i32()).unwrap()
    }
}

impl<T: AsI32> core::ops::AddAssign<T> for Length {
    fn add_assign(&mut self, rhs: T) {
        *self = Self::try_from_i32(self.as_u32() as i32 + rhs.as_i32()).unwrap();
    }
}

impl<T: AsI32> core::ops::Sub<T> for Length {
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        Self::try_from_i32(self.as_u32() as i32 - rhs.as_i32()).unwrap()
    }
}

impl<T: AsI32> core::ops::SubAssign<T> for Length {
    fn sub_assign(&mut self, rhs: T) {
        *self = Self::try_from_i32(self.as_u32() as i32 - rhs.as_i32()).unwrap();
    }
}
