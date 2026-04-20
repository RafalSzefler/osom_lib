use osom_lib_reprc::macros::reprc;

/// This struct holds a single unsigned value and iterates
/// over indexes for which this value has the corresponding bit set.
///
/// The iteration is always performed from higher to lower bits.
#[repr(transparent)]
#[reprc]
#[must_use]
pub struct SetBitIterator {
    value: u32,
}

impl SetBitIterator {
    #[inline(always)]
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

impl Iterator for SetBitIterator {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.value.leading_zeros();
        if idx == 32 {
            return None;
        }
        self.value = !((!self.value) | (1 << (31 - idx)));
        Some(idx as usize)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(0, &[])]
    #[case(0b101, &[29, 31])]
    #[case(0b110, &[29, 30])]
    #[case(0b11001001, &[24, 25, 28, 31])]
    #[case(0b10000000_00000000_00000000_00000000, &[0])]
    #[case(0b00010000_00000000_00000000_00000000, &[3])]
    #[case(0b10000100_00010000_00000010_01000000, &[0, 5, 11, 22, 25])]
    fn test_iterator(#[case] initial_value: u32, #[case] expected_result: &[usize]) {
        let result: Vec<usize> = SetBitIterator::new(initial_value).collect();
        assert_eq!(result, expected_result);
    }
}
