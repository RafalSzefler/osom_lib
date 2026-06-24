use osom_lib_entropy::{std::StdEntropyGenerator, traits::EntropyGenerator};

mod common;

#[cfg(not(miri))]
#[test]
fn test_std_entropy_averages() {
    common::test_entropy_averages(StdEntropyGenerator::default);
}

#[cfg(not(miri))]
#[test]
fn test_std_entropy_fill() {
    common::test_entropy_fill(StdEntropyGenerator::default);
}

#[test]
fn test_std_entropy_miri() {
    let mut entropy = StdEntropyGenerator::default();
    for _ in 0..10 {
        let mut data = [0i8; 1024];
        let slice = unsafe { core::slice::from_raw_parts_mut(data.as_mut_ptr().cast::<u8>(), data.len()) };
        entropy.fill(slice).unwrap();
        let mut below_zero = 0i32;
        let mut above_zero = 0i32;
        for item in data {
            if item < 0 {
                below_zero += item as i32;
            } else {
                above_zero += item as i32;
            }
        }
        assert!(above_zero > 10000);
        assert!(below_zero < -10000);
    }
}
