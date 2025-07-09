use core::ops::Deref;


/// Represents an array of `N+M` elements. The only reason this struct
/// exists is because `[T; N+M]` is not allowed in Rust yet.
#[repr(C)]
pub struct ConstSumArray<const N: usize, const M: usize, T>
{
    left_values: [T; N],
    right_values: [T; M],
}

impl<const N: usize, const M: usize, T> ConstSumArray<N, M, T>
{
    #[inline(always)]
    pub const fn new(initial_values: [T; N], final_value: [T; M]) -> Self
    {
        Self { left_values: initial_values, right_values: final_value }
    }

    #[inline(always)]
    pub const fn as_slice(&self) -> &[T] {
        unsafe {
            let ptr = self as *const Self;
            let len = N + M;
            core::slice::from_raw_parts(ptr as *const T, len)
        }
    }

    #[inline(always)]
    pub const fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe {
            let ptr = self as *mut Self;
            let len = N + M;
            core::slice::from_raw_parts_mut(ptr as *mut T, len)
        }
    }

    #[inline(always)]
    pub const fn len(&self) -> usize {
        N + M
    }

    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline(always)]
    pub const fn at(&self, index: usize) -> &T {
        if index < N {
            &self.left_values[index]
        } else if index < N + M {
            &self.right_values[index - N]
        } else {
            panic!("Index out of bounds");
        }
    }

    #[inline(always)]
    pub const fn at_mut(&mut self, index: usize) -> &mut T {
        if index < N {
            &mut self.left_values[index]
        } else if index < N + M {
            &mut self.right_values[index - N]
        } else {
            panic!("Index out of bounds");
        }
    }
}

impl<const N: usize, const M: usize, T> Deref for ConstSumArray<N, M, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<const N: usize, const M: usize, T> core::ops::DerefMut for ConstSumArray<N, M, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_const_sum_array() {
        let array = ConstSumArray::<5, 7, u8>::new([1, 2, 3, 4, 5], [6, 7, 8, 9, 10, 11, 12]);
        assert_eq!(array.len(), 12);
        assert_eq!(array.as_slice(), &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
    }

    #[test]
    fn test_const_sum_array_i32() {
        let array = ConstSumArray::<3, 2, i32>::new([-1, 0, 1], [-5, 17]);
        assert_eq!(array.len(), 5);
        assert_eq!(array.as_slice(), &[-1, 0, 1, -5, 17]);
    }

    #[test]
    fn test_const_sum_array_mut() {
        let mut array = ConstSumArray::<3, 2, i32>::new([-1, 0, 1], [-5, 17]);
        assert_eq!(array.as_mut_slice(), &[-1, 0, 1, -5, 17]);
        *array.at_mut(0) = 11;
        assert_eq!(array.as_slice(), &[11, 0, 1, -5, 17]);
        array.as_mut_slice()[1] = 22;
        assert_eq!(array.as_slice(), &[11, 22, 1, -5, 17]);
    }
}
