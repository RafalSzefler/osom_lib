use std::{collections::HashMap, hint::black_box};

use criterion::{BenchmarkGroup, Criterion, criterion_group, criterion_main, measurement::WallTime};

use osom_lib_hash_tables::{bytell::defaults::StdBytellHashTable, traits::MutableHashTable};

#[inline(never)]
fn bench_insertions<T: MutableHashTable<String, i32>>(hash_table: &mut T, strings: Vec<String>) {
    let mut idx = 0;
    for txt in strings.into_iter() {
        idx += 1;
        hash_table.insert(txt, idx);
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

    fn add_benchmark<T: MutableHashTable<String, i32>>(&mut self, name: &str, mut hash_function: T) {
        let data = generate_data(self.size);
        self.group.bench_function(name, move |b| {
            b.iter(|| {
                let new_data = data.clone();
                bench_insertions(black_box(&mut hash_function), black_box(new_data));
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

    let mut bencher = Bencher::new(c, "SmallInsertions", SMALL_DATA_SIZE);
    bencher.add_benchmark("HashMap", HashMap::new());
    bencher.add_benchmark("Bytell", StdBytellHashTable::new());
    bencher.finish();

    let mut bencher = Bencher::new(c, "BigInsertions", BIG_DATA_SIZE);
    bencher.add_benchmark("HashMap", HashMap::new());
    bencher.add_benchmark("Bytell", StdBytellHashTable::new());
    bencher.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
