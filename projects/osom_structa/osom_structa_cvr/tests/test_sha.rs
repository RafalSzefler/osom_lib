#![cfg(all(feature = "std", feature = "serde"))]
#![allow(unused)]
use rstest::rstest;

use osom_lib_hashes::traits::HashFunction as _;
use osom_structa_cvr::{serde::CVRSeed, std::StdCVRDeserializeContext, tools::binarize};
use priv_osom_lib_tests::deserialize::deserialize_json_with_seed;

mod common;

#[test]
#[cfg(not(miri))]
fn test_sha() {
    const EXPECTED: &[u8; 32] = b"\x3a\xfd\x62\xc9\xcd\x3d\xc3\x14\x0b\x50\xa5\x61\xe6\x7f\x6d\xba\xd4\x07\x24\x3a\x7f\x06\x37\x64\x71\xb7\x62\x2b\x0a\xa7\x79\x88";
    let mut sha = osom_lib_hashes::sha2::sha2_256::portable::SHA2_256_Portable::new();

    let mut context = StdCVRDeserializeContext::new().unwrap();
    let seed = CVRSeed { context: &mut context };
    let result = deserialize_json_with_seed(common::TEXT_5MB, seed).unwrap();

    binarize(&result, |data| sha.update(data));

    let final_hash = sha.result_const();
    assert_eq!(&final_hash, EXPECTED);
}

#[rstest]
#[case("{\"is internal\":true,\"test\":1,\"val\":[1,null]}")]
#[case("{\"is internal\": true, \"test\": 1, \"val\": [1, null]}")]
#[case("{\"is internal\": true, \"val\": [1, null], \"test\": 1}")]
#[case("{\"test\": 1, \"is internal\": true, \"val\": [1, null]}")]
#[case("{\"test\": 1, \"val\": [1, null], \"is internal\": true}")]
#[case("{\"val\": [1, null], \"test\": 1, \"is internal\": true}")]
#[case("{\"val\": [1, null], \"is internal\": true, \"test\": 1}")]
fn test_sha_commutes(#[case] input: &str) {
    const EXPECTED: &[u8; 32] = b"\xEF\xEB\xBE\x66\xB8\xA0\xBD\x6A\x36\x0A\x7A\x65\x3C\x86\x5A\x0A\x06\xE9\x98\xA5\x02\x8E\xA4\xE9\x39\x5A\xE8\xB3\x6C\xD4\xFD\x37";
    let mut sha = osom_lib_hashes::sha2::sha2_256::portable::SHA2_256_Portable::new();

    let mut context = StdCVRDeserializeContext::new().unwrap();
    let seed = CVRSeed { context: &mut context };
    let result = deserialize_json_with_seed(input, seed).unwrap();

    binarize(&result, |data| sha.update(data));

    let final_hash = sha.result_const();
    assert_eq!(&final_hash, EXPECTED);
}
