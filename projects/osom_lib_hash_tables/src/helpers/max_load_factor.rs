use osom_lib_reprc::macros::reprc;

/// Represents the load limits for the hash table. The load is typically calculated
/// as the ratio of `number_of_elements` to `capacity`.
#[derive(Debug, Clone, Copy)]
#[reprc]
#[repr(transparent)]
#[must_use]
pub struct MaxLoadFactor {
    value: f64,
}

impl MaxLoadFactor {
    /// Creates a new [`MaxLoadFactor`] instance out of raw values.
    ///
    /// # Panics
    ///
    /// If `min < 0.001` or when `max > 0.999` or when `min > max - 0.05`.
    #[inline(always)]
    pub const fn new(max: f64) -> Self {
        assert!(max <= 0.999, "Max load factor cannot exceed 0.999");
        assert!(max > 0.1, "Max load factor must be above 0.1");
        Self { value: max }
    }

    /// Returns a max threshold for the load. This is guaranteed to be in
    /// `0.0 .. 1.0` range.
    #[inline(always)]
    #[must_use]
    pub const fn value(self) -> f64 {
        let result = self.value;
        unsafe {
            core::hint::assert_unchecked(result > 0.1 && result <= 0.999);
        }
        result
    }
}
