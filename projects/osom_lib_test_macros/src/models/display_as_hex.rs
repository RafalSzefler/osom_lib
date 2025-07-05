use core::fmt;

/// A simple wrapper around a slice of bytes that implements `Display`
/// which display the slice in the form of hex bytes.
#[must_use]
#[repr(transparent)]
pub struct DisplayAsHex<'a> {
    data: &'a [u8],
}

impl<'a> DisplayAsHex<'a> {
    #[inline(always)]
    pub const fn new(data: &'a [u8]) -> Self {
        Self { data }
    }

    #[inline(always)]
    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        self.data
    }
}

impl fmt::Display for DisplayAsHex<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;

        let mut iter = self.data.iter();
        if let Some(byte) = iter.next() {
            write!(f, "{byte:#04x}")?;
            for byte in iter {
                write!(f, ", {byte:#04x}")?;
            }
        }

        write!(f, "]")
    }
}
