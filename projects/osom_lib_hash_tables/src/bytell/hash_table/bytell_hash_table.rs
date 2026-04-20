use core::{
    alloc::Layout,
    borrow::Borrow,
    hash::{BuildHasher, Hash},
    marker::PhantomData,
    mem::ManuallyDrop,
    ops::Deref,
    ptr::{self, NonNull},
};

#[allow(unused_imports)]
use core::fmt::Debug;

use osom_lib_alloc::traits::Allocator;
use osom_lib_macros::debug_check_or_release_hint;
use osom_lib_primitives::{length::Length, power_of_two::PowerOfTwo32};
use osom_lib_reprc::traits::ReprC;

use crate::{
    bytell::{
        configuration::BytellConfig,
        constants::JUMP_DISTANCES,
        errors::BytellError,
        hash_table::{block_layout::BlockLayoutHolder, control_byte::ControlByte, entry::Entry},
        hash_to_index::HashToIndex,
    },
    helpers::{KVP, ptr_to_mut, ptr_to_ref},
    traits::{ImmutableHashTable, MutableHashTable},
};

/// The bytell hash table.
#[repr(C)]
#[must_use]
pub struct BytellHashTable<TKey, TValue, TConfig>
where
    TKey: Eq + Hash,
    TConfig: BytellConfig,
{
    pub(super) data: *mut u8,
    pub(super) elements_count: Length,
    pub(super) blocks_count: PowerOfTwo32,
    pub(super) config: ManuallyDrop<TConfig>,
    _marker: PhantomData<KVP<TKey, TValue>>,
}

unsafe impl<TKey, TValue, TConfig> ReprC for BytellHashTable<TKey, TValue, TConfig>
where
    TKey: Eq + Hash + ReprC,
    TValue: ReprC,
    TConfig: BytellConfig + ReprC,
{
    const CHECK: () = const {
        let () = <*mut u8 as ReprC>::CHECK;
        let () = <Length as ReprC>::CHECK;
        let () = <PowerOfTwo32 as ReprC>::CHECK;
        let () = <TConfig as ReprC>::CHECK;
        let () = <PhantomData<KVP<TKey, TValue>> as ReprC>::CHECK;
    };
}

unsafe impl<TKey, TValue, TConfig> Send for BytellHashTable<TKey, TValue, TConfig>
where
    TKey: Send + Eq + Hash,
    TValue: Send,
    TConfig: BytellConfig + Send,
{
}

unsafe impl<TKey, TValue, TConfig> Sync for BytellHashTable<TKey, TValue, TConfig>
where
    TKey: Sync + Eq + Hash,
    TValue: Sync,
    TConfig: BytellConfig + Sync,
{
}

