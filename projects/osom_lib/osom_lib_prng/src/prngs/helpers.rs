#![allow(clippy::cast_possible_wrap, clippy::cast_sign_loss, clippy::items_after_statements)]

use core::ops::{Bound, RangeBounds};

use crate::traits::{PRNConcreteGenerator, PRNGenerator};

#[inline]
pub fn fill_raw_from_array_generator<const TSIZE: usize, TGen>(mut gene: TGen, dst_ptr: *mut u8, dst_len: usize)
where
    TGen: FnMut() -> [u8; TSIZE],
{
    let mut len = dst_len;
    let mut ptr = dst_ptr;
    while len >= TSIZE {
        let value = gene();
        let value_ptr = (&raw const value).cast();
        unsafe {
            ptr.copy_from_nonoverlapping(value_ptr, TSIZE);
            ptr = ptr.add(TSIZE);
        }
        len -= TSIZE;
    }

    if len > 0 {
        let value = gene();
        let value_ptr = (&raw const value).cast();
        unsafe {
            ptr.copy_from_nonoverlapping(value_ptr, len);
        }
    }
}

const CRC8_TABLE: &[u8] = &[
    0, 7, 14, 9, 28, 27, 18, 21, 56, 63, 54, 49, 36, 35, 42, 45, 112, 119, 126, 121, 108, 107, 98, 101, 72, 79, 70, 65,
    84, 83, 90, 93, 224, 231, 238, 233, 252, 251, 242, 245, 216, 223, 214, 209, 196, 195, 202, 205, 144, 151, 158, 153,
    140, 139, 130, 133, 168, 175, 166, 161, 180, 179, 186, 189, 199, 192, 201, 206, 219, 220, 213, 210, 255, 248, 241,
    246, 227, 228, 237, 234, 183, 176, 185, 190, 171, 172, 165, 162, 143, 136, 129, 134, 147, 148, 157, 154, 39, 32, 41,
    46, 59, 60, 53, 50, 31, 24, 17, 22, 3, 4, 13, 10, 87, 80, 89, 94, 75, 76, 69, 66, 111, 104, 97, 102, 115, 116, 125,
    122, 137, 142, 135, 128, 149, 146, 155, 156, 177, 182, 191, 184, 173, 170, 163, 164, 249, 254, 247, 240, 229, 226,
    235, 236, 193, 198, 207, 200, 221, 218, 211, 212, 105, 110, 103, 96, 117, 114, 123, 124, 81, 86, 95, 88, 77, 74, 67,
    68, 25, 30, 23, 16, 5, 2, 11, 12, 33, 38, 47, 40, 61, 58, 51, 52, 78, 73, 64, 71, 82, 85, 92, 91, 118, 113, 120,
    127, 106, 109, 100, 99, 62, 57, 48, 55, 34, 37, 44, 43, 6, 1, 8, 15, 26, 29, 20, 19, 174, 169, 160, 167, 178, 181,
    188, 187, 150, 145, 152, 159, 138, 141, 132, 131, 222, 217, 208, 215, 194, 197, 204, 203, 230, 225, 232, 239, 250,
    253, 244, 243,
];
pub const fn calculate_crc8(arr: &[u8]) -> u8 {
    let mut result = 0u8;
    let mut arr_ptr = arr.as_ptr();
    let mut len = arr.len();
    while len > 0 {
        let byte = unsafe { arr_ptr.read() };
        let crc_index = byte ^ result;
        result = CRC8_TABLE[crc_index as usize];
        arr_ptr = unsafe { arr_ptr.add(1) };
        len -= 1;
    }
    result
}

#[inline]
fn sample_u32_below<TGen: PRNGenerator>(generator: &mut TGen, upper_exclusive: u64) -> u32
where
    u32: PRNConcreteGenerator<TGen>,
{
    assert!(upper_exclusive > 0, "upper_exclusive must be positive");
    if upper_exclusive > (u64::from(u32::MAX)) {
        debug_assert!(upper_exclusive == u64::from(u32::MAX) + 1);
        return generator.generate::<u32>();
    }

    #[allow(clippy::cast_possible_truncation)]
    let mask = (upper_exclusive.next_power_of_two() - 1) as u32;
    loop {
        let value = generator.generate::<u32>() & mask;
        if u64::from(value) < upper_exclusive {
            return value;
        }
    }
}

