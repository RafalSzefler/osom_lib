use osom_lib_prng::prngs::SplitMix64;

#[cfg(not(miri))]
use osom_lib_prng::traits::PRNGenerator as _;

use rstest::rstest;

mod helpers;

#[cfg(not(miri))]
#[rstest]
#[case(1)]
#[case(123)]
#[case(634)]
#[case(72345601)]
#[case(15431212345)]
fn test_splitmix_statistics(#[case] seed: u64) {
    let mut sm = SplitMix64::with_seed(seed);
    let test = helpers::statistical_tests::StatisticalTest::builder().build();
    test.test_1d(|| sm.generate::<u32>());
}

#[cfg(not(miri))]
#[rstest]
#[case(2)]
#[case(124)]
#[case(635)]
#[case(72345600)]
#[case(15431212346)]
fn test_splitmix_statistics_2(#[case] seed: u64) {
    let mut sm = SplitMix64::with_seed(seed);
    let mut sm2 = SplitMix64::with_seed(seed + 5);
    let test = helpers::statistical_tests::StatisticalTest::builder().build();
    test.test_2d(|| sm.generate::<u32>(), || sm2.generate::<u32>());
}

#[cfg(not(miri))]
#[rstest]
#[case(2)]
#[case(124)]
#[case(635)]
#[case(72345600)]
#[case(15431212346)]
fn test_splitmix_statistics_3(#[case] seed: u64) {
    let mut sm = SplitMix64::with_seed(seed);
    let mut sm2 = SplitMix64::with_seed(seed + 5);
    let mut sm3 = SplitMix64::with_seed(seed + 10);
    let test = helpers::statistical_tests::StatisticalTest::builder().build();
    test.test_3d(
        || sm.generate::<u32>(),
        || sm2.generate::<u32>(),
        || sm3.generate::<u32>(),
    );
}

#[rstest]
#[case(2)]
#[case(124)]
#[case(635)]
#[case(72345600)]
#[case(15431212346)]
fn test_splitmix_in_range(#[case] seed: u64) {
    let builder = || SplitMix64::with_seed(seed);
    helpers::basic_tests::test_in_range_u32(builder);
    helpers::basic_tests::test_in_range_u64(builder);
    helpers::basic_tests::test_in_range_i32(builder);
    helpers::basic_tests::test_in_range_i64(builder);
    helpers::basic_tests::test_in_range_f32(builder);
    helpers::basic_tests::test_in_range_f64(builder);
}
