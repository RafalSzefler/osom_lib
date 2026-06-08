/// A generator for permutations of `N` elements.
#[must_use]
pub struct ConstPermutationGenerator<const N: usize> {
    current: [usize; N],
    first: bool,
    done: bool,
}

impl<const N: usize> ConstPermutationGenerator<N> {
    /// The length of the permutations.
    #[inline(always)]
    #[must_use]
    pub const fn length(&self) -> usize {
        N
    }

    /// Creates a new [`ConstPermutationGenerator`].
    pub const fn new() -> Self {
        let mut current = [0; N];
        let mut i = 0;
        while i < N {
            current[i] = i;
            i += 1;
        }
        Self {
            current,
            first: true,
            done: false,
        }
    }

    /// Returns the next permutation.
    #[must_use]
    pub const fn next(&mut self) -> Option<[usize; N]> {
        if self.done {
            return None;
        }

        if self.first {
            self.first = false;
            return Some(self.current);
        }

        if Self::next_permutation(&mut self.current) {
            Some(self.current)
        } else {
            self.done = true;
            None
        }
    }

    const fn next_permutation(data: &mut [usize; N]) -> bool {
        if N < 2 {
            return false;
        }

        // Find rightmost ascent
        let mut i = N - 2;
        loop {
            if data[i] < data[i + 1] {
                break;
            }
            if i == 0 {
                return false;
            }
            i -= 1;
        }

        // Find smallest larger element to the right
        let mut j = data.len() - 1;
        while data[j] <= data[i] {
            j -= 1;
        }

        data.swap(i, j);

        let slice = unsafe { core::slice::from_raw_parts_mut(data.as_mut_ptr().add(i + 1), N - i - 1) };
        slice.reverse();

        true
    }
}

impl<const N: usize> Default for ConstPermutationGenerator<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> Clone for ConstPermutationGenerator<N> {
    fn clone(&self) -> Self {
        Self {
            current: self.current,
            first: self.first,
            done: self.done,
        }
    }
}

impl<const N: usize> Iterator for ConstPermutationGenerator<N> {
    type Item = [usize; N];
    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}
