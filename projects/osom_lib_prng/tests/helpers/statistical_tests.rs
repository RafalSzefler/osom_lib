use std::num::NonZero;

use crate::helpers::hd_vec::HdVec;

const STATISTICAL_THRESHOLD: f64 = 0.15;

pub struct StatisticalTest {
    cube_size: usize,
    sample_size: usize,
}

impl StatisticalTest {
    #[inline(always)]
    pub const fn builder() -> StatisticalTestBuilder {
        StatisticalTestBuilder::new()
    }
}

pub struct StatisticalTestBuilder {
    cube_size: Option<NonZero<usize>>,
    sample_size: Option<NonZero<usize>>,
}

impl StatisticalTestBuilder {
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            cube_size: None,
            sample_size: None,
        }
    }

    pub const fn set_cube_size(mut self, cube_size: usize) -> Self {
        if cube_size > 10000 {
            panic!("The maximum value of cube_size is 10001");
        }

        if cube_size < 11 {
            panic!("The minimum value of cube_size is 11");
        }

        if cube_size & 1 == 1 {
            panic!("cube_size has to be a prime number");
        }

        let mut factor = 3;
        let limit = (cube_size / 2) + 1;
        while factor < limit {
            if cube_size % factor == 0 {
                panic!("cube_size has to be a prime number");
            }
            factor += 2;
        }

        self.cube_size = Some(unsafe { NonZero::new_unchecked(cube_size) });
        self
    }

    pub const fn build(self) -> StatisticalTest {
        let cube_size = match self.cube_size {
            Some(val) => val.get(),
            None => 11,
        };
        let sample_size = match self.sample_size {
            Some(val) => val.get(),
            None => 100001,
        };
        StatisticalTest { cube_size, sample_size }
    }
}

impl StatisticalTest {
    pub fn test_1d<TGen>(&self, mut gene: TGen)
    where
        TGen: FnMut() -> u32,
    {
        let cube_size = self.cube_size;
        let sample_size = self.sample_size;
        assert!(
            sample_size > cube_size * cube_size,
            "We need sample_size to be bigger than cube squared."
        );
        let mut square = Vec::new();
        square.resize(cube_size, 0u32);

        for _ in 0..sample_size {
            let x = (gene() as usize) % cube_size;
            square[x] += 1;
        }

        let average = (sample_size as f64) / (cube_size as f64);
        for x in 0..cube_size {
            let real = square[x] as f64;
            let percentage = (real - average) / average;
            assert!(
                percentage.abs() < STATISTICAL_THRESHOLD,
                "Real count {real} is too far away from the expected average {average}."
            );
        }
    }

    pub fn test_2d<TGen1, TGen2>(&self, mut gen1: TGen1, mut gen2: TGen2)
    where
        TGen1: FnMut() -> u32,
        TGen2: FnMut() -> u32,
    {
        let cube_size = self.cube_size;
        let sample_size = self.sample_size;
        assert!(
            sample_size > cube_size * cube_size,
            "We need sample_size to be bigger than cube squared."
        );
        let mut square = HdVec::<2, u32>::new(cube_size, cube_size);

        for _ in 0..sample_size {
            let x = (gen1() as usize) % cube_size;
            let y = (gen2() as usize) % cube_size;
            square[(x, y)] += 1;
        }

        let average = (sample_size as f64) / (cube_size as f64).powi(2);
        for x in 0..cube_size {
            for y in 0..cube_size {
                let real = square[(x, y)] as f64;
                let percentage = (real - average) / average;
                assert!(
                    percentage.abs() < STATISTICAL_THRESHOLD,
                    "Real count {real} is too far away from the expected average {average}."
                );
            }
        }
    }

    pub fn test_3d<TGen1, TGen2, TGen3>(&self, mut gen1: TGen1, mut gen2: TGen2, mut gen3: TGen3)
    where
        TGen1: FnMut() -> u32,
        TGen2: FnMut() -> u32,
        TGen3: FnMut() -> u32,
    {
        let cube_size = self.cube_size;
        let sample_size = self.sample_size;
        assert!(
            sample_size > cube_size * cube_size,
            "We need sample_size to be bigger than cube squared."
        );
        let mut square = HdVec::<3, u32>::new(cube_size, cube_size, cube_size);

        for _ in 0..sample_size {
            let x = (gen1() as usize) % cube_size;
            let y = (gen2() as usize) % cube_size;
            let z = (gen3() as usize) % cube_size;
            square[(x, y, z)] += 1;
        }

        let average = (sample_size as f64) / (cube_size as f64).powi(2);
        for z in 0..cube_size {
            for x in 0..cube_size {
                for y in 0..cube_size {
                    let real = square[(x, y, z)] as f64;
                    let percentage = (real - average) / average;
                    assert!(
                        percentage.abs() < STATISTICAL_THRESHOLD,
                        "Real count {real} is too far away from the expected average {average}."
                    );
                }
            }
        }
    }
}
