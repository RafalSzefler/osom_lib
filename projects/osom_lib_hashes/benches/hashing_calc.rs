use std::{hint::black_box, sync::Arc};

use criterion::{BenchmarkGroup, Criterion, criterion_group, criterion_main, measurement::WallTime};

use osom_lib_hashes::{
    fnv::FNV1a_64, fxhash::FxHash, sha2::sha2_256::portable::SHA2_256_Portable, siphash::SipHash, traits::HashFunction,
};

#[inline(never)]
fn hash_data<T: HashFunction>(hash_function: &mut T, data: &[u8]) {
    hash_function.update(data);
    let _ = hash_function.result();
}

fn generate_data(size: usize) -> Vec<u8> {
    const A: u128 = 0xdb36357734e34abb0050d0761fcdfc15;
    const C: u128 = 0x86e9;
    let upper = 4 * ((size / 4) + 1);
    let mut data = Vec::with_capacity(upper);
    let mut state = 0u128;
    for _ in 0..upper {
        state = state.wrapping_mul(A).wrapping_add(C);
        data.extend_from_slice(&(state as u32).to_le_bytes());
    }
    data.resize(size, 0);
    data
}

struct Bencher<'a> {
    group: BenchmarkGroup<'a, WallTime>,
    data: Arc<Vec<u8>>,
}

impl<'a> Bencher<'a> {
    fn new(c: &'a mut Criterion, name: &str, size: usize) -> Self {
        let group = c.benchmark_group(name);
        let data = Arc::new(generate_data(size));
        Self { group, data }
    }

    fn add_benchmark<T: HashFunction>(&mut self, name: &str, mut hash_function: T) {
        let data = self.data.clone();
        self.group.bench_function(name, move |b| {
            b.iter(|| hash_data(black_box(&mut hash_function), black_box(&data)))
        });
    }

    fn finish(self) {
        self.group.finish();
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    const BIG_DATA_SIZE: usize = 25000;
    const SMALL_DATA_SIZE: usize = 50;

    let mut bencher = Bencher::new(c, "BigDataHashing", BIG_DATA_SIZE);
    bencher.add_benchmark("FxHash", FxHash::new());
    bencher.add_benchmark("FNV1a_64", FNV1a_64::new());
    bencher.add_benchmark("SipHash", SipHash::for_keys(0, 1));
    bencher.add_benchmark("SHA2_256_Portable", SHA2_256_Portable::new());

    #[cfg(all(
        any(target_arch = "x86", target_arch = "x86_64"),
        target_feature = "sha",
        target_feature = "sse4.1"
    ))]
    {
        use osom_lib_hashes::sha2::sha2_256::platform::SHA2_256_x86;
        bencher.add_benchmark("SHA2_256_x86", SHA2_256_x86::new());
    }

    #[cfg(target_arch = "aarch64")]
    {
        use osom_lib_hashes::sha2::sha2_256::platform::SHA2_256_aarch64;
        bencher.add_benchmark("SHA2_256_aarch64", SHA2_256_aarch64::new());
    }

    bencher.finish();

    let mut bencher = Bencher::new(c, "SmallDataHashing", SMALL_DATA_SIZE);
    bencher.add_benchmark("FxHash", FxHash::new());
    bencher.add_benchmark("FNV1a_64", FNV1a_64::new());
    bencher.add_benchmark("SipHash", SipHash::for_keys(0, 1));
    bencher.add_benchmark("SHA2_256_Portable", SHA2_256_Portable::new());

    #[cfg(all(
        any(target_arch = "x86", target_arch = "x86_64"),
        target_feature = "sha",
        target_feature = "sse4.1"
    ))]
    {
        use osom_lib_hashes::sha2::sha2_256::platform::SHA2_256_x86;
        bencher.add_benchmark("SHA2_256_x86", SHA2_256_x86::new());
    }

    #[cfg(target_arch = "aarch64")]
    {
        use osom_lib_hashes::sha2::sha2_256::platform::SHA2_256_aarch64;
        bencher.add_benchmark("SHA2_256_aarch64", SHA2_256_aarch64::new());
    }

    bencher.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
