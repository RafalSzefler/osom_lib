use core::fmt::Debug;
use std::ops::RangeBounds;

use osom_lib_prng::traits::{PRNConcreteBoundedGenerator, PRNConcreteGenerator, PRNGSerialize, PRNGenerator};

pub fn test_prng_serialization<TGen>(expected: &[u8], gene: TGen)
where
    TGen: PRNGenerator + PRNGSerialize + Debug,
    TGen::SerializeError: Debug,
{
    assert!(TGen::MAX_SERIALIZED_SIZE <= 1024);
    let mut buffer = [0u8; 1024];

    let written = gene.serialize(&mut buffer).unwrap();
    let slice: &[u8] = &buffer;
    let slice = &slice[0..written];
    assert_eq!(slice, expected);
}

pub fn test_prng_deserialization<TGen>(given: &[u8], nexts: &[u64])
where
    TGen: PRNGenerator + PRNGSerialize + Debug,
    TGen::DeserializeError: Debug,
    u64: PRNConcreteGenerator<TGen>,
{
    assert!(given.len() >= TGen::MAX_SERIALIZED_SIZE);
    assert!(nexts.len() > 0);

    let gene_result = TGen::deserialize(given).unwrap();
    assert!(gene_result.read_bytes <= given.len());

    let mut gene = gene_result.value;

    for item in nexts {
        assert_eq!(gene.generate::<u64>(), *item);
    }
}

fn test_in_range<TValue, TBounds, TGen>(gene: &mut TGen, range: TBounds, msg: &str)
where
    TValue: PRNConcreteBoundedGenerator<TGen> + PartialOrd + Debug,
    TBounds: RangeBounds<TValue> + Debug + Clone,
    TGen: PRNGenerator + Debug,
{
    let value = gene.generate_in_range(range.clone());
    if !range.contains(&value) {
        panic!("{}: Value {:?} is not in range {:?}", msg, value, range);
    }
}

pub fn test_in_range_u64<TGen, TBuilder>(mut builder: TBuilder)
where
    TGen: PRNGenerator + Debug,
    TBuilder: FnMut() -> TGen,
    u64: PRNConcreteBoundedGenerator<TGen>,
{
    #[cfg(not(miri))]
    const ITERATIONS: usize = 10000;
    #[cfg(miri)]
    const ITERATIONS: usize = 10;

    let mut gene = builder();
    let range1 = 10u64..17u64;
    let range2 = 25u64..;
    let range3 = ..6431u64;
    let range4 = 10u64..=17u64;
    for _ in 0..ITERATIONS {
        test_in_range(&mut gene, range1.clone(), "u64");
        test_in_range(&mut gene, range2.clone(), "u64");
        test_in_range(&mut gene, range3.clone(), "u64");
        test_in_range(&mut gene, range4.clone(), "u64");
    }
}

pub fn test_in_range_u32<TGen, TBuilder>(mut builder: TBuilder)
where
    TGen: PRNGenerator + Debug,
    TBuilder: FnMut() -> TGen,
    u32: PRNConcreteBoundedGenerator<TGen>,
{
    #[cfg(not(miri))]
    const ITERATIONS: usize = 10000;
    #[cfg(miri)]
    const ITERATIONS: usize = 10;

    let mut gene = builder();
    let range1 = 643121u32..643133u32;
    let range2 = 743254u32..;
    let range3 = ..643211u32;
    let range4 = 643121u32..=643133u32;
    for _ in 0..ITERATIONS {
        test_in_range(&mut gene, range1.clone(), "u32");
        test_in_range(&mut gene, range2.clone(), "u32");
        test_in_range(&mut gene, range3.clone(), "u32");
        test_in_range(&mut gene, range4.clone(), "u32");
    }
}

