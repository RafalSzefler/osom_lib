#![cfg(all(not(osom_running_env = "github"), feature = "std_os_rand"))]
mod common;

use osom_lib_rand::randomness_sources::OsRandomnessSource;
use osom_lib_rand::traits::RandomnessSource;

#[test]
fn test_statistical_properties_os_randomness_source_u32() {
    let mut source = OsRandomnessSource::<u32>::default();
    common::test_statistical_properties(|| source.next_number());
    common::test_fill_bytes::<u32, _>(|bytes| source.fill_bytes(bytes));
}

#[test]
fn test_statistical_properties_os_randomness_source_u64() {
    let mut source = OsRandomnessSource::<u64>::default();
    common::test_statistical_properties(|| source.next_number());
    common::test_fill_bytes::<u64, _>(|bytes| source.fill_bytes(bytes));
}

#[test]
fn test_statistical_properties_os_randomness_source_u128() {
    let mut source = OsRandomnessSource::<u128>::default();
    common::test_statistical_properties(|| source.next_number());
    common::test_fill_bytes::<u128, _>(|bytes| source.fill_bytes(bytes));
}
