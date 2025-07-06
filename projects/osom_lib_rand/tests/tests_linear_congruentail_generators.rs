use rstest::rstest;

mod common;

use osom_lib_rand::pseudo_random_generators::LinearCongruentialGenerator;
use osom_lib_rand::randomness_sources::OsRandomnessSource;
use osom_lib_rand::traits::PseudoRandomGenerator as _;

#[rstest]
#[case(1)]
#[case(2)]
#[case(3)]
#[case(13212)]
#[case(4563221)]
#[case(u32::MAX - 2)]
#[case(u32::MAX - 1)]
#[case(u32::MAX)]
fn test_statistical_properties_lcg_u32(#[case] initial: u32) {
    let mut generator = LinearCongruentialGenerator::<u32>::new(initial);
    common::test_statistical_properties(|| generator.next_value());
    common::test_fill_bytes::<u32, _>(|bytes| generator.fill_bytes(bytes));
}

#[cfg(feature = "std_rand")]
#[test]
fn test_statistical_properties_lcg_u32_with_random_seed() {
    let mut os_rand = OsRandomnessSource::default();
    let mut generator = LinearCongruentialGenerator::<u32>::from_randomness_source(&mut os_rand);
    common::test_statistical_properties(|| generator.next_value());
    common::test_fill_bytes::<u32, _>(|bytes| generator.fill_bytes(bytes));
}

#[rstest]
#[case(1)]
#[case(2)]
#[case(3)]
#[case(13212)]
#[case(4563221)]
#[case(9090567890)]
#[case(u64::MAX - 2)]
#[case(u64::MAX - 1)]
#[case(u64::MAX)]
fn test_statistical_properties_lcg_u64(#[case] initial: u64) {
    let mut generator = LinearCongruentialGenerator::<u64>::new(initial);
    common::test_statistical_properties(|| generator.next_value());
    common::test_fill_bytes::<u64, _>(|bytes| generator.fill_bytes(bytes));
}

#[cfg(feature = "std_rand")]
#[test]
fn test_statistical_properties_lcg_u64_with_random_seed() {
    let mut os_rand = OsRandomnessSource::<u64>::default();
    let mut generator = LinearCongruentialGenerator::<u64>::from_randomness_source(&mut os_rand);
    common::test_statistical_properties(|| generator.next_value());
    common::test_fill_bytes::<u64, _>(|bytes| generator.fill_bytes(bytes));
}

#[rstest]
#[case(1)]
#[case(2)]
#[case(3)]
#[case(13212)]
#[case(4563221)]
#[case(9090567890)]
#[case(713243213243217654322541)]
#[case(u128::MAX - 2)]
#[case(u128::MAX - 1)]
#[case(u128::MAX)]
fn test_statistical_properties_lcg_u128(#[case] initial: u128) {
    let mut generator = LinearCongruentialGenerator::<u128>::new(initial);
    common::test_statistical_properties(|| generator.next_value());
    common::test_fill_bytes::<u128, _>(|bytes| generator.fill_bytes(bytes));
}

#[cfg(feature = "std_rand")]
#[test]
fn test_statistical_properties_lcg_u128_with_random_seed() {
    let mut os_rand = OsRandomnessSource::<u64>::default();
    let mut generator = LinearCongruentialGenerator::<u128>::from_randomness_source(&mut os_rand);
    common::test_statistical_properties(|| generator.next_value());
    common::test_fill_bytes::<u128, _>(|bytes| generator.fill_bytes(bytes));
}
