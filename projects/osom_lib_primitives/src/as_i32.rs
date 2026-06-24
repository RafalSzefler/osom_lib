pub(crate) trait AsI32 {
    fn as_i32(&self) -> i32;
}

impl AsI32 for i8 {
    fn as_i32(&self) -> i32 {
        i32::from(*self)
    }
}

impl AsI32 for i16 {
    fn as_i32(&self) -> i32 {
        i32::from(*self)
    }
}

impl AsI32 for i32 {
    #[inline(always)]
    fn as_i32(&self) -> i32 {
        *self
    }
}
