use osom_lib_reprc::macros::reprc;

/// This struct holds a single unsigned value and iterates
/// over indexes for which this value has the corresponding bit set.
///
/// The iteration is always performed from higher to lower bits.
#[repr(transparent)]
#[reprc]
#[must_use]
pub struct SetBitIterator {
    value: u16,
}

impl SetBitIterator {
    #[inline(always)]
    pub const fn new(value: u16) -> Self {
        Self { value }
    }
}

impl Iterator for SetBitIterator {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.value == 0 {
            return None;
        }

        let idx = self.value.trailing_zeros();
        self.value &= unsafe { self.value.unchecked_sub(1) };
        Some(idx as usize)
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(0, &[])]
    #[case(0b101, &[0, 2])]
    #[case(0b110, &[1, 2])]
    #[case(0b11001001, &[0, 3, 6, 7])]
    #[case(0b10000000_00000000, &[15])]
    #[case(0b00010000_00000000, &[12])]
    #[case(0b10000100_00010000, &[4, 10, 15])]
    fn test_iterator(#[case] initial_value: u16, #[case] expected_result: &[usize]) {
        let result: Vec<usize> = SetBitIterator::new(initial_value).collect();
        assert_eq!(result, expected_result);
    }
}
