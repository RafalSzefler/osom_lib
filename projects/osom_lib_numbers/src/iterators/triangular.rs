/// An iterator that generates triangular numbers, i.e.
/// numbers of the form `n * (n + 1) / 2` but modulo `2^32`.
///
/// The generator generates up to `count` numbers.
///
/// Note that the iterator starts at 0 and not at 1.
#[repr(C)]
#[derive(Debug, Clone)]
#[must_use]
pub struct IterTriangular {
    current: u32,
    count: u32,
}

impl IterTriangular {
    /// Creates a new iterator that generates `count` triangular numbers
    /// as `u32`.
    #[inline(always)]
    pub const fn new(count: u32) -> Self {
        Self { current: 0, count }
    }

    /// Returns the next triangular number.
    #[must_use]
    pub const fn next(&mut self) -> Option<u32> {
        if self.current == self.count {
            return None;
        }

        let result = self.current as u64;
        let result = unsafe { result.unchecked_mul(result.unchecked_add(1)) } / 2;

        #[allow(clippy::cast_possible_truncation)]
        let result = result as u32;

        unsafe {
            self.current = self.current.unchecked_add(1);
        }

        Some(result)
    }
}

impl Iterator for IterTriangular {
    type Item = u32;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        #[allow(clippy::cast_possible_truncation)]
        {
            let remaining = self.count as usize - self.current as usize;
            (remaining, Some(remaining))
        }
    }
}
