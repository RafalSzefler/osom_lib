use core::{hash::Hash, marker::PhantomData, mem::ManuallyDrop, ptr::null_mut};
use core::{ops::Deref, ptr::NonNull};

use osom_lib_alloc::traits::Allocator as _;
use osom_lib_macros::debug_check_or_release_hint;
use osom_lib_primitives::{length::Length, power_of_two::PowerOfTwo32};
use osom_lib_reprc::traits::ReprC;

use crate::{
    abseil::{
        configuration::AbseilConfig,
        hash_table::{
            abseil_block::{AbseilBlock, CONTROL_BYTE_EMPTY},
            abseil_layout::{ABSEIL_BLOCK_SIZE, AbseilLayout},
            abseil_unsafe_iter::{AbseilUnsafeIter, AbseilUnsafeMutIter},
            platform::{PlatformImpl, PlatformOps},
        },
        utils::probe_block_indexes,
    },
    helpers::{KVP, ptr_to_mut, ptr_to_ref},
    traits::MutableHashTable,
};

/// The Abseil hash table.
#[repr(C)]
#[must_use]
pub struct AbseilHashTable<TKey, TValue, TConfig>
where
    TKey: Eq + Hash,
    TConfig: AbseilConfig,
{
    /// Pointer to the actual data.
    pub(super) control_data: *mut u8,

    /// Pointer to the (key, value) pairs. This is not an independent pointer.
    /// The memory it points to is owned by control_data. This is just a
    /// cached offset. For allocation and deallocation we use control_data only.
    pub(super) kvp_data: *mut u8,

    /// The number of elements in the table.
    pub(super) elements_count: Length,

    /// This fields indicates how many inserts can be performed
    /// before the table needs to be resized.
    pub(super) remaining_capacity: Length,

    /// The total capacity of the table.
    pub(super) total_capacity: PowerOfTwo32,

    /// The configuration of the table.
    pub(super) config: ManuallyDrop<TConfig>,

    /// Marker for the key and value types.
    _marker: PhantomData<KVP<TKey, TValue>>,
}

unsafe impl<TKey, TValue, TConfig> ReprC for AbseilHashTable<TKey, TValue, TConfig>
where
    TKey: Eq + Hash + ReprC,
    TValue: ReprC,
    TConfig: AbseilConfig + ReprC,
{
    const CHECK: () = const {
        let () = <*mut u8 as ReprC>::CHECK;
        let () = <TConfig as ReprC>::CHECK;
        let () = <PhantomData<KVP<TKey, TValue>> as ReprC>::CHECK;
    };
}

unsafe impl<TKey, TValue, TConfig> Send for AbseilHashTable<TKey, TValue, TConfig>
where
    TKey: Send + Eq + Hash,
    TValue: Send,
    TConfig: AbseilConfig + Send,
{
}

unsafe impl<TKey, TValue, TConfig> Sync for AbseilHashTable<TKey, TValue, TConfig>
where
    TKey: Sync + Eq + Hash,
    TValue: Sync,
    TConfig: AbseilConfig + Sync,
{
}

