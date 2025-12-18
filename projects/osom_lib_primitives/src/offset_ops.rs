use crate::{as_i32::AsI32, offset::Offset};

impl AsI32 for Offset {
    fn as_i32(&self) -> i32 {
        Offset::as_i32(self)
    }
}

impl<T: AsI32> core::ops::Add<T> for Offset {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        unsafe { Self::new_unchecked(self.as_i32() + rhs.as_i32()) }
    }
}

impl<T: AsI32> core::ops::AddAssign<T> for Offset {
    fn add_assign(&mut self, rhs: T) {
        *self = unsafe { Self::new_unchecked(self.as_i32() + rhs.as_i32()) };
    }
}

impl<T: AsI32> core::ops::Sub<T> for Offset {
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        unsafe { Self::new_unchecked(self.as_i32() - rhs.as_i32()) }
    }
}

impl<T: AsI32> core::ops::SubAssign<T> for Offset {
    fn sub_assign(&mut self, rhs: T) {
        *self = unsafe { Self::new_unchecked(self.as_i32() - rhs.as_i32()) };
    }
}
