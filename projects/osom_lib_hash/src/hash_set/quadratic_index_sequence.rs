/// A quadratic index sequence for probing.
pub(crate) struct QuadraticIndexSequence {
    hash: usize,
    i: usize,
    data_len: usize,
    modulus: usize,
}

impl QuadraticIndexSequence {
    #[inline(always)]
    pub const fn new(hash: usize, data_len: usize) -> Self {
        let modulus = data_len.next_power_of_two();
        Self {
            hash,
            i: 0,
            data_len,
            modulus,
        }
    }

    #[inline(always)]
    pub const fn next(&mut self) -> usize {
        let data_len = self.data_len;
        let modulus = self.modulus;
        let hash = self.hash;
        let mut i = self.i;

        loop {
            let new_index = Self::calculate_index(hash, i, modulus);
            i += 1;
            if new_index < data_len {
                self.i = i;
                return new_index;
            }
        }
    }

    #[inline(always)]
    const fn calculate_index(hash: usize, i: usize, modulus: usize) -> usize {
        let idx = (i * i + i) / 2;
        (hash + idx) % modulus
    }
}
