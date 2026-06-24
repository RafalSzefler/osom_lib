use std::ops::{Index, IndexMut};

pub struct HdVec<const DIM: usize, T: Default> {
    sizes: [usize; DIM],
    inner: Vec<T>,
}

impl<const DIM: usize, T: Default> HdVec<DIM, T> {
    const _CHECK: () = const {
        assert!(DIM >= 2);
        assert!(DIM <= 4);
    };
}

impl<T: Default> HdVec<2, T> {
    pub fn new(coord1_size: usize, coord2_size: usize) -> Self {
        Self::with_factory(coord1_size, coord2_size, |_, _| T::default())
    }

    pub fn with_factory<TFactory>(coord1_size: usize, coord2_size: usize, mut factory: TFactory) -> Self
    where
        TFactory: FnMut(usize, usize) -> T,
    {
        let mut new_vec = Vec::new();
        new_vec.reserve_exact(coord1_size * coord2_size);
        for coord2 in 0..coord2_size {
            for coord1 in 0..coord1_size {
                new_vec.push(factory(coord1, coord2));
            }
        }
        Self {
            sizes: [coord1_size, coord2_size],
            inner: new_vec,
        }
    }

    const fn calculate_index(&self, coord1: usize, coord2: usize) -> usize {
        assert!(coord1 < self.sizes[0], "First coordinate out of range.");
        assert!(coord2 < self.sizes[1], "Second coordinate out of range.");
        let coord2_len = self.sizes[1];
        coord1 * coord2_len + coord2
    }
}

impl<T: Default> Index<(usize, usize)> for HdVec<2, T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let index = self.calculate_index(index.0, index.1);
        &self.inner[index]
    }
}

impl<T: Default> IndexMut<(usize, usize)> for HdVec<2, T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let index = self.calculate_index(index.0, index.1);
        &mut self.inner[index]
    }
}

impl<T: Default> HdVec<3, T> {
    pub fn new(coord1_size: usize, coord2_size: usize, coord3_size: usize) -> Self {
        Self::with_factory(coord1_size, coord2_size, coord3_size, |_, _, _| T::default())
    }

    pub fn with_factory<TFactory>(
        coord1_size: usize,
        coord2_size: usize,
        coord3_size: usize,
        mut factory: TFactory,
    ) -> Self
    where
        TFactory: FnMut(usize, usize, usize) -> T,
    {
        let mut new_vec = Vec::new();
        new_vec.reserve_exact(coord1_size * coord2_size);
        for coord3 in 0..coord3_size {
            for coord2 in 0..coord2_size {
                for coord1 in 0..coord1_size {
                    new_vec.push(factory(coord1, coord2, coord3));
                }
            }
        }

        Self {
            sizes: [coord1_size, coord2_size, coord3_size],
            inner: new_vec,
        }
    }

    const fn calculate_index(&self, coord1: usize, coord2: usize, coord3: usize) -> usize {
        assert!(coord1 < self.sizes[0], "First coordinate out of range.");
        assert!(coord2 < self.sizes[1], "Second coordinate out of range.");
        assert!(coord3 < self.sizes[2], "Third coordinate out of range.");
        (coord1 + self.sizes[1] * coord1) * self.sizes[2] + coord3
    }
}

impl<T: Default> Index<(usize, usize, usize)> for HdVec<3, T> {
    type Output = T;

    fn index(&self, index: (usize, usize, usize)) -> &Self::Output {
        let index = self.calculate_index(index.0, index.1, index.2);
        &self.inner[index]
    }
}

impl<T: Default> IndexMut<(usize, usize, usize)> for HdVec<3, T> {
    fn index_mut(&mut self, index: (usize, usize, usize)) -> &mut Self::Output {
        let index = self.calculate_index(index.0, index.1, index.2);
        &mut self.inner[index]
    }
}
