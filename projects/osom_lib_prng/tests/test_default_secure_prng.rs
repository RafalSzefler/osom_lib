use osom_lib_prng::{defaults::DefaultSecurePRNG, traits::Seedable};

#[cfg(not(miri))]
use osom_lib_prng::traits::PRNGenerator as _;
#[cfg(not(miri))]
use osom_lib_prng::traits::Splittable as _;

use rstest::rstest;

mod helpers;

#[rstest]
#[case(2)]
#[case(124)]
#[case(635)]
#[case(15431212346)]
#[case(65435431378659610013)]
fn test_default_prng_in_range(#[case] seed: u128) {
    let builder = || DefaultSecurePRNG::with_seed(seed);
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
fn test_default_prng_statistics(#[case] seed: u128) {
    let mut lcg = DefaultSecurePRNG::with_seed(seed);
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
fn test_default_prng_statistics_2(#[case] seed: u128) {
    let mut lcg = DefaultSecurePRNG::with_seed(seed);
    let mut lcg2 = DefaultSecurePRNG::with_seed(seed + 5);
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
fn test_default_prng_statistics_split(#[case] seed: u128) {
    let mut lcg = DefaultSecurePRNG::with_seed(seed);
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
fn test_default_prng_statistics_split2(#[case] seed: u128) {
    let mut lcg = DefaultSecurePRNG::with_seed(seed);
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
fn test_default_prng_statistics_split3(#[case] seed: u128) {
    let mut lcg = DefaultSecurePRNG::with_seed(seed);
    let mut lcg2 = lcg.split();
    let mut lcg3 = lcg2.split();
    let test = helpers::statistical_tests::StatisticalTest::builder().build();
    test.test_3d(
        || lcg.generate::<u32>(),
        || lcg2.generate::<u32>(),
        || lcg3.generate::<u32>(),
    );
}
