//! Contains strategies for converting hash values to indices in the bytell hash table.

use osom_lib_macros::debug_check_or_release_hint;
use osom_lib_primitives::power_of_two::PowerOfTwo32;
use osom_lib_reprc::macros::reprc;

/// This trait represents a strategy for converting hash values to indices in the bytell hash table.
pub trait HashToIndex: Default + Clone {
    /// Converts a hash value to an index in the bytell hash table.
    fn hash_to_index(&self, hash_value: u64, table_capacity: PowerOfTwo32) -> usize;

    /// Updates the hash-to-index policy when the table changes its capacity.
    fn update_for_new_table_capacity(&mut self, table_capacity: PowerOfTwo32);
}

/// Represents the Fibonacci variant of hash-to-index policy.
#[derive(Default, Clone, Copy)]
#[reprc]
#[repr(transparent)]
pub struct FibonacciHashToIndex {
    shift: u8,
}

impl HashToIndex for FibonacciHashToIndex {
    #[inline]
    fn hash_to_index(&self, hash_value: u64, table_capacity: PowerOfTwo32) -> usize {
        let value = table_capacity.value();
        debug_check_or_release_hint!(value.is_power_of_two(), "table_size not a power of two");
        let shift = self.shift;
        let mut result = hash_value;
        result ^= result >> shift;
        result = result.wrapping_mul(11400714819323198485) >> shift;
        debug_check_or_release_hint!(result < u64::from(u32::MAX), "hash_to_index beyond u32::MAX");

        #[allow(clippy::cast_possible_truncation)]
        {
            result as usize
        }
    }

    fn update_for_new_table_capacity(&mut self, table_capacity: PowerOfTwo32) {
        #[allow(clippy::cast_possible_truncation)]
        {
            self.shift = (64 - table_capacity.value().trailing_zeros()) as u8;
        }
    }
}

/// Represents the power of two variant of hash-to-index policy. This basically
/// just does modulo operation.
#[derive(Default, Clone, Copy)]
#[reprc]
#[repr(transparent)]
pub struct PowerOfTwoHashToIndex;

impl HashToIndex for PowerOfTwoHashToIndex {
    #[inline(always)]
    fn hash_to_index(&self, hash_value: u64, table_capacity: PowerOfTwo32) -> usize {
        #[allow(clippy::cast_possible_truncation)]
        let hash_value = hash_value as u32;
        (hash_value & (table_capacity.value() - 1)) as usize
    }

    #[inline(always)]
    fn update_for_new_table_capacity(&mut self, _: PowerOfTwo32) {}
}
