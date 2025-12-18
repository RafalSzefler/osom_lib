//! Holds the [`Align`] primitive.

#![allow(clippy::unreadable_literal)]

use paste::paste;

use osom_lib_reprc::{macros::reprc, traits::ReprC};

/// Represents a zero-sized type that has `ALIGN` as its alignment.
/// By embedding it into a struct we can enforce a different alignment
/// based on `const ALIGN: usize` generic parameter.
///
/// # Example
///
/// ```rust
/// use osom_lib_primitives::align::{Align, Alignment};
///
/// /// This struct's alignment depends on `ALIGN` parameter.
/// pub struct TestAlign<const ALIGN: usize>
/// where
///     Align<ALIGN>: Alignment,
/// {
///     _inner: Align<ALIGN>,
/// }
/// ```
#[reprc]
#[repr(transparent)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
#[must_use]
pub struct Align<const ALIGN: usize>
where
    Self: Alignment,
{
    inner: [<Self as private::AlignedTrait>::AlignedType; 0],
}

impl<const ALIGN: usize> Align<ALIGN>
where
    Self: Alignment,
{
    #[inline(always)]
    pub const fn default() -> Self {
        Self { inner: [] }
    }
}

/// A helper trait to put for constrainting generic `ALIGN` parameter.
pub trait Alignment: private::AlignedTrait {}

#[allow(clippy::wildcard_imports)]
mod private {
    use super::*;

    pub trait AlignedTrait {
        /// A zero-sized type of particular alignment.
        type AlignedType: core::fmt::Debug + Copy + Eq + PartialEq + Send + Sync + Unpin + ReprC;
    }

    macro_rules! align_impl {
        ( $size: literal ) => {
            paste! {
                #[reprc]
                #[repr(C, align($size))]
                #[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
                pub struct [< Align $size >] { }

                impl AlignedTrait for Align<$size> {
                    type AlignedType = [< Align $size >];
                }

                impl Alignment for Align<$size> { }
            }
        };
    }

    align_impl!(0b00000000_00000000_00000000_00000001);
    align_impl!(0b00000000_00000000_00000000_00000010);
    align_impl!(0b00000000_00000000_00000000_00000100);
    align_impl!(0b00000000_00000000_00000000_00001000);
    align_impl!(0b00000000_00000000_00000000_00010000);
    align_impl!(0b00000000_00000000_00000000_00100000);
    align_impl!(0b00000000_00000000_00000000_01000000);
    align_impl!(0b00000000_00000000_00000000_10000000);
    align_impl!(0b00000000_00000000_00000001_00000000);
    align_impl!(0b00000000_00000000_00000010_00000000);
    align_impl!(0b00000000_00000000_00000100_00000000);
    align_impl!(0b00000000_00000000_00001000_00000000);
    align_impl!(0b00000000_00000000_00010000_00000000);
    align_impl!(0b00000000_00000000_00100000_00000000);
    align_impl!(0b00000000_00000000_01000000_00000000);
    align_impl!(0b00000000_00000000_10000000_00000000);
    align_impl!(0b00000000_00000001_00000000_00000000);
    align_impl!(0b00000000_00000010_00000000_00000000);
    align_impl!(0b00000000_00000100_00000000_00000000);
    align_impl!(0b00000000_00001000_00000000_00000000);
    align_impl!(0b00000000_00010000_00000000_00000000);
    align_impl!(0b00000000_00100000_00000000_00000000);
    align_impl!(0b00000000_01000000_00000000_00000000);
    align_impl!(0b00000000_10000000_00000000_00000000);
    align_impl!(0b00000001_00000000_00000000_00000000);
    align_impl!(0b00000010_00000000_00000000_00000000);
    align_impl!(0b00000100_00000000_00000000_00000000);
    align_impl!(0b00001000_00000000_00000000_00000000);
    align_impl!(0b00010000_00000000_00000000_00000000);
    align_impl!(0b00100000_00000000_00000000_00000000);
}