#[inline]
fn sample_u64_below<TGen: PRNGenerator>(generator: &mut TGen, upper_exclusive: u128) -> u64
where
    u64: PRNConcreteGenerator<TGen>,
{
    assert!(upper_exclusive > 0, "upper_exclusive must be positive");
    if upper_exclusive > (u128::from(u64::MAX)) {
        debug_assert!(upper_exclusive == u128::from(u64::MAX) + 1);
        return generator.generate::<u64>();
    }

    #[allow(clippy::cast_possible_truncation)]
    let mask = (upper_exclusive.next_power_of_two() - 1) as u64;
    loop {
        let value = generator.generate::<u64>() & mask;
        if u128::from(value) < upper_exclusive {
            return value;
        }
    }
}

#[inline]
fn sample_unit_f32<TGen: PRNGenerator>(generator: &mut TGen, start_excluded: bool, end_included: bool) -> f32
where
    u32: PRNConcreteGenerator<TGen>,
{
    const MANTISSA_BITS: u32 = 24;
    const MAX_VALUE: u32 = (1u32 << MANTISSA_BITS) - 1;

    #[allow(clippy::cast_precision_loss)]
    const INV_EXCLUSIVE: f32 = 1.0 / ((1u32 << MANTISSA_BITS) as f32);
    #[allow(clippy::cast_precision_loss)]
    const INV_INCLUSIVE: f32 = 1.0 / (MAX_VALUE as f32);

    loop {
        let value = generator.generate::<u32>() >> (u32::BITS - MANTISSA_BITS);
        if start_excluded && value == 0 {
            continue;
        }
        #[allow(clippy::cast_precision_loss)]
        if end_included {
            return (value as f32) * INV_INCLUSIVE;
        }
        #[allow(clippy::cast_precision_loss)]
        return (value as f32) * INV_EXCLUSIVE;
    }
}

#[inline]
fn sample_unit_f64<TGen: PRNGenerator>(generator: &mut TGen, start_excluded: bool, end_included: bool) -> f64
where
    u64: PRNConcreteGenerator<TGen>,
{
    const MANTISSA_BITS: u32 = 53;
    const MAX_VALUE: u64 = (1u64 << MANTISSA_BITS) - 1;

    #[allow(clippy::cast_precision_loss)]
    const INV_EXCLUSIVE: f64 = 1.0 / ((1u64 << MANTISSA_BITS) as f64);
    #[allow(clippy::cast_precision_loss)]
    const INV_INCLUSIVE: f64 = 1.0 / (MAX_VALUE as f64);

    loop {
        let value = generator.generate::<u64>() >> (u64::BITS - MANTISSA_BITS);
        if start_excluded && value == 0 {
            continue;
        }
        #[allow(clippy::cast_precision_loss)]
        if end_included {
            return (value as f64) * INV_INCLUSIVE;
        }
        #[allow(clippy::cast_precision_loss)]
        return (value as f64) * INV_EXCLUSIVE;
    }
}

pub fn generate_u32_in_range<TGen: PRNGenerator, TBounds: RangeBounds<u32>>(generator: &mut TGen, range: TBounds) -> u32
where
    u32: PRNConcreteGenerator<TGen>,
{
    let start = match range.start_bound() {
        Bound::Included(val) => *val,
        Bound::Excluded(val) => {
            let val = *val;
            assert!(val < u32::MAX, "You cannot make u32::MAX as excluded start_bound.");
            val + 1
        }
        Bound::Unbounded => 0,
    };
    let end = match range.end_bound() {
        Bound::Included(val) => *val,
        Bound::Excluded(val) => {
            let val = *val;
            assert!(val > 0, "You cannot make 0 as excluded end_bound");
            val - 1
        }
        Bound::Unbounded => u32::MAX,
    };
    assert!(end >= start, "You cannot use range with end smaller than start.");

    let value = sample_u32_below(generator, u64::from(end) - u64::from(start) + 1);
    value + start
}

pub fn generate_u64_in_range<TGen: PRNGenerator, TBounds: RangeBounds<u64>>(generator: &mut TGen, range: TBounds) -> u64
where
    u64: PRNConcreteGenerator<TGen>,
{
    let start = match range.start_bound() {
        Bound::Included(val) => *val,
        Bound::Excluded(val) => {
            let val = *val;
            assert!(val < u64::MAX, "You cannot make u64::MAX as excluded start_bound.");
            val + 1
        }
        Bound::Unbounded => 0,
    };
    let end = match range.end_bound() {
        Bound::Included(val) => *val,
        Bound::Excluded(val) => {
            let val = *val;
            assert!(val > 0, "You cannot make 0 as excluded end_bound");
            val - 1
        }
        Bound::Unbounded => u64::MAX,
    };
    assert!(end >= start, "You cannot use range with end smaller than start.");

    let value = sample_u64_below(generator, u128::from(end) - u128::from(start) + 1);
    value + start
}

