use std::hint::black_box;

use criterion::{BenchmarkGroup, Criterion, criterion_group, criterion_main, measurement::WallTime};
use osom_lib_prng::{
    prngs::{ChaCha, LinearCongruentialGenerator128, SplitMix64},
    traits::{PRNConcreteGenerator, PRNGenerator, Seedable},
};

#[inline(never)]
fn generate_u64<T: PRNGenerator>(gene: &mut T, rounds: u32)
where
    u64: PRNConcreteGenerator<T>,
{
    for _ in 0..rounds {
        let _ = gene.generate::<u64>();
    }
}

fn bench_prng_u64_rounds<T: PRNGenerator>(group: &mut BenchmarkGroup<'_, WallTime>, name: &str, mut gene: T)
where
    u64: PRNConcreteGenerator<T>,
{
    const ROUNDS: u32 = 100000;
    group.bench_function(name, |b| {
        b.iter(|| generate_u64(black_box(&mut gene), black_box(ROUNDS)))
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("PRNG_u64_rounds");
    group.significance_level(0.1).sample_size(100);

    bench_prng_u64_rounds(&mut group, "ChaCha", ChaCha::<20>::with_seed(256u128));
    bench_prng_u64_rounds(&mut group, "LCG", LinearCongruentialGenerator128::with_seed(256u128));
    bench_prng_u64_rounds(&mut group, "SplitMix64", SplitMix64::with_seed(1234u64));

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
