use osom_lib_macros::debug_check_or_release_hint;
use osom_lib_reprc::macros::reprc;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[reprc]
#[repr(transparent)]
pub struct ControlByte {
    value: u8,
}

#[rustfmt::skip]
mod magic {
    pub const EMPTY_VALUE: u8       = 0b11111111;
    pub const RESERVED_VALUE: u8    = 0b11111110;
    pub const DISTANCE_MASK: u8     = 0b01111111;
    pub const DIRECT_HIT_MASK: u8   = 0b10000000;
    pub const DIRECT_HIT_VALUE: u8  = 0b00000000;
    pub const STORAGE_VALUE: u8     = 0b10000000;
}

impl ControlByte {
    pub const EMPTY: Self = Self {
        value: magic::EMPTY_VALUE,
    };

    pub const NEW_TAIL: Self = Self {
        value: magic::STORAGE_VALUE,
    };

    pub const NEW_DIRECT_HIT: Self = Self {
        value: magic::DIRECT_HIT_VALUE,
    };

    pub const RESERVED: Self = Self {
        value: magic::RESERVED_VALUE,
    };

    #[inline(always)]
    pub const fn distance_index(self) -> usize {
        (self.value & magic::DISTANCE_MASK) as usize
    }

    #[inline(always)]
    pub const fn set_distance_index(&mut self, value: u8) {
        debug_check_or_release_hint!(value <= magic::DISTANCE_MASK);
        self.value = (self.value & (!magic::DISTANCE_MASK)) | value;
    }

    #[inline(always)]
    pub const fn is_direct_hit(self) -> bool {
        (self.value & magic::DIRECT_HIT_MASK) == magic::DIRECT_HIT_VALUE
    }

    #[inline(always)]
    pub const fn contains_data(self) -> bool {
        let v = self.value;
        (v != magic::EMPTY_VALUE) && (v != magic::RESERVED_VALUE)
    }

    #[inline(always)]
    pub const fn binary_value(self) -> u8 {
        self.value
    }
}
