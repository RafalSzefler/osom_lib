use osom_lib_alloc::traits::Allocator;
use osom_lib_hash_tables::defaults::DefaultHashTable;
use osom_lib_reprc::macros::reprc;
use osom_lib_strings::immutable::ImmutableString;
use osom_lib_try_clone::TryClone;

/// The errors that can occur when dealing with [`CVRDeserializeContext`].
#[reprc]
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Hash)]
#[must_use]
pub enum CVRDeserializeContextError {
    /// The allocator failed to be cloned.
    FailedToCloneAllocator = 0,
}

/// This is the `serde` deserialization context for [`CVR`][crate::cvr::CVR].
#[reprc]
#[derive(Debug, PartialEq, Eq, Hash)]
#[must_use]
pub struct CVRDeserializeContext<TAllocator: Allocator + TryClone> {
    allocator: TAllocator,
    string_cache: DefaultHashTable<ImmutableString<TAllocator>, (), TAllocator>,
}

impl<TAllocator: Allocator + TryClone> CVRDeserializeContext<TAllocator> {
    /// Creates a new [`CVRDeserializeContext`] instance with the default allocator.
    /// 
    /// # Errors
    /// 
    /// See [`CVRDeserializeContextError`] for details.
    #[inline]
    pub fn new() -> Result<Self, CVRDeserializeContextError>
    where TAllocator: Default,
    {
        Self::with_allocator(TAllocator::default())
    }

    /// Creates a new [`CVRDeserializeContext`] instance with the given allocator.
    /// 
    /// # Errors
    /// 
    /// See [`CVRDeserializeContextError`] for details.
    #[inline]
    pub fn with_allocator(allocator: TAllocator) -> Result<Self, CVRDeserializeContextError> {
        let allocator_clone = allocator.try_clone()
            .map_err(|_| CVRDeserializeContextError::FailedToCloneAllocator)?;
        let string_cache = DefaultHashTable::with_allocator(allocator_clone);
        Ok(Self { allocator, string_cache })
    }

    #[inline(always)]
    pub(super) const fn allocator(&self) -> &TAllocator {
        &self.allocator
    }

    #[inline(always)]
    pub(super) const fn string_cache_mut(&mut self) -> &mut DefaultHashTable<ImmutableString<TAllocator>, (), TAllocator> {
        &mut self.string_cache
    }
}

macro_rules! make_seed_struct {
    ( $name:ident ) => {
        /// This struct is used to create a seed for the `serde` deserialization.
        #[osom_lib_reprc::macros::reprc]
        #[repr(transparent)]
        #[must_use]
        #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
        pub struct $name<'a, TAllocator: Allocator + TryClone> {
            pub context: &'a mut super::CVRDeserializeContext<TAllocator>,
        }
    };
}
pub(super) use make_seed_struct;
