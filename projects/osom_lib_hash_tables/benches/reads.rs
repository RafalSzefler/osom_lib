use std::{collections::HashMap, hint::black_box};

use criterion::{BenchmarkGroup, Criterion, criterion_group, criterion_main, measurement::WallTime};

use osom_lib_hash_tables::abseil::defaults::StdAbseilHashTable;
use osom_lib_hash_tables::bytell::defaults::StdBytellHashTable;
use osom_lib_hash_tables::traits::{ImmutableHashTable, MutableHashTable};

#[inline(never)]
fn bench_reads<T: ImmutableHashTable<String, usize>>(hash_table: &T, strings: &Vec<String>) {
    for txt in strings {
        let _ = hash_table.get(txt);
    }
}

fn generate_data(size: usize) -> Vec<String> {
    let mut vec = Vec::with_capacity(size + 10);
    let first = 3 * size / 4;
    for idx in 0..first {
        vec.push(format!("first{idx}"));
    }

    vec.push("foo".to_string());

    let second = size / 4;
    for idx in 0..second {
        vec.push(format!("second{idx}"));
    }

    vec.push("baz".to_string());

    vec
}

struct Bencher<'a> {
    group: BenchmarkGroup<'a, WallTime>,
    size: usize,
}

impl<'a> Bencher<'a> {
    fn new(c: &'a mut Criterion, name: &str, size: usize) -> Self {
        let group = c.benchmark_group(name);
        Self { group, size }
    }

    fn add_benchmark<T: MutableHashTable<String, usize>>(&mut self, name: &str, mut hash_table: T) {
        let data = generate_data(self.size);
        for (idx, text) in data.iter().enumerate() {
            let result = hash_table.insert(text.clone(), idx);
            assert!(result.is_none());
        }

        self.group.bench_function(name, move |b| {
            b.iter(|| {
                bench_reads(black_box(&hash_table), black_box(&data));
            })
        });
    }

    fn finish(self) {
        self.group.finish();
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    const BIG_DATA_SIZE: usize = 50000;
    const SMALL_DATA_SIZE: usize = 500;

    let mut bencher = Bencher::new(c, "SmallReads", SMALL_DATA_SIZE);
    bencher.add_benchmark("HashMap", HashMap::new());
    bencher.add_benchmark("Bytell", StdBytellHashTable::new());
    bencher.add_benchmark("Abseil", StdAbseilHashTable::new());
    bencher.finish();

    let mut bencher = Bencher::new(c, "BigReads", BIG_DATA_SIZE);
    bencher.add_benchmark("HashMap", HashMap::new());
    bencher.add_benchmark("Bytell", StdBytellHashTable::new());
    bencher.add_benchmark("Abseil", StdAbseilHashTable::new());
    bencher.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