pub fn generate_i32_in_range<TGen: PRNGenerator, TBounds: RangeBounds<i32>>(generator: &mut TGen, range: TBounds) -> i32
where
    i32: PRNConcreteGenerator<TGen>,
{
    let start = match range.start_bound() {
        Bound::Included(val) => *val,
        Bound::Excluded(val) => {
            let val = *val;
            assert!(val < i32::MAX, "You cannot make i32::MAX as excluded start_bound.");
            val + 1
        }
        Bound::Unbounded => i32::MIN,
    };
    let end = match range.end_bound() {
        Bound::Included(val) => *val,
        Bound::Excluded(val) => {
            let val = *val;
            assert!(val > i32::MIN, "You cannot make i32::MIN as excluded end_bound");
            val - 1
        }
        Bound::Unbounded => i32::MAX,
    };
    assert!(end >= start, "You cannot use range with end smaller than start.");

    let upper_exclusive = (i64::from(end) - i64::from(start) + 1) as u64;
    let value = if upper_exclusive > u64::from(u32::MAX) {
        debug_assert!(upper_exclusive == u64::from(u32::MAX) + 1);
        generator.generate::<i32>() as u32
    } else {
        #[allow(clippy::cast_possible_truncation)]
        let mask = (upper_exclusive.next_power_of_two() - 1) as u32;
        loop {
            let value = (generator.generate::<i32>() as u32) & mask;
            if u64::from(value) < upper_exclusive {
                break value;
            }
        }
    };
    let value = i64::from(value);
    i32::try_from(i64::from(start) + value).expect("generated i32 range result out of bounds")
}

pub fn generate_i64_in_range<TGen: PRNGenerator, TBounds: RangeBounds<i64>>(generator: &mut TGen, range: TBounds) -> i64
where
    i64: PRNConcreteGenerator<TGen>,
{
    let start = match range.start_bound() {
        Bound::Included(val) => *val,
        Bound::Excluded(val) => {
            let val = *val;
            assert!(val < i64::MAX, "You cannot make i64::MAX as excluded start_bound.");
            val + 1
        }
        Bound::Unbounded => i64::MIN,
    };
    let end = match range.end_bound() {
        Bound::Included(val) => *val,
        Bound::Excluded(val) => {
            let val = *val;
            assert!(val > i64::MIN, "You cannot make i64::MIN as excluded end_bound");
            val - 1
        }
        Bound::Unbounded => i64::MAX,
    };
    assert!(end >= start, "You cannot use range with end smaller than start.");

    let upper_exclusive = (i128::from(end) - i128::from(start) + 1) as u128;
    let value = if upper_exclusive > u128::from(u64::MAX) {
        debug_assert!(upper_exclusive == u128::from(u64::MAX) + 1);
        generator.generate::<i64>() as u64
    } else {
        #[allow(clippy::cast_possible_truncation)]
        let mask = (upper_exclusive.next_power_of_two() - 1) as u64;
        loop {
            let value = (generator.generate::<i64>() as u64) & mask;
            if u128::from(value) < upper_exclusive {
                break value;
            }
        }
    };
    let value = i128::from(value);
    i64::try_from(i128::from(start) + value).expect("generated i64 range result out of bounds")
}

pub fn generate_f32_in_range<TGen: PRNGenerator, TBounds: RangeBounds<f32>>(generator: &mut TGen, range: TBounds) -> f32
where
    u32: PRNConcreteGenerator<TGen>,
{
    let mut start_excluded = false;
    let start = match range.start_bound() {
        Bound::Included(val) => *val,
        Bound::Excluded(val) => {
            start_excluded = true;
            *val
        }
        Bound::Unbounded => f32::MIN,
    };

    let mut end_included = false;
    let end = match range.end_bound() {
        Bound::Included(val) => {
            end_included = true;
            *val
        }
        Bound::Excluded(val) => *val,
        Bound::Unbounded => f32::MAX,
    };

    assert!(end >= start, "You cannot use range with end smaller than start.");
    #[allow(clippy::float_cmp)]
    if end == start {
        assert!(!start_excluded && end_included, "You cannot use empty range.");
        return start;
    }

    let diff = end - start;

    let chosen_f = sample_unit_f32(generator, start_excluded, end_included);
    chosen_f * diff + start
}

