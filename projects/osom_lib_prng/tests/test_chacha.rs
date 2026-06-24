use osom_lib_prng::{prngs::ChaCha, streams::ChaChaStream, traits::PRNGenerator as _};

#[cfg(not(miri))]
use osom_lib_prng::traits::Splittable as _;

use rstest::rstest;

mod helpers;

#[rstest]
#[case([0; 32], [0; 12])]
#[case([1; 32], [5; 12])]
fn test_chacha_in_range(#[case] key: [u8; 32], #[case] nonce: [u8; 12]) {
    let chacha_stream = ChaChaStream::from_arrays(key, nonce);
    let builder = || ChaCha::<20>::new(chacha_stream);
    helpers::basic_tests::test_in_range_u32(builder);
    helpers::basic_tests::test_in_range_u64(builder);
    helpers::basic_tests::test_in_range_i32(builder);
    helpers::basic_tests::test_in_range_i64(builder);
    helpers::basic_tests::test_in_range_f32(builder);
    helpers::basic_tests::test_in_range_f64(builder);
}

#[rstest]
#[case(1)]
#[case(123)]
#[case(634)]
#[case(72345601)]
#[case(15431212345)]
#[case(65435431378659610012)]
fn test_chacha_in_range_from_seed(#[case] seed: u128) {
    let chacha_stream = ChaChaStream::from_seed(seed);
    let builder = || ChaCha::<20>::new(chacha_stream);
    helpers::basic_tests::test_in_range_u32(builder);
    helpers::basic_tests::test_in_range_u64(builder);
    helpers::basic_tests::test_in_range_i32(builder);
    helpers::basic_tests::test_in_range_i64(builder);
    helpers::basic_tests::test_in_range_f32(builder);
    helpers::basic_tests::test_in_range_f64(builder);
}

#[cfg(not(miri))]
#[rstest]
#[case([0; 32], [0; 12])]
#[case([1; 32], [5; 12])]
fn test_chacha_statistics(#[case] key: [u8; 32], #[case] nonce: [u8; 12]) {
    let chacha_stream = ChaChaStream::from_arrays(key, nonce);
    let mut chacha = ChaCha::<20>::new(chacha_stream);
    let test = helpers::statistical_tests::StatisticalTest::builder().build();
    test.test_1d(|| chacha.generate::<u32>());
}

#[cfg(not(miri))]
#[rstest]
#[case([0; 32])]
#[case([1; 32])]
#[case([5; 32])]
fn test_chacha_statistics_2(#[case] key: [u8; 32]) {
    let chacha_stream = ChaChaStream::from_arrays(key, [0; 12]);
    let mut chacha = ChaCha::<20>::new(chacha_stream);
    let chacha_stream2 = ChaChaStream::from_arrays(key, [5; 12]);
    let mut chacha2 = ChaCha::<20>::new(chacha_stream2);
    let test = helpers::statistical_tests::StatisticalTest::builder().build();
    test.test_2d(|| chacha.generate::<u32>(), || chacha2.generate::<u32>());
}

#[cfg(not(miri))]
#[rstest]
#[case([0; 32], [0; 12])]
#[case([1; 32], [5; 12])]
fn test_chacha_statistics_split(#[case] key: [u8; 32], #[case] nonce: [u8; 12]) {
    let chacha_stream = ChaChaStream::from_arrays(key, nonce);
    let mut chacha = ChaCha::<20>::new(chacha_stream);
    let mut chacha2 = chacha.split();
    let mut chacha3 = chacha2.split();
    let test = helpers::statistical_tests::StatisticalTest::builder().build();
    test.test_3d(
        || chacha.generate::<u32>(),
        || chacha2.generate::<u32>(),
        || chacha3.generate::<u32>(),
    );
}

#[rstest]
#[case(
    "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f",
    "000000090000004a00000000",
    "10f1e7e4d13b5915500fdd1fa32071c4c7d1f4c733c068030422aa9ac3d46c4ed2826446079faa0914c2d705d98b02a2b5129cd1de164eb9cbd083e8a2503c4e"
)]
#[case(
    "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f",
    "000000000000004a00000000",
    "224f51f3401bd9e12fde276fb8631ded8c131f823d2c06e27e4fcaec9ef3cf788a3b0aa372600a92b57974cded2b9334794cba40c63e34cdea212c4cf07d41b7"
)]
#[case(
    "0000000000000000000000000000000000000000000000000000000000000000",
    "000000000000000000000000",
    "9f07e7be5551387a98ba977c732d080dcb0f29a048e3656912c6533e32ee7aed29b721769ce64e43d57133b074d839d531ed1f28510afb45ace10a1f4b794d6f"
)]
#[case(
    "0000000000000000000000000000000000000000000000000000000000000001",
    "000000000000000000000000",
    "3aeb5224ecf849929b9d828db1ced4dd832025e8018b8160b82284f3c949aa5a8eca00bbb4a73bdad192b5c42f73f2fd4e273644c8b36125a64addeb006c13a0"
)]
fn test_chacha_block(#[case] key: &str, #[case] nonce: &str, #[case] expected: &str) {
    let key = helpers::encoding::decode_hex(key);
    assert_eq!(key.len(), 32);
    let nonce = helpers::encoding::decode_hex(nonce);
    assert_eq!(nonce.len(), 12);
    let expected = helpers::encoding::decode_hex(expected);
    assert_eq!(expected.len(), 64);

    let chacha_stream = ChaChaStream::from_slices(&key, &nonce);
    let mut chacha = ChaCha::<20>::new(chacha_stream);
    let _ = chacha.generate::<[u8; 64]>();
    let block = chacha.generate::<[u8; 64]>();
    assert_eq!(block, expected.as_slice());
}
