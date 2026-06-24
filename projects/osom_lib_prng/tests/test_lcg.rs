use osom_lib_prng::prngs::LinearCongruentialGenerator128;
use osom_lib_prng::traits::{PRNGenerator as _, Seedable as _};

#[cfg(not(miri))]
use osom_lib_prng::traits::Splittable as _;

use rstest::rstest;

mod helpers;

#[rstest]
#[case(100, &[76, 67, 71, 4, 252, 129, 76, 150, 215, 94, 234, 186, 251, 29, 242, 90, 237, 40, 20, 205])]
#[case(101, &[76, 67, 71, 159, 221, 80, 217, 12, 237, 131, 172, 196, 81, 252, 35, 87, 202, 5, 170, 86])]
#[case(112, &[76, 67, 71, 231, 136, 52, 230, 36, 217, 27, 4, 48, 4, 138, 72, 45, 73, 131, 26, 64])]
fn test_lcg_serialization(#[case] seed: u128, #[case] expected: &[u8]) {
    let mut gene = LinearCongruentialGenerator128::with_seed(seed);
    for _ in 0..1000 {
        let _ = gene.generate::<u128>();
    }
    helpers::basic_tests::test_prng_serialization(expected, gene);
}

#[rstest]
#[case(&[76, 67, 71, 4, 252, 129, 76, 150, 215, 94, 234, 186, 251, 29, 242, 90, 237, 40, 20, 205], &[2886042978282324117, 9534460276493811490, 9946567228286987443])]
#[case(&[76, 67, 71, 231, 136, 52, 230, 36, 217, 27, 4, 48, 4, 138, 72, 45, 73, 131, 26, 64], &[821111466127046161, 1456313996308460110, 9913780813794144079])]
fn test_lcg_deserialization(#[case] given: &[u8], #[case] nexts: &[u64]) {
    helpers::basic_tests::test_prng_deserialization::<LinearCongruentialGenerator128>(given, nexts);
}

#[rstest]
#[case(2)]
#[case(124)]
#[case(635)]
#[case(15431212346)]
#[case(65435431378659610013)]
fn test_lcg_in_range(#[case] seed: u128) {
    let builder = || LinearCongruentialGenerator128::with_seed(seed);
    helpers::basic_tests::test_in_range_u32(builder);
    helpers::basic_tests::test_in_range_u64(builder);
    helpers::basic_tests::test_in_range_i32(builder);
    helpers::basic_tests::test_in_range_i64(builder);
    helpers::basic_tests::test_in_range_f32(builder);
    helpers::basic_tests::test_in_range_f64(builder);
}

#[cfg(not(miri))]
#[rstest]
#[case(1)]
#[case(123)]
#[case(634)]
#[case(15431212345)]
#[case(65435431378659610012)]
fn test_lcg_statistics(#[case] seed: u128) {
    let mut lcg = LinearCongruentialGenerator128::with_seed(seed);
    let test = helpers::statistical_tests::StatisticalTest::builder().build();
    test.test_1d(|| lcg.generate::<u32>());
}

#[cfg(not(miri))]
#[rstest]
#[case(2)]
#[case(124)]
#[case(635)]
#[case(15431212346)]
#[case(65435431378659610013)]
fn test_lcg_statistics_2(#[case] seed: u128) {
    let mut lcg = LinearCongruentialGenerator128::with_seed(seed);
    let mut lcg2 = LinearCongruentialGenerator128::with_seed(seed + 5);
    let test = helpers::statistical_tests::StatisticalTest::builder().build();
    test.test_2d(|| lcg.generate::<u32>(), || lcg2.generate::<u32>());
}

#[cfg(not(miri))]
#[rstest]
#[case(2)]
#[case(124)]
#[case(635)]
#[case(15431212346)]
#[case(65435431378659610013)]
fn test_lcg_statistics_split(#[case] seed: u128) {
    let mut lcg = LinearCongruentialGenerator128::with_seed(seed);
    let mut lcg2 = lcg.split();
    let test = helpers::statistical_tests::StatisticalTest::builder().build();
    test.test_2d(|| lcg.generate::<u32>(), || lcg2.generate::<u32>());
}

#[cfg(not(miri))]
#[rstest]
#[case(2)]
#[case(124)]
#[case(635)]
#[case(15431212346)]
#[case(65435431378659610013)]
fn test_lcg_statistics_split2(#[case] seed: u128) {
    let mut lcg = LinearCongruentialGenerator128::with_seed(seed);
    let mut lcg2 = lcg.split();
    let mut lcg3 = lcg.split();
    let test = helpers::statistical_tests::StatisticalTest::builder().build();
    test.test_3d(
        || lcg.generate::<u32>(),
        || lcg2.generate::<u32>(),
        || lcg3.generate::<u32>(),
    );
}

#[cfg(not(miri))]
#[rstest]
#[case(2)]
#[case(124)]
#[case(635)]
#[case(15431212346)]
#[case(65435431378659610013)]
fn test_lcg_statistics_split3(#[case] seed: u128) {
    let mut lcg = LinearCongruentialGenerator128::with_seed(seed);
    let mut lcg2 = lcg.split();
    let mut lcg3 = lcg2.split();
    let test = helpers::statistical_tests::StatisticalTest::builder().build();
    test.test_3d(
        || lcg.generate::<u32>(),
        || lcg2.generate::<u32>(),
        || lcg3.generate::<u32>(),
    );
}
