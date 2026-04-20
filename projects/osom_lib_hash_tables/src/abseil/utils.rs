use osom_lib_numbers::IterTriangular;
use osom_lib_primitives::power_of_two::PowerOfTwo32;

#[inline(always)]
#[allow(clippy::cast_possible_truncation)]
pub fn probe_block_indexes(h1: u64, block_count: PowerOfTwo32) -> impl Iterator<Item = usize> {
    let group_count = block_count.value();
    let mask = u64::from(group_count.wrapping_sub(1));
    IterTriangular::new(group_count).map(move |v| (h1.wrapping_add(u64::from(v)) & mask) as usize)
}
