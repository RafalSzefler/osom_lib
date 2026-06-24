#![allow(unused)]
use std::collections::{HashMap, hash_map::Entry};
use std::fmt::Debug;

use osom_lib_entropy::traits::{EntropyConcreteGenerator, EntropyGenerator};

const UP_TO: f32 = 0.3333;

pub fn test_entropy_averages<TGenerator, TBuilder>(builder: TBuilder)
where
    TGenerator: EntropyGenerator + Debug,
    TBuilder: FnOnce() -> TGenerator,
    u64: EntropyConcreteGenerator<TGenerator>,
    TGenerator::Error: Debug,
{
    const RANGE: u64 = 1009;
    const ITERATIONS: usize = 500000;
    let mut generator = builder();
    let mut data = HashMap::with_capacity(RANGE as usize);

    for _ in 0..ITERATIONS {
        let value = generate_in_range::<_, RANGE>(&mut generator);

        match data.entry(value) {
            Entry::Occupied(mut occupied_entry) => {
                *(occupied_entry.get_mut()) += 1;
            }
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(1);
            }
        }
    }

    let average = (ITERATIONS as f32) / (RANGE as f32);
    let diff = average * UP_TO;
    let start = (average - diff) as u32 - 1;
    let end = (average + diff) as u32 + 1;
    let range = start..end;
    for item in data.values() {
        if !range.contains(item) {
            panic!("Item {} is not in {}..{} range.", item, range.start, range.end);
        }
    }
}

fn generate_in_range<TGenerator, const RANGE: u64>(generator: &mut TGenerator) -> u64
where
    TGenerator: EntropyGenerator + Debug,
    u64: EntropyConcreteGenerator<TGenerator>,
    TGenerator::Error: Debug,
{
    let next_power_mask = u64::next_power_of_two(RANGE) - 1;
    let mut value = generator.generate::<u64>().unwrap() & next_power_mask;
    while value >= RANGE {
        value = generator.generate::<u64>().unwrap() & next_power_mask;
    }
    value
}

pub fn test_entropy_fill<TGenerator, TBuilder>(builder: TBuilder)
where
    TGenerator: EntropyGenerator + Debug,
    TBuilder: FnOnce() -> TGenerator,
    TGenerator::Error: Debug,
{
    const COUNT: usize = 50000;
    let mut generator = builder();
    let mut data = [0u8; COUNT];
    generator.fill(&mut data).unwrap();
    let mut counts = [0u32; 256];
    for item in data.iter() {
        let idx = *item as usize;
        counts[idx] += 1;
    }

    let average = (COUNT as f32) / (counts.len() as f32);
    let diff = average * UP_TO;
    let start = (average - diff) as u32 - 1;
    let end = (average + diff) as u32 + 1;
    let range = start..end;
    for item in counts {
        if !range.contains(&item) {
            panic!("Item {} is not in {}..{} range.", item, range.start, range.end);
        }
    }
}