pub fn generate_f64_in_range<TGen: PRNGenerator, TBounds: RangeBounds<f64>>(generator: &mut TGen, range: TBounds) -> f64
where
    u64: PRNConcreteGenerator<TGen>,
{
    let mut start_excluded = false;
    let start = match range.start_bound() {
        Bound::Included(val) => *val,
        Bound::Excluded(val) => {
            start_excluded = true;
            *val
        }
        Bound::Unbounded => f64::MIN,
    };

    let mut end_included = false;
    let end = match range.end_bound() {
        Bound::Included(val) => {
            end_included = true;
            *val
        }
        Bound::Excluded(val) => *val,
        Bound::Unbounded => f64::MAX,
    };

    assert!(end >= start, "You cannot use range with end smaller than start.");
    #[allow(clippy::float_cmp)]
    if end == start {
        assert!(!start_excluded && end_included, "You cannot use empty range.");
        return start;
    }

    let diff = end - start;

    let chosen_f = sample_unit_f64(generator, start_excluded, end_included);
    chosen_f * diff + start
}

#[cfg(test)]
mod tests {
    use osom_lib_reprc::macros::reprc;

    use super::*;

    #[reprc]
    #[derive(Clone, Debug)]
    struct TestGenerator {
        values32: [u32; 4],
        values32_len: usize,
        values32_idx: usize,
        values64: [u64; 4],
        values64_len: usize,
        values64_idx: usize,
    }

    impl TestGenerator {
        fn from_u32<const N: usize>(values: [u32; N]) -> Self {
            assert!(N <= 4);
            let mut result = Self {
                values32: [0; 4],
                values32_len: N,
                values32_idx: 0,
                values64: [0; 4],
                values64_len: 0,
                values64_idx: 0,
            };
            result.values32[..N].copy_from_slice(&values);
            result
        }

        fn from_u64<const N: usize>(values: [u64; N]) -> Self {
            assert!(N <= 4);
            let mut result = Self {
                values32: [0; 4],
                values32_len: 0,
                values32_idx: 0,
                values64: [0; 4],
                values64_len: N,
                values64_idx: 0,
            };
            result.values64[..N].copy_from_slice(&values);
            result
        }
    }

    impl PRNGenerator for TestGenerator {
        unsafe fn fill_raw(&mut self, _: *mut u8, _: usize) {
            unreachable!("typed generation is used in tests");
        }
    }

    impl PRNConcreteGenerator<TestGenerator> for u32 {
        fn generate(generator: &mut TestGenerator) -> Self {
            assert!(generator.values32_idx < generator.values32_len);
            let value = generator.values32[generator.values32_idx];
            generator.values32_idx += 1;
            value
        }
    }

    impl PRNConcreteGenerator<TestGenerator> for i32 {
        fn generate(generator: &mut TestGenerator) -> Self {
            <u32 as PRNConcreteGenerator<TestGenerator>>::generate(generator) as i32
        }
    }

    impl PRNConcreteGenerator<TestGenerator> for u64 {
        fn generate(generator: &mut TestGenerator) -> Self {
            assert!(generator.values64_idx < generator.values64_len);
            let value = generator.values64[generator.values64_idx];
            generator.values64_idx += 1;
            value
        }
    }

    impl PRNConcreteGenerator<TestGenerator> for i64 {
        fn generate(generator: &mut TestGenerator) -> Self {
            <u64 as PRNConcreteGenerator<TestGenerator>>::generate(generator) as i64
        }
    }

    #[test]
    fn test_generate_u32_in_range_reaches_exclusive_upper_bound() {
        let mut generator = TestGenerator::from_u32([1]);
        assert_eq!(generate_u32_in_range(&mut generator, 10u32..12u32), 11);
    }

    #[test]
    fn test_generate_i32_in_range_reaches_inclusive_upper_bound() {
        let mut generator = TestGenerator::from_u32([2]);
        assert_eq!(generate_i32_in_range(&mut generator, -2i32..=0i32), 0);
    }

    #[test]
    fn test_generate_f32_in_range_can_return_inclusive_end() {
        let mut generator = TestGenerator::from_u32([u32::MAX]);
        let value = generate_f32_in_range(&mut generator, 0.0f32..=1.0f32);
        assert!((value - 1.0).abs() <= f32::EPSILON);
    }

    #[test]
    fn test_generate_f64_in_range_can_return_inclusive_end() {
        let mut generator = TestGenerator::from_u64([u64::MAX]);
        let value = generate_f64_in_range(&mut generator, 0.0f64..=1.0f64);
        assert!((value - 1.0).abs() <= f64::EPSILON);
    }

    #[test]
    fn test_generate_f64_in_range_keeps_exclusive_end_excluded() {
        let mut generator = TestGenerator::from_u64([u64::MAX]);
        assert!(generate_f64_in_range(&mut generator, 0.0f64..1.0f64) < 1.0);
    }
}