pub fn test_in_range_i64<TGen, TBuilder>(mut builder: TBuilder)
where
    TGen: PRNGenerator + Debug,
    TBuilder: FnMut() -> TGen,
    i64: PRNConcreteBoundedGenerator<TGen>,
{
    #[cfg(not(miri))]
    const ITERATIONS: usize = 10000;
    #[cfg(miri)]
    const ITERATIONS: usize = 10;

    let mut gene = builder();
    let range1 = -17i64..10i64;
    let range2 = 25i64..;
    let range3 = ..-6431i64;
    let range4 = -17i64..=10i64;
    let range5 = -17i64..=-10i64;
    for _ in 0..ITERATIONS {
        test_in_range(&mut gene, range1.clone(), "i64");
        test_in_range(&mut gene, range2.clone(), "i64");
        test_in_range(&mut gene, range3.clone(), "i64");
        test_in_range(&mut gene, range4.clone(), "i64");
        test_in_range(&mut gene, range5.clone(), "i64");
    }
}

pub fn test_in_range_i32<TGen, TBuilder>(mut builder: TBuilder)
where
    TGen: PRNGenerator + Debug,
    TBuilder: FnMut() -> TGen,
    i32: PRNConcreteBoundedGenerator<TGen>,
{
    #[cfg(not(miri))]
    const ITERATIONS: usize = 10000;
    #[cfg(miri)]
    const ITERATIONS: usize = 10;

    let mut gene = builder();
    let range1 = -643151i32..-643133i32;
    let range2 = 743254i32..;
    let range3 = ..-643211i32;
    let range4 = -643151i32..=-643133i32;
    let range5 = -17i32..=10i32;
    let range6 = -17i32..=-10i32;
    for _ in 0..ITERATIONS {
        test_in_range(&mut gene, range1.clone(), "i32");
        test_in_range(&mut gene, range2.clone(), "i32");
        test_in_range(&mut gene, range3.clone(), "i32");
        test_in_range(&mut gene, range4.clone(), "i32");
        test_in_range(&mut gene, range5.clone(), "i32");
        test_in_range(&mut gene, range6.clone(), "i32");
    }
}

pub fn test_in_range_f32<TGen, TBuilder>(mut builder: TBuilder)
where
    TGen: PRNGenerator + Debug,
    TBuilder: FnMut() -> TGen,
    f32: PRNConcreteBoundedGenerator<TGen>,
{
    #[cfg(not(miri))]
    const ITERATIONS: usize = 10000;
    #[cfg(miri)]
    const ITERATIONS: usize = 10;

    let mut gene = builder();
    let range1 = 0.0f32..1.0f32;
    let range2 = 0.0f32..=1.0f32;
    let range3 = ..0.0f32;
    let range4 = -100f32..-90f32;
    let range5 = -50f32..50f32;
    let range6 = 1300f32..7500f32;
    let range7 = 10.0f32..;
    for _ in 0..ITERATIONS {
        test_in_range(&mut gene, range1.clone(), "f32");
        test_in_range(&mut gene, range2.clone(), "f32");
        test_in_range(&mut gene, range3.clone(), "f32");
        test_in_range(&mut gene, range4.clone(), "f32");
        test_in_range(&mut gene, range5.clone(), "f32");
        test_in_range(&mut gene, range6.clone(), "f32");
        test_in_range(&mut gene, range7.clone(), "f32");
    }
}

pub fn test_in_range_f64<TGen, TBuilder>(mut builder: TBuilder)
where
    TGen: PRNGenerator + Debug,
    TBuilder: FnMut() -> TGen,
    f64: PRNConcreteBoundedGenerator<TGen>,
{
    #[cfg(not(miri))]
    const ITERATIONS: usize = 10000;
    #[cfg(miri)]
    const ITERATIONS: usize = 10;

    let mut gene = builder();
    let range1 = 0.0f64..1.0f64;
    let range2 = 0.0f64..=1.0f64;
    let range3 = ..0.0f64;
    let range4 = -100f64..-90f64;
    let range5 = -50f64..50f64;
    let range6 = 1300f64..7500f64;
    let range7 = 10.0f64..;
    for _ in 0..ITERATIONS {
        test_in_range(&mut gene, range1.clone(), "f64");
        test_in_range(&mut gene, range2.clone(), "f64");
        test_in_range(&mut gene, range3.clone(), "f64");
        test_in_range(&mut gene, range4.clone(), "f64");
        test_in_range(&mut gene, range5.clone(), "f64");
        test_in_range(&mut gene, range6.clone(), "f64");
        test_in_range(&mut gene, range7.clone(), "f64");
    }
}
