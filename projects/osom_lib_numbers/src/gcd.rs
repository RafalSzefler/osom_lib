//! Hold helpers for computing the greatest common divisor.

macro_rules! gcd_impl {
    ( $a: expr, $b: expr ) => {{
        let mut a = { $a };
        let mut b = { $b };
        if a == 0 {
            return b;
        }
        if b == 0 {
            return a;
        }

        let shift = (a | b).trailing_zeros();

        a >>= a.trailing_zeros();
        b >>= b.trailing_zeros();

        while b != 0 {
            b >>= b.trailing_zeros();
            if a < b {
                b -= a;
            } else {
                let temp = a - b;
                a = b;
                b = temp;
            }
        }
        a << shift
    }};
}

/// Computes the greatest common divisor of two 32-bit unsigned integers
/// in an efficient way.
#[must_use]
pub const fn gcd_32(a: u32, b: u32) -> u32 {
    gcd_impl!(a, b)
}

/// Computes the greatest common divisor of two 64-bit unsigned integers
/// in an efficient way.
#[must_use]
pub const fn gcd_64(a: u64, b: u64) -> u64 {
    gcd_impl!(a, b)
}