impl<TKey, TValue, TConfig> BytellHashTable<TKey, TValue, TConfig>
where
    TKey: Eq + Hash,
    TConfig: BytellConfig,
{
    /// Creates a new [`BytellHashTable`] with the default configuration.
    #[inline(always)]
    pub fn new() -> Self {
        Self::with_config(TConfig::default())
    }

    /// Creates a new [`BytellHashTable`] with the specified configuration.
    #[inline]
    pub const fn with_config(config: TConfig) -> Self {
        Self {
            data: ptr::null_mut(),
            elements_count: Length::ZERO,
            blocks_count: PowerOfTwo32::ZERO,
            config: ManuallyDrop::new(config),
            _marker: PhantomData,
        }
    }

    /// Creates a new [`BytellHashTable`] with the specified capacity and the default configuration.
    ///
    /// # Errors
    ///
    /// Returns [`BytellError::AllocationError`] if the allocation fails.
    ///
    /// # Panics
    ///
    /// If the expected_capacity exceeds [`u32::MAX`]
    #[inline(always)]
    pub fn with_capacity(number_of_items: u32) -> Result<Self, BytellError> {
        Self::with_capacity_and_config(number_of_items, TConfig::default())
    }

    /// Creates a new [`BytellHashTable`] expected to support passed `number_of_items`.
    ///
    /// # Notes
    ///
    /// This method will almost surely overallocate, since it takes `config.load_factor()`
    /// into account.
    ///
    /// # Errors
    ///
    /// Returns [`BytellError::AllocationError`] if the allocation fails.
    ///
    /// # Panics
    ///
    /// If the expected_capacity exceeds [`u32::MAX`].
    pub fn with_capacity_and_config(number_of_items: u32, config: TConfig) -> Result<Self, BytellError> {
        let block_layout = BlockLayoutHolder::<TKey, TValue>::LAYOUT;
        let block_capacity = block_layout.block_capacity().value();
        debug_check_or_release_hint!(block_capacity >= 4, "block_capacity is less than 4");
        let block_binary_layout = block_layout.layout();
        let block_size = block_binary_layout.size() as u64;
        debug_check_or_release_hint!(
            u32::try_from(block_size).is_ok(),
            "block_size expected to be at most u32::MAX"
        );

        #[allow(clippy::cast_sign_loss)]
        let expected_capacity = ((f64::from(number_of_items)) / config.load_factor().value()) as u64 + 1;

        assert!(
            expected_capacity < u64::from(u32::MAX),
            "The number_of_items is too big"
        );
        let expected_capacity = expected_capacity as u32;
        let expected_number_of_buckets = (expected_capacity / block_capacity) + 1;
        let number_of_buckets = expected_number_of_buckets.next_power_of_two();
        let block_count = unsafe { PowerOfTwo32::new_unchecked(number_of_buckets) };
        Self::with_block_count_and_config(block_count, config)
    }

    /// Reduces the memory of the current table, if possible.
    ///
    /// # Notes
    ///
    /// If the number of underlying blocks cannot be reduced, it
    /// does nothing.
    ///
    /// Otherwise it tries to reduce the number of blocks to the
    /// minimal possible, creates a new table with that layout,
    /// and moves (rehases) items into it. Then overwrites `self`
    /// with the new table.
    ///
    /// # Errors
    ///
    /// Returns [`BytellError::AllocationError`] if it cannot allocate
    /// memory for the new hidden table.
    pub fn shrink_to_fit(&mut self) -> Result<(), BytellError> {
        let current_size = self.elements_count;
        let mut new_blocks_count = unsafe { PowerOfTwo32::new_unchecked(current_size.as_u32().next_power_of_two()) };
        if Self::static_should_grow(current_size, new_blocks_count, &self.config) {
            new_blocks_count = new_blocks_count.next();
        }

        if new_blocks_count == self.blocks_count {
            return Ok(());
        }

        let mut new_table = Self::with_block_count_and_config(new_blocks_count, self.config.deref().clone())?;
        self.move_content_to(&mut new_table);
        core::mem::swap(self, &mut new_table);
        Ok(())
    }

    fn with_block_count_and_config(block_count: PowerOfTwo32, config: TConfig) -> Result<Self, BytellError> {
        let mut new_table = Self::with_config(config);
        if block_count == PowerOfTwo32::ZERO {
            return Ok(new_table);
        }

        let block_capacity = BlockLayoutHolder::<TKey, TValue>::LAYOUT.block_capacity().as_usize();
        let block_layout = BlockLayoutHolder::<TKey, TValue>::LAYOUT.layout();
        let total_size = block_layout.size() * block_count.as_usize();
        let layout = unsafe { Layout::from_size_align_unchecked(total_size, block_layout.align()) };
        let ptr = new_table.config.allocator().allocate(layout).map_err(Into::into)?;
        new_table.data = ptr.as_ptr();
        new_table.blocks_count = block_count;
        let new_capacity = new_table.capacity();
        new_table
            .config
            .hash_to_index_mut()
            .update_for_new_table_capacity(new_capacity);

        // Initialize all control bytes to EMPTY
        let mut block_ptr = new_table.data;
        for _ in 0..new_table.blocks_count.value() {
            unsafe {
                block_ptr.write_bytes(ControlByte::EMPTY.binary_value(), block_capacity);
                block_ptr = block_ptr.add(block_layout.size());
            }
        }

        Ok(new_table)
    }

    /// Returns the length of the [`BytellHashTable`].
    #[inline(always)]
    pub const fn length(&self) -> Length {
        self.elements_count
    }

    /// Returns the capacity of the [`BytellHashTable`].
    #[inline(always)]
    pub const fn capacity(&self) -> PowerOfTwo32 {
        let block_capacity = BlockLayoutHolder::<TKey, TValue>::LAYOUT.block_capacity().value();
        let result = self.blocks_count.value() * block_capacity;
        debug_check_or_release_hint!(result == 0 || result.is_power_of_two(), "result is not a power of two");
        unsafe { PowerOfTwo32::new_unchecked(result) }
    }

    #[inline(always)]
    pub(super) const unsafe fn get_entry_by_index(&self, index: usize) -> Entry<TKey, TValue> {
        Entry::new(self.data, self.blocks_count, index)
    }

    #[inline(always)]
    pub(super) fn should_grow(&self) -> bool {
        Self::static_should_grow(self.elements_count, self.capacity(), &self.config)
    }

    #[inline]
    fn static_should_grow(current_size: Length, current_capacity: PowerOfTwo32, config: &TConfig) -> bool {
        #[allow(clippy::cast_sign_loss)]
        {
            let capacity = f64::from(current_capacity.value());
            let threshold = config.load_factor().value() * capacity;
            f64::from(current_size.as_u32() + 1) > threshold
        }
    }

    #[allow(clippy::used_underscore_binding)]
    pub(super) fn grow(&mut self) {
        let old_block_count = self.blocks_count.as_usize();
        let new_block_count = (old_block_count + 1).next_power_of_two();
        debug_check_or_release_hint!(new_block_count < u32::MAX as usize, "Too many blocks");
        let new_block_count = unsafe { PowerOfTwo32::new_unchecked(new_block_count as u32) };
        let mut new_table = Self::with_block_count_and_config(new_block_count, self.config.deref().clone())
            .expect("Failed to allocate new bytell hash table");
        self.move_content_to(&mut new_table);
        debug_assert!(self.elements_count == Length::ZERO, "self is not empty");
        debug_assert!(!new_table.data.is_null(), "data is null");
        core::mem::swap(self, &mut new_table);
    }

    pub(super) unsafe fn get_entry_by_key<Q>(&self, key: &Q) -> Entry<TKey, TValue>
    where
        TKey: Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        let hash_value = self.config.build_hasher().hash_one(key);
        let index = self.config.hash_to_index().hash_to_index(hash_value, self.capacity());
        unsafe { self.get_entry_by_index(index) }
    }

    pub(super) fn search_for_free_entry(
        &self,
        current_entry: &Entry<TKey, TValue>,
    ) -> Option<(Entry<TKey, TValue>, u8)> {
        debug_check_or_release_hint!(JUMP_DISTANCES.len() < u8::MAX as usize);
        let current_entry_index = current_entry.element_index() as usize;
        let capacity = self.capacity().value() as usize;
        debug_check_or_release_hint!(capacity > 0, "capacity is zero");
        for (index, jmp_distance) in JUMP_DISTANCES.iter().enumerate().skip(1) {
            let real_offset = jmp_distance.wrapping_add(current_entry_index) & (capacity - 1);
            let entry = Entry::<TKey, TValue>::new(self.data, self.blocks_count, real_offset);
            unsafe {
                if *entry.control_byte() == ControlByte::EMPTY {
                    return Some((entry, index as u8));
                }
            }
        }

        None
    }

    pub(super) unsafe fn find_parent_for_storage_entry(&self, entry: &Entry<TKey, TValue>) -> Entry<TKey, TValue> {
        unsafe {
            debug_check_or_release_hint!(
                !ptr_to_ref!(entry.control_byte()).is_direct_hit(),
                "find_parent_for_storage_entry expects storage entry, got direct hit"
            );
            let key = &ptr_to_ref!(entry.kvp()).key;
            let mut current = self.get_entry_by_key(key);
            debug_check_or_release_hint!(
                ptr_to_ref!(current.control_byte()).is_direct_hit(),
                "get_entry_by_key did not return direct hit"
            );
            debug_check_or_release_hint!(&current != entry, "current == entry should not happen");

            loop {
                let next_link = current.next_link();
                debug_check_or_release_hint!(
                    next_link.is_some(),
                    "next_link is None, that shouldn't have happened, but it did"
                );
                let next = next_link.unwrap_unchecked();
                if &next == entry {
                    return current;
                }
                current = next;
            }
        }
    }

    pub(super) fn move_content_to(&mut self, other: &mut Self) {
        debug_check_or_release_hint!(self.data != other.data);

        unsafe {
            let capacity = self.capacity().as_usize();
            let mut el_idx = 0;
            let mut remaining_items = self.elements_count.as_u32();
            while el_idx < capacity {
                if remaining_items == 0 {
                    break;
                }

                let entry = Entry::<TKey, TValue>::new(self.data, self.blocks_count, el_idx);
                el_idx += 1;

                let control_byte = ptr_to_mut!(entry.control_byte());
                if !control_byte.contains_data() {
                    continue;
                }
                *control_byte = ControlByte::EMPTY;

                let kvp = entry.kvp().read();

                other.insert_or_update_with(kvp.key, || kvp.value, |_| panic!("Update should not happen"));

                remaining_items -= 1;
            }
            debug_check_or_release_hint!(remaining_items == 0, "Moved less items than was supposed to");
            self.elements_count = Length::ZERO;
        }
    }

    pub(super) fn clone_content_to(&self, other: &mut Self)
    where
        TKey: Clone,
        TValue: Clone,
    {
        let capacity = self.capacity().as_usize();
        let mut el_idx = 0;
        while el_idx < capacity {
            let entry = Entry::<TKey, TValue>::new(self.data, self.blocks_count, el_idx);
            el_idx += 1;

            let control_byte = ptr_to_ref!(entry.control_byte());
            if !control_byte.contains_data() {
                continue;
            }

            let kvp = ptr_to_ref!(entry.kvp());
            other.insert(kvp.key.clone(), kvp.value.clone());
        }
    }
}