impl<TKey, TValue, TConfig> AbseilHashTable<TKey, TValue, TConfig>
where
    TKey: Eq + Hash,
    TConfig: AbseilConfig,
{
    /// Creates a new [`AbseilHashTable`] with the default configuration.
    #[inline(always)]
    pub fn new() -> Self {
        Self::with_config(TConfig::default())
    }

    /// Creates a new [`AbseilHashTable`] with the specified configuration.
    #[inline]
    pub const fn with_config(config: TConfig) -> Self {
        Self {
            control_data: null_mut(),
            kvp_data: null_mut(),
            elements_count: Length::ZERO,
            remaining_capacity: Length::ZERO,
            total_capacity: PowerOfTwo32::ZERO,
            config: ManuallyDrop::new(config),
            _marker: PhantomData,
        }
    }

    /// Creates a new [`AbseilHashTable`] with the specified configuration and capacity.
    #[inline]
    pub fn with_capacity_and_config(capacity: Length, config: TConfig) -> Self {
        let mut new_self = Self::with_config(config);
        new_self.grow_for_size(capacity);
        new_self
    }

    /// Creates a new [`AbseilHashTable`] with the capacity and default configuration.
    #[inline(always)]
    pub fn with_capacity(capacity: Length) -> Self {
        Self::with_capacity_and_config(capacity, TConfig::default())
    }

    /// Returns the length of the [`AbseilHashTable`].
    #[inline(always)]
    pub const fn length(&self) -> Length {
        self.elements_count
    }

    /// Returns the capacity of the [`AbseilHashTable`].
    #[inline(always)]
    pub const fn capacity(&self) -> Length {
        unsafe {
            let sum = self
                .elements_count
                .as_u32()
                .unchecked_add(self.remaining_capacity.as_u32());
            Length::new_unchecked(sum)
        }
    }

    #[inline(always)]
    pub(super) fn get_block_by_index(&self, index: usize) -> AbseilBlock<TKey, TValue> {
        unsafe {
            let control_block_ptr = self.control_data.cast::<[u8; ABSEIL_BLOCK_SIZE]>().add(index);
            let key_values_ptr = self
                .kvp_data
                .cast::<[KVP<TKey, TValue>; ABSEIL_BLOCK_SIZE]>()
                .add(index);
            AbseilBlock::new(control_block_ptr, key_values_ptr)
        }
    }

    #[inline]
    pub(super) fn blocks_count(&self) -> PowerOfTwo32 {
        let value = self.total_capacity.as_usize() / ABSEIL_BLOCK_SIZE;
        debug_check_or_release_hint!(value == 0 || value.is_power_of_two());
        unsafe { PowerOfTwo32::new_unchecked(value as u32) }
    }

    /// Allocates a new backing store (doubling capacity, or using 1 block if empty),
    /// initialises all control bytes to EMPTY, and rehashes every live element
    /// into the new table.  `remaining_capacity` is set to
    /// `floor(new_total_capacity * load_factor) – elements_count`.
    #[allow(clippy::cast_precision_loss, clippy::cast_sign_loss)]
    pub(super) fn grow_for_size(&mut self, size: Length) {
        let size = size.as_usize() as u64;
        assert!(size < u64::from(u32::MAX), "Max size exceeded, cannot grow so much");

        let new_size = (size as f64) * 1f64 / self.config.load_factor().value();
        assert!(new_size < f64::from(u32::MAX), "Max size exceeded, cannot grow so much");

        let new_size = new_size as usize;
        let blocks_capacity = unsafe { (new_size / ABSEIL_BLOCK_SIZE).unchecked_add(1) };
        let new_blocks_capacity = blocks_capacity.next_power_of_two();
        assert!(
            new_blocks_capacity < u32::MAX as usize / ABSEIL_BLOCK_SIZE,
            "Max capacity of AbseilHashTable exceeded"
        );

        let new_blocks_capacity = unsafe { PowerOfTwo32::new_unchecked(new_blocks_capacity as u32) };
        let new_layout = AbseilLayout::<TKey, TValue>::new(new_blocks_capacity);

        // Precalculate remaining_capacity
        let elements_capacity = new_blocks_capacity.as_usize() * ABSEIL_BLOCK_SIZE;
        let new_max_elements = elements_capacity as f64 * self.config.load_factor().value();
        let new_max_elements = new_max_elements as u64;
        assert!(
            new_max_elements < u64::from(u32::MAX),
            "Max size exceeded, cannot grow so much"
        );
        let new_max_elements = new_max_elements as u32;

        // Allocate new storage.
        let new_ptr = self
            .config
            .allocator()
            .allocate(new_layout.total_layout())
            .expect("AbseilHashTable: allocation failed")
            .as_ptr();

        // Fill initial control bytes.
        let control_bytes = unsafe { core::slice::from_raw_parts_mut(new_ptr, new_layout.control_blocks_size()) };
        control_bytes.fill(CONTROL_BYTE_EMPTY);

        // Set new data
        let mut new_self = Self {
            control_data: new_ptr,
            kvp_data: unsafe { new_ptr.add(new_layout.key_value_pairs_offset()) },
            elements_count: Length::ZERO,
            remaining_capacity: unsafe { Length::new_unchecked(new_max_elements) },
            total_capacity: unsafe { PowerOfTwo32::new_unchecked(elements_capacity as u32) },
            config: self.config.clone(),
            _marker: PhantomData,
        };

        // Rehash data if needed
        for kvp in AbseilUnsafeMutIter::from_hash_table(self) {
            let kvp = unsafe { kvp.read() };
            unsafe { new_self.insert_without_conflict_and_resize(kvp.key, kvp.value) };
        }

        // Drop old memory. We don't need to loop through items though, they have been moved.
        // So just deallocate memory, and drop the config.
        self.deconstruct_buffer();

        // Swap self with new_self, and forget the previous value
        core::mem::swap(self, &mut new_self);
        core::mem::forget(new_self);
    }

    #[inline(always)]
    fn deconstruct_buffer(&mut self) {
        if self.control_data.is_null() {
            return;
        }
        let data = unsafe { NonNull::new_unchecked(self.control_data) };
        let layout = AbseilLayout::<TKey, TValue>::new(self.blocks_count());
        unsafe {
            self.config.allocator().deallocate(data, layout.total_layout());
            ManuallyDrop::drop(&mut self.config);
        }
    }

    #[inline]
    fn deconstruct_buffer_and_drop_data(&mut self) {
        if self.control_data.is_null() {
            return;
        }

        if core::mem::needs_drop::<TKey>() || core::mem::needs_drop::<TValue>() {
            for kvp in AbseilUnsafeMutIter::from_hash_table(self) {
                let kvp = ptr_to_mut!(kvp);
                unsafe {
                    core::ptr::drop_in_place(&raw mut kvp.key);
                    core::ptr::drop_in_place(&raw mut kvp.value);
                }
            }
        }

        self.deconstruct_buffer();
    }

    /// This function inserts (key, value) pair. It is an optimized variant of insert,
    /// where we know for sure that `key` is not present, that resize won't happen,
    /// and that table does not contain tombstones. This should be used on rehashing only.
    unsafe fn insert_without_conflict_and_resize(&mut self, key: TKey, value: TValue) {
        let (h1, h2) = self.config.calculate_partial_hashes(&key);
        let blocks_count = self.blocks_count();

        for group_index in probe_block_indexes(h1, blocks_count) {
            let block = self.get_block_by_index(group_index);
            let control_bytes = ptr_to_mut!(block.control_block_ptr());
            let mut scan_result = PlatformImpl::empty_scan(control_bytes);

            if let Some(empty_idx) = scan_result.next() {
                // Empty slot proves the key is absent. Prefer tombstone (reuse deleted slot)
                // over the empty slot when available.
                self.remaining_capacity =
                    unsafe { Length::new_unchecked(self.remaining_capacity.as_u32().unchecked_sub(1)) };
                let (target_group, target_slot) = (group_index, empty_idx);

                let target_block = self.get_block_by_index(target_group);
                let target_ctrl = ptr_to_mut!(target_block.control_block_ptr());
                let target_kvp_ptr = target_block.key_value_pair_at_index(target_slot);

                unsafe {
                    *target_ctrl.get_unchecked_mut(target_slot) = h2;
                    target_kvp_ptr.write(KVP { key, value });
                };

                unsafe {
                    self.elements_count = Length::new_unchecked(self.elements_count.as_u32().unchecked_add(1));
                }

                return;
            }
        }

        // With correct remaining_capacity management there is always at least one
        // empty slot in the table, so this path is unreachable.
        unreachable!("no empty slot found despite remaining_capacity > 0")
    }
}

