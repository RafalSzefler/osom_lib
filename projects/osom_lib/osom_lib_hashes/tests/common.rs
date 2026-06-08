#![allow(unused)]

use osom_lib_hashes::traits::HashFunction;

fn generate_data(size: usize) -> Vec<u8> {
    const A: u128 = 0xdb36357734e34abb0050d0761fcdfc15;
    const C: u128 = 0x86e9;
    let mut data = Vec::with_capacity(size);
    let mut state = 0u128;
    for _ in 0..(size / 4) {
        state = state.wrapping_mul(A).wrapping_add(C);
        data.extend_from_slice(&state.to_le_bytes());
    }
    data.resize(size, 0);
    data
}

pub fn test_pseudo_random_hashing<TH, TG>(hash_builder: TG, size: usize, expected: &[u8])
where
    TH: HashFunction,
    TG: FnOnce() -> TH,
{
    let data = generate_data(size);
    let mut hash = hash_builder();
    hash.update(&data);
    let block = hash.result();
    assert_eq!(block.as_ref(), expected);
}
