use crate::models::DisplayAsHex;

pub trait AsDisplayAsHex {
    fn as_display_as_hex(&self) -> DisplayAsHex<'_>;
}

impl AsDisplayAsHex for &[u8] {
    fn as_display_as_hex(&self) -> DisplayAsHex<'_> {
        DisplayAsHex::new(self)
    }
}

impl AsDisplayAsHex for [u8] {
    fn as_display_as_hex(&self) -> DisplayAsHex<'_> {
        DisplayAsHex::new(self)
    }
}

impl<const N: usize> AsDisplayAsHex for [u8; N] {
    fn as_display_as_hex(&self) -> DisplayAsHex<'_> {
        DisplayAsHex::new(self)
    }
}

impl<const N: usize> AsDisplayAsHex for &[u8; N] {
    fn as_display_as_hex(&self) -> DisplayAsHex<'_> {
        DisplayAsHex::new(*self)
    }
}

impl AsDisplayAsHex for Vec<u8> {
    fn as_display_as_hex(&self) -> DisplayAsHex<'_> {
        DisplayAsHex::new(self)
    }
}

impl AsDisplayAsHex for &Vec<u8> {
    fn as_display_as_hex(&self) -> DisplayAsHex<'_> {
        DisplayAsHex::new(self)
    }
}
