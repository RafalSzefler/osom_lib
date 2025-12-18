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
    if end <= start {
        if end == start {
            return start;
        }
        panic!("You cannot use range with end smaller than start.");
    }
    let diff = end - start;
    let mask = if diff > (1 << 31) {
        u32::MAX
    } else {
        diff.next_power_of_two() - 1
    };
    let value = loop {
        let tmp_value = generator.generate::<u32>() & mask;
        if tmp_value < diff {
            break tmp_value;
        }
    };
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
    if end <= start {
        if end == start {
            return start;
        }
        panic!("You cannot use range with end smaller than start.");
    }
    let diff = end - start;
    let mask = if diff > (1 << 63) {
        u64::MAX
    } else {
        diff.next_power_of_two() - 1
    };
    let value = loop {
        let tmp_value = generator.generate::<u64>() & mask;
        if tmp_value < diff {
            break tmp_value;
        }
    };
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
    if end <= start {
        if end == start {
            return start;
        }
        panic!("You cannot use range with end smaller than start.");
    }
    let diff = end.wrapping_sub(start) as u32;
    let mask = if diff > (1 << 31) {
        u32::MAX
    } else {
        diff.next_power_of_two() - 1
    };
    let value = loop {
        let tmp_value = (generator.generate::<i32>() as u32) & mask;
        if tmp_value < diff {
            break tmp_value;
        }
    };

    value.wrapping_add(start as u32) as i32
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
    if end <= start {
        if end == start {
            return start;
        }
        panic!("You cannot use range with end smaller than start.");
    }
    let diff = (end - start) as u64;
    let mask = if diff > (1 << 63) {
        u64::MAX
    } else {
        diff.next_power_of_two() - 1
    };
    let value = loop {
        let tmp_value = (generator.generate::<i64>() as u64) & mask;
        if tmp_value < diff {
            break tmp_value;
        }
    };
    value.wrapping_add(start as u64) as i64
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

    #[allow(clippy::float_cmp)]
    if end <= start {
        if end == start {
            return start;
        }
        panic!("You cannot use range with end smaller than start.");
    }

    union UF32 {
        f: f32,
        u: u32,
    }

    let diff = end - start;

    #[allow(clippy::float_cmp)]
    loop {
        #[allow(clippy::cast_precision_loss)]
        const INV: f32 = 1.0 / (1u32 << 24) as f32;
        let value = generator.generate::<u32>() & ((1u32 << 25) - 1);
        let uf32 = UF32 { u: value };
        let chosen_f = unsafe { uf32.f } * INV;
        if start_excluded && chosen_f == 0.0 {
            continue;
        }
        if end_included && chosen_f == 1.0 {
            continue;
        }
        return chosen_f * diff + start;
    }
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

    #[allow(clippy::float_cmp)]
    if end <= start {
        if end == start {
            return start;
        }
        panic!("You cannot use range with end smaller than start.");
    }

    union UF32 {
        f: f64,
        u: u64,
    }

    let diff = end - start;

    #[allow(clippy::float_cmp)]
    loop {
        #[allow(clippy::cast_precision_loss)]
        const INV: f64 = 1.0 / (1u64 << 53) as f64;
        let value = generator.generate::<u64>() & ((1u64 << 54) - 1);
        let uf32 = UF32 { u: value };
        let chosen_f = unsafe { uf32.f } * INV;
        if start_excluded && chosen_f == 0.0 {
            continue;
        }
        if end_included && chosen_f == 1.0 {
            continue;
        }
        return chosen_f * diff + start;
    }
}