impl<TKey, TValue, TConfig> Default for BytellHashTable<TKey, TValue, TConfig>
where
    TKey: Eq + Hash,
    TConfig: BytellConfig,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<TKey, TValue, TConfig> Clone for BytellHashTable<TKey, TValue, TConfig>
where
    TKey: Eq + Hash + Clone,
    TValue: Clone,
    TConfig: BytellConfig,
{
    fn clone(&self) -> Self {
        let mut new_table = Self::with_capacity_and_config(self.elements_count.as_u32(), self.config.deref().clone())
            .expect("Failed to allocate new bytell hash table");
        self.clone_content_to(&mut new_table);
        new_table
    }
}

impl<TKey, TValue, TConfig> PartialEq for BytellHashTable<TKey, TValue, TConfig>
where
    TKey: Eq + Hash,
    TValue: PartialEq,
    TConfig: BytellConfig,
{
    fn eq(&self, other: &Self) -> bool {
        use crate::traits::ImmutableHashTable;
        if self.length() != other.length() {
            return false;
        }
        for (key, value) in self.iter() {
            let Some(other_value) = other.get(key) else {
                return false;
            };
            if other_value != value {
                return false;
            }
        }
        true
    }
}

impl<TKey, TValue, TConfig> Eq for BytellHashTable<TKey, TValue, TConfig>
where
    TKey: Eq + Hash,
    TValue: Eq,
    TConfig: BytellConfig,
{
}

