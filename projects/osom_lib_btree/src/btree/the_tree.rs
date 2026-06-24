use core::ops::DerefMut;
use core::{marker::PhantomData, mem::ManuallyDrop};

use osom_lib_primitives::length::Length;
use osom_lib_reprc::traits::ReprC;

use super::BTreeConfig;
use super::node_ptr::BTreeNodePtr;

/// The main data structure for the B-tree algorithm.
#[repr(C)]
#[must_use]
#[derive(Debug)]
pub struct BTree<TKey, TValue, TConfig>
where
    TKey: Ord,
    TConfig: BTreeConfig,
{
    pub(super) data: BTreeNodePtr<TKey, TValue, TConfig>,
    pub(super) total_len: Length,
    pub(super) config: ManuallyDrop<TConfig>,
    _phantom: PhantomData<[(TKey, TValue)]>,
}

impl<TKey, TValue, TConfig> BTree<TKey, TValue, TConfig>
where
    TKey: Ord,
    TConfig: BTreeConfig,
{
    /// Creates a new [`BTree`] with the default configuration.
    ///
    /// Note: this method doesn't allocate anything.
    #[inline]
    pub fn new() -> Self
    where
        TConfig: Default,
    {
        Self::with_config(TConfig::default())
    }

    /// Creates a new [`BTree`] with the specified configuration.
    ///
    /// Note: this method doesn't allocate anything.
    #[inline]
    pub fn with_config(config: TConfig) -> Self {
        Self::new_unchecked(BTreeNodePtr::NULL, Length::ZERO, config)
    }

    /// Returns the number of key-value pairs in the [`BTree`].
    #[inline(always)]
    pub const fn len(&self) -> Length {
        self.total_len
    }

    #[inline(always)]
    const fn new_unchecked(data: BTreeNodePtr<TKey, TValue, TConfig>, total_len: Length, config: TConfig) -> Self {
        const {
            assert!(
                TConfig::CHILDREN_COUNT >= 4,
                "BTreeConfig::CHILDREN_COUNT must be greater or equal to 4"
            );
            assert!(
                TConfig::CHILDREN_COUNT < 65536,
                "BTreeConfig::CHILDREN_COUNT must be less than 65536"
            );
            assert!(
                TConfig::CHILDREN_COUNT.is_multiple_of(2),
                "BTreeConfig::CHILDREN_COUNT must be a multiple of 2"
            );
        }

        Self {
            data,
            total_len,
            config: ManuallyDrop::new(config),
            _phantom: PhantomData,
        }
    }
}

unsafe impl<TKey, TValue, TConfig> Send for BTree<TKey, TValue, TConfig>
where
    TKey: Ord + Send,
    TValue: Send,
    TConfig: BTreeConfig + Send,
{
}

unsafe impl<TKey, TValue, TConfig> Sync for BTree<TKey, TValue, TConfig>
where
    TKey: Ord + Sync,
    TValue: Sync,
    TConfig: BTreeConfig + Sync,
{
}

unsafe impl<TKey, TValue, TConfig> ReprC for BTree<TKey, TValue, TConfig>
where
    TKey: Ord + ReprC,
    TValue: ReprC,
    TConfig: BTreeConfig,
{
    const CHECK: () = const {
        osom_lib_reprc::hidden::is_reprc::<TKey>();
        osom_lib_reprc::hidden::is_reprc::<TValue>();
        osom_lib_reprc::hidden::is_reprc::<TConfig>();
    };
}

impl<TKey, TValue, TConfig> Default for BTree<TKey, TValue, TConfig>
where
    TKey: Ord,
    TConfig: BTreeConfig + Default,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<TKey, TValue, TConfig> Drop for BTree<TKey, TValue, TConfig>
where
    TKey: Ord,
    TConfig: BTreeConfig,
{
    fn drop(&mut self) {
        unsafe {
            if !self.data.is_null() {
                self.data.drop_recursively(self.config.deref_mut());
            }
            ManuallyDrop::drop(&mut self.config);
        }
    }
}
