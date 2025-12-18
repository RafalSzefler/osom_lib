//! Holds the [`KeyValuePair`][KVP] struct and its implementation.

use osom_lib_reprc::traits::ReprC;

/// Represents the `(key, value)` pair, but with `#[repr(C)]` ABI,
/// where `key` comes first, then `value`.
///
/// This struct is readonly.
#[allow(clippy::upper_case_acronyms)]
#[repr(C)]
#[must_use]
pub struct KVP<TKey, TValue> {
    pub key: TKey,
    pub value: TValue,
}

unsafe impl<TKey: ReprC, TValue: ReprC> ReprC for KVP<TKey, TValue> {
    const CHECK: () = const {
        let () = <TKey as ReprC>::CHECK;
        let () = <TValue as ReprC>::CHECK;
    };
}