impl<const N: usize> core::hash::Hash for Align<N>
where
    Self: Alignment,
{
    #[inline(always)]
    fn hash<H: core::hash::Hasher>(&self, _: &mut H) {}
}

impl<const N: usize> core::cmp::Ord for Align<N>
where
    Self: Alignment,
{
    #[inline(always)]
    fn cmp(&self, _: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
    }
}

impl<const N: usize> core::cmp::PartialOrd<Self> for Align<N>
where
    Self: Alignment,
{
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<const N: usize> core::fmt::Display for Align<N>
where
    Self: Alignment,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Align<{N}>")
    }
}

const _: () = const {
    assert!(align_of::<Align<1>>() == 1, "Align(1) is expected to be of alignment 1");
    assert!(size_of::<Align<1>>() == 0, "Align(1) is expected to be of size 0");
    assert!(align_of::<Align<2>>() == 2, "Align(2) is expected to be of alignment 2");
    assert!(size_of::<Align<2>>() == 0, "Align(2) is expected to be of size 0");
    assert!(align_of::<Align<4>>() == 4, "Align(4) is expected to be of alignment 4");
    assert!(size_of::<Align<4>>() == 0, "Align(4) is expected to be of size 0");
    assert!(align_of::<Align<8>>() == 8, "Align(8) is expected to be of alignment 8");
    assert!(size_of::<Align<8>>() == 0, "Align(8) is expected to be of size 0");
    assert!(
        align_of::<Align<16>>() == 16,
        "Align(16) is expected to be of alignment 16"
    );
    assert!(size_of::<Align<16>>() == 0, "Align(16) is expected to be of size 0");
    assert!(
        align_of::<Align<32>>() == 32,
        "Align(32) is expected to be of alignment 32"
    );
    assert!(size_of::<Align<32>>() == 0, "Align(32) is expected to be of size 0");
    assert!(
        align_of::<Align<64>>() == 64,
        "Align(64) is expected to be of alignment 64"
    );
    assert!(size_of::<Align<64>>() == 0, "Align(64) is expected to be of size 0");
    assert!(
        align_of::<Align<128>>() == 128,
        "Align(128) is expected to be of alignment 128"
    );
    assert!(size_of::<Align<128>>() == 0, "Align(128) is expected to be of size 0");
    assert!(
        align_of::<Align<256>>() == 256,
        "Align(256) is expected to be of alignment 256"
    );
    assert!(size_of::<Align<256>>() == 0, "Align(256) is expected to be of size 0");
    assert!(
        align_of::<Align<512>>() == 512,
        "Align(512) is expected to be of alignment 512"
    );
    assert!(size_of::<Align<512>>() == 0, "Align(512) is expected to be of size 0");
    assert!(
        align_of::<Align<1024>>() == 1024,
        "Align(1024) is expected to be of alignment 1024"
    );
    assert!(size_of::<Align<1024>>() == 0, "Align(1024) is expected to be of size 0");
    assert!(
        align_of::<Align<2048>>() == 2048,
        "Align(2048) is expected to be of alignment 2048"
    );
    assert!(size_of::<Align<2048>>() == 0, "Align(2048) is expected to be of size 0");
    assert!(
        align_of::<Align<4096>>() == 4096,
        "Align(4096) is expected to be of alignment 4096"
    );
    assert!(size_of::<Align<4096>>() == 0, "Align(4096) is expected to be of size 0");
    assert!(
        align_of::<Align<8192>>() == 8192,
        "Align(8192) is expected to be of alignment 8192"
    );
    assert!(size_of::<Align<8192>>() == 0, "Align(8192) is expected to be of size 0");
    assert!(
        align_of::<Align<16384>>() == 16384,
        "Align(16384) is expected to be of alignment 16384"
    );
    assert!(
        size_of::<Align<16384>>() == 0,
        "Align(16384) is expected to be of size 0"
    );
    assert!(
        align_of::<Align<32768>>() == 32768,
        "Align(32768) is expected to be of alignment 32768"
    );
    assert!(
        size_of::<Align<32768>>() == 0,
        "Align(32768) is expected to be of size 0"
    );
    assert!(
        align_of::<Align<65536>>() == 65536,
        "Align(65536) is expected to be of alignment 65536"
    );
    assert!(
        size_of::<Align<65536>>() == 0,
        "Align(65536) is expected to be of size 0"
    );
    assert!(
        align_of::<Align<131072>>() == 131072,
        "Align(131072) is expected to be of alignment 131072"
    );
    assert!(
        size_of::<Align<131072>>() == 0,
        "Align(131072) is expected to be of size 0"
    );
    assert!(
        align_of::<Align<262144>>() == 262144,
        "Align(262144) is expected to be of alignment 262144"
    );
    assert!(
        size_of::<Align<262144>>() == 0,
        "Align(262144) is expected to be of size 0"
    );
    assert!(
        align_of::<Align<524288>>() == 524288,
        "Align(524288) is expected to be of alignment 524288"
    );
    assert!(
        size_of::<Align<524288>>() == 0,
        "Align(524288) is expected to be of size 0"
    );
    assert!(
        align_of::<Align<1048576>>() == 1048576,
        "Align(1048576) is expected to be of alignment 1048576"
    );
    assert!(
        size_of::<Align<1048576>>() == 0,
        "Align(1048576) is expected to be of size 0"
    );
    assert!(
        align_of::<Align<2097152>>() == 2097152,
        "Align(2097152) is expected to be of alignment 2097152"
    );
    assert!(
        size_of::<Align<2097152>>() == 0,
        "Align(2097152) is expected to be of size 0"
    );
    assert!(
        align_of::<Align<4194304>>() == 4194304,
        "Align(4194304) is expected to be of alignment 4194304"
    );
    assert!(
        size_of::<Align<4194304>>() == 0,
        "Align(4194304) is expected to be of size 0"
    );
    assert!(
        align_of::<Align<8388608>>() == 8388608,
        "Align(8388608) is expected to be of alignment 8388608"
    );
    assert!(
        size_of::<Align<8388608>>() == 0,
        "Align(8388608) is expected to be of size 0"
    );
    assert!(
        align_of::<Align<16777216>>() == 16777216,
        "Align(16777216) is expected to be of alignment 16777216"
    );
    assert!(
        size_of::<Align<16777216>>() == 0,
        "Align(16777216) is expected to be of size 0"
    );
    assert!(
        align_of::<Align<33554432>>() == 33554432,
        "Align(33554432) is expected to be of alignment 33554432"
    );
    assert!(
        size_of::<Align<33554432>>() == 0,
        "Align(33554432) is expected to be of size 0"
    );
    assert!(
        align_of::<Align<67108864>>() == 67108864,
        "Align(67108864) is expected to be of alignment 67108864"
    );
    assert!(
        size_of::<Align<67108864>>() == 0,
        "Align(67108864) is expected to be of size 0"
    );
    assert!(
        align_of::<Align<134217728>>() == 134217728,
        "Align(134217728) is expected to be of alignment 134217728"
    );
    assert!(
        size_of::<Align<134217728>>() == 0,
        "Align(134217728) is expected to be of size 0"
    );
    assert!(
        align_of::<Align<268435456>>() == 268435456,
        "Align(268435456) is expected to be of alignment 268435456"
    );
    assert!(
        size_of::<Align<268435456>>() == 0,
        "Align(268435456) is expected to be of size 0"
    );
    assert!(
        align_of::<Align<536870912>>() == 536870912,
        "Align(536870912) is expected to be of alignment 536870912"
    );
    assert!(
        size_of::<Align<536870912>>() == 0,
        "Align(536870912) is expected to be of size 0"
    );
};
