mod common;

use osom_lib_hash_tables::bytell::defaults::StdBytellHashTable;

common::build_tests!(StdBytellHashTable);

#[cfg(not(miri))]
#[test]
fn test_bytell_distribution() {
    use osom_lib_hash_tables::traits::MutableHashTable;

    const ITERATIONS: u32 = 100000;

    const WORDS: &str = include_str!("words.txt");
    let words: Vec<String> = WORDS.split_terminator("\n").map(str::to_owned).collect();
    assert!(words.len() > 50000);

    const A: u128 = 0xdb36357734e34abb0050d0761fcdfc15;
    const C: u128 = 0x86e9;
    let mut state = 0u128;

    let mut generate_string = || -> String {
        state = state.wrapping_mul(A).wrapping_add(C);
        let idx = (state as usize) % words.len();
        words[idx].clone()
    };

    let mut table = StdBytellHashTable::new();
    for idx in 0..ITERATIONS {
        let mut key = generate_string();
        key.push_str(idx.to_string().as_str());
        table.insert(key, idx);
    }
}