impl<TKey, TValue, TConfig> core::hash::Hash for BytellHashTable<TKey, TValue, TConfig>
where
    TKey: Eq + Hash,
    TValue: Hash,
    TConfig: BytellConfig,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        use core::hash::Hasher;
        use osom_lib_hashes::siphash::SipHashBuilder;

        // We need to ensure that calculating the hash does not depend on the order of iteration,
        // which is not guaranteed at all.
        //
        // Therefore, we first calculate a temporary hash, and then add all of those hashes
        // together. We take advantage of the fact that addition is commutative and associative.
        let sip_hash_builder = SipHashBuilder::with_keys(0, u64::from(self.length().as_u32()));
        let mut result = 0u64;
        for (key, value) in self.iter() {
            let mut sip_hash = sip_hash_builder.create_hasher();
            key.hash(&mut sip_hash);
            value.hash(&mut sip_hash);
            result = result.wrapping_add(sip_hash.finish());
        }
        state.write_u64(result);
    }
}

impl<TKey, TValue, TConfig> Drop for BytellHashTable<TKey, TValue, TConfig>
where
    TKey: Eq + Hash,
    TConfig: BytellConfig,
{
    fn drop(&mut self) {
        unsafe {
            if core::mem::needs_drop::<TKey>() || core::mem::needs_drop::<TValue>() {
                let capacity = self.capacity().as_usize();
                let mut el_idx = 0;
                let mut remaining_items = self.elements_count.as_u32();

                while el_idx < capacity {
                    if remaining_items == 0 {
                        break;
                    }
                    let entry = self.get_entry_by_index(el_idx);
                    el_idx += 1;
                    if !(*entry.control_byte()).contains_data() {
                        continue;
                    }
                    let kvp = ptr_to_mut!(entry.kvp());
                    core::ptr::drop_in_place(&raw mut kvp.key);
                    core::ptr::drop_in_place(&raw mut kvp.value);
                    remaining_items -= 1;
                }
            }

            let blocks_count = self.blocks_count.as_usize();
            if blocks_count > 0 {
                let block_size = BlockLayoutHolder::<TKey, TValue>::LAYOUT.layout().size();
                let block_align = BlockLayoutHolder::<TKey, TValue>::LAYOUT.layout().align();
                let layout = Layout::from_size_align_unchecked(blocks_count * block_size, block_align);
                self.config
                    .allocator()
                    .deallocate(NonNull::new_unchecked(self.data), layout);
            }

            ManuallyDrop::drop(&mut self.config);
        }
    }
}