impl<TKey, TValue, TConfig> Default for AbseilHashTable<TKey, TValue, TConfig>
where
    TKey: Eq + Hash,
    TConfig: AbseilConfig,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<TKey, TValue, TConfig> Clone for AbseilHashTable<TKey, TValue, TConfig>
where
    TKey: Eq + Hash + Clone,
    TValue: Clone,
    TConfig: AbseilConfig + Clone,
{
    fn clone(&self) -> Self {
        let mut new_hashtable = Self::with_capacity_and_config(self.elements_count, self.config.deref().clone());
        for kvp in AbseilUnsafeIter::from_hash_table(self) {
            let kvp = ptr_to_ref!(kvp);
            new_hashtable.insert(kvp.key.clone(), kvp.value.clone());
        }
        new_hashtable
    }
}

impl<TKey, TValue, TConfig> PartialEq for AbseilHashTable<TKey, TValue, TConfig>
where
    TKey: Eq + Hash,
    TValue: PartialEq,
    TConfig: AbseilConfig,
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

impl<TKey, TValue, TConfig> Eq for AbseilHashTable<TKey, TValue, TConfig>
where
    TKey: Eq + Hash,
    TValue: Eq,
    TConfig: AbseilConfig,
{
}

impl<TKey, TValue, TConfig> core::hash::Hash for AbseilHashTable<TKey, TValue, TConfig>
where
    TKey: Eq + Hash,
    TValue: Hash,
    TConfig: AbseilConfig,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        use crate::traits::ImmutableHashTable;
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

impl<TKey, TValue, TConfig> Drop for AbseilHashTable<TKey, TValue, TConfig>
where
    TKey: Eq + Hash,
    TConfig: AbseilConfig,
{
    fn drop(&mut self) {
        self.deconstruct_buffer_and_drop_data();
    }
}
