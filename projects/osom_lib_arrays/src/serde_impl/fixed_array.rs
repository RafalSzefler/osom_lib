//! NOTE: [`FixedArray`] has to have capacity specified at runtime. This means
//! that deserialization is not really possible. Even though serde does provide
//! `size_hint()` at runtime, these are not reliable, and depending on the protocol
//! will fail to work. For example `serde_json` does not provide those hints.
//!
//! For that reason it is better not to implement `Deserialize` and let the
//! caller handle it manually if needed.

use serde::Serialize;

use osom_lib_alloc::traits::Allocator;

use crate::fixed_array::FixedArray;

impl<T: Serialize, TAllocator: Allocator> Serialize for FixedArray<T, TAllocator> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}
