#![allow(clippy::cast_ptr_alignment)]
use core::alloc::Layout;
use core::marker::PhantomData;
use core::ptr::NonNull;

use osom_lib_alloc::traits::Allocator as _;
use osom_lib_macros::debug_check_or_release_hint;
use osom_lib_reprc::traits::ReprC;

use super::BTreeConfig;
use super::node_layout::BTreeNodeLayout;

use crate::errors::BTreeError;

#[repr(transparent)]
pub struct BTreeNodePtr<TKey, TValue, TConfig>
where
    TKey: Ord,
    TConfig: BTreeConfig,
{
    ptr: *mut u8,
    _phantom1: PhantomData<[(TKey, TValue)]>,
    _phantom2: PhantomData<TConfig>,
}

impl<TKey, TValue, TConfig> core::fmt::Debug for BTreeNodePtr<TKey, TValue, TConfig>
where
    TKey: Ord,
    TConfig: BTreeConfig,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let ptr_value = self.ptr.addr();
        write!(f, "{ptr_value:#x}")
    }
}

unsafe impl<TKey, TValue, TConfig> ReprC for BTreeNodePtr<TKey, TValue, TConfig>
where
    TKey: Ord + ReprC,
    TValue: ReprC,
    TConfig: BTreeConfig,
{
    const CHECK: () = const {
        osom_lib_reprc::hidden::is_reprc::<*mut u8>();
        osom_lib_reprc::hidden::is_reprc::<TKey>();
        osom_lib_reprc::hidden::is_reprc::<TValue>();
        osom_lib_reprc::hidden::is_reprc::<TConfig>();
    };
}

impl<TKey, TValue, TConfig> Clone for BTreeNodePtr<TKey, TValue, TConfig>
where
    TKey: Ord,
    TConfig: BTreeConfig,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<TKey, TValue, TConfig> Copy for BTreeNodePtr<TKey, TValue, TConfig>
where
    TKey: Ord,
    TConfig: BTreeConfig,
{
}

impl<TKey, TValue, TConfig> PartialEq for BTreeNodePtr<TKey, TValue, TConfig>
where
    TKey: Ord,
    TConfig: BTreeConfig,
{
    fn eq(&self, other: &Self) -> bool {
        core::ptr::eq(self.ptr, other.ptr)
    }
}

impl<TKey, TValue, TConfig> Eq for BTreeNodePtr<TKey, TValue, TConfig>
where
    TKey: Ord,
    TConfig: BTreeConfig,
{
}

impl<TKey, TValue, TConfig> BTreeNodePtr<TKey, TValue, TConfig>
where
    TKey: Ord,
    TConfig: BTreeConfig,
{
    const INTERNAL_NODE_LAYOUT: BTreeNodeLayout<TKey, TValue, TConfig> = BTreeNodeLayout::new(false);
    const LEAF_NODE_LAYOUT: BTreeNodeLayout<TKey, TValue, TConfig> = BTreeNodeLayout::new(true);

    pub const NULL: Self = Self::new_unchecked(core::ptr::null_mut());

    pub fn allocate_internal_node(config: &mut TConfig) -> Result<Self, BTreeError> {
        let ptr = config
            .allocator_mut()
            .allocate(Self::INTERNAL_NODE_LAYOUT.total_layout)
            .map_err(|_| BTreeError::AllocationError)?
            .as_ptr();

        unsafe { ptr.write_bytes(0, Self::INTERNAL_NODE_LAYOUT.total_layout.size()) };

        Ok(Self::new_unchecked(ptr))
    }

    pub fn allocate_leaf_node(config: &mut TConfig) -> Result<Self, BTreeError> {
        let ptr = config
            .allocator_mut()
            .allocate(Self::LEAF_NODE_LAYOUT.total_layout)
            .map_err(|_| BTreeError::AllocationError)?
            .as_ptr();

        unsafe { ptr.write_bytes(0, Self::LEAF_NODE_LAYOUT.total_layout.size()) };

        Ok(Self::new_unchecked(ptr))
    }

    #[inline]
    pub fn memory_layout(self) -> Layout {
        unsafe {
            if self.is_leaf() {
                Self::LEAF_NODE_LAYOUT.total_layout
            } else {
                Self::INTERNAL_NODE_LAYOUT.total_layout
            }
        }
    }

    #[inline(always)]
    pub unsafe fn deallocate(self, config: &mut TConfig) {
        debug_check_or_release_hint!(!self.is_null(), "deallocate: node is null");
        unsafe {
            let layout = self.memory_layout();
            config
                .allocator_mut()
                .deallocate(NonNull::new_unchecked(self.ptr), layout);
        }
    }

    pub unsafe fn drop_recursively(&mut self, config: &mut TConfig) {
        debug_check_or_release_hint!(!self.is_null(), "drop_recursively: node is null");

        unsafe {
            if !self.is_leaf() {
                for index in 0..=self.len() {
                    let mut child = self.children_ptr().add(index).read();
                    if child.is_null() {
                        continue;
                    }
                    child.drop_recursively(config);
                }
            }

            // Two separate loops for better cache locality.
            if core::mem::needs_drop::<TKey>() {
                for index in 0..self.len() {
                    let key = self.keys_ptr().add(index);
                    core::ptr::drop_in_place(key);
                }
            }
            if core::mem::needs_drop::<TValue>() {
                for index in 0..self.len() {
                    let value = self.values_ptr().add(index);
                    core::ptr::drop_in_place(value);
                }
            }

            self.deallocate(config);
        }
    }

    #[inline(always)]
    pub fn is_null(self) -> bool {
        self.ptr.is_null()
    }

    #[inline]
    pub unsafe fn is_leaf(self) -> bool {
        // By maintaining the B Tree invariants we can ensure that
        // internal node always has the first child.
        unsafe { self.children_ptr().read().is_null() }
    }

    #[inline]
    pub const unsafe fn keys_ptr(self) -> *mut TKey {
        unsafe { self.ptr.add(Self::INTERNAL_NODE_LAYOUT.keys_offset).cast::<TKey>() }
    }

    #[inline]
    pub const unsafe fn children_ptr(self) -> *mut Self {
        unsafe { self.ptr.add(Self::INTERNAL_NODE_LAYOUT.children_offset).cast::<Self>() }
    }

    #[inline]
    pub unsafe fn values_ptr(self) -> *mut TValue {
        unsafe { self.ptr.add(Self::INTERNAL_NODE_LAYOUT.values_offset).cast::<TValue>() }
    }

    #[inline]
    pub unsafe fn parent_ptr(self) -> *mut Self {
        unsafe { self.ptr.add(Self::INTERNAL_NODE_LAYOUT.parent_offset).cast::<Self>() }
    }

    /// Returns the number of stored (key, value) pairs.
    #[inline(always)]
    pub unsafe fn len(self) -> usize {
        unsafe { self.len_ptr().read() }
    }

    /// Returns a pointer to the number of stored (key, value) pairs.
    #[inline]
    pub unsafe fn len_ptr(self) -> *mut usize {
        unsafe { self.ptr.add(Self::INTERNAL_NODE_LAYOUT.len_offset).cast::<usize>() }
    }

    #[inline]
    pub unsafe fn is_full(self) -> bool {
        unsafe { self.len() == TConfig::CHILDREN_COUNT - 1 }
    }

    /// Inserts a key-value pair at the given index, shifting
    /// the remaining keys and values to the right.
    ///
    /// Note: it doesn't modify children pointers or length.
    pub unsafe fn insert_at(self, index: usize, key: TKey, value: TValue) {
        unsafe {
            let len = self.len();
            debug_check_or_release_hint!(index <= len, "insert_at: index out of node bounds");
            if index < len {
                self.keys_ptr()
                    .add(index + 1)
                    .copy_from(self.keys_ptr().add(index), len - index);
                self.values_ptr()
                    .add(index + 1)
                    .copy_from(self.values_ptr().add(index), len - index);
            }
            self.keys_ptr().add(index).write(key);
            self.values_ptr().add(index).write(value);
        }
    }

    /// Inserts a child at the given index, shifting
    /// the remaining children to the right. It does
    /// not modify the length of the node.
    ///
    /// Note: it doesn't modify keys and values or length.
    pub unsafe fn insert_child_at(self, index: usize, child: Self) {
        unsafe {
            let len = self.len();
            debug_check_or_release_hint!(index <= len + 1, "insert_child_at: index out of bounds");
            // We always have one more child than keys.
            if index < len + 1 {
                self.children_ptr()
                    .add(index + 1)
                    .copy_from(self.children_ptr().add(index), len - index + 1);
            }
            self.children_ptr().add(index).write(child);
        }
    }

    /// Splits `self` into two nodes and returns the promoted separator `(key, value)` and the right sibling.
    pub unsafe fn split(self, config: &mut TConfig) -> Result<(TKey, TValue, Self), BTreeError> {
        unsafe {
            debug_check_or_release_hint!(self.is_full(), "split: node is not full");
            let total = self.len();
            let left_size = TConfig::CHILDREN_COUNT / 2;
            let right_size = total - left_size - 1;
            let is_leaf = self.is_leaf();
            let right = if is_leaf {
                Self::allocate_leaf_node(config)?
            } else {
                Self::allocate_internal_node(config)?
            };

            let promoted_key = self.keys_ptr().add(left_size).read();
            let promoted_value = self.values_ptr().add(left_size).read();

            right
                .keys_ptr()
                .copy_from_nonoverlapping(self.keys_ptr().add(left_size + 1), right_size);
            right
                .values_ptr()
                .copy_from_nonoverlapping(self.values_ptr().add(left_size + 1), right_size);
            right.len_ptr().write(right_size);
            self.len_ptr().write(left_size);

            #[cfg(debug_assertions)]
            {
                self.keys_ptr().add(left_size + 1).write_bytes(0, right_size);
                self.values_ptr().add(left_size + 1).write_bytes(0, right_size);
            }

            if !is_leaf {
                right
                    .children_ptr()
                    .copy_from_nonoverlapping(self.children_ptr().add(left_size + 1), right_size + 1);

                #[cfg(debug_assertions)]
                self.children_ptr().add(left_size + 1).write_bytes(0, right_size + 1);
            }

            Ok((promoted_key, promoted_value, right))
        }
    }

    /// Removes a key-value pair at the given index, shifting
    /// the remaining keys and values to the left.
    ///
    /// Note: it doesn't modify children pointers or length.
    pub unsafe fn remove_at(self, index: usize) {
        unsafe {
            let len = self.len();
            debug_check_or_release_hint!(index < len, "remove_at: index out of node bounds");
            if index < len - 1 {
                self.keys_ptr()
                    .add(index)
                    .copy_from(self.keys_ptr().add(index + 1), len - index - 1);
                self.values_ptr()
                    .add(index)
                    .copy_from(self.values_ptr().add(index + 1), len - index - 1);
            } else if cfg!(debug_assertions) {
                self.keys_ptr().add(index).write_bytes(0, 1);
                self.values_ptr().add(index).write_bytes(0, 1);
            }
        }
    }

    /// Removes a child at the given index, shifting
    /// the remaining children to the left.
    ///
    /// Note: it doesn't modify keys and values or length.
    pub unsafe fn remove_child_at(self, index: usize) {
        unsafe {
            let len = self.len();
            debug_check_or_release_hint!(index <= len, "remove_child_at: index out of node bounds");
            if index < len {
                self.children_ptr()
                    .add(index)
                    .copy_from(self.children_ptr().add(index + 1), len - index - 1);
            } else if cfg!(debug_assertions) {
                self.children_ptr().add(index).write_bytes(0, 1);
            }
        }
    }

    #[inline(always)]
    const fn new_unchecked(ptr: *mut u8) -> Self {
        const {
            // Note: layouts of leafs and internal nodes are almost the same. The difference
            // is that internal node has multiple children at the end, while leaf node always
            // has one child (which is null, indicating that this is a leaf node).
            //
            // The checks below ensure that.
            assert!(
                Self::INTERNAL_NODE_LAYOUT.keys_offset == Self::LEAF_NODE_LAYOUT.keys_offset,
                "INTERNAL_NODE_LAYOUT.keys_offset != LEAF_NODE_LAYOUT.keys_offset"
            );

            assert!(
                Self::INTERNAL_NODE_LAYOUT.children_offset == Self::LEAF_NODE_LAYOUT.children_offset,
                "INTERNAL_NODE_LAYOUT.children_offset != LEAF_NODE_LAYOUT.children_offset"
            );

            assert!(
                Self::INTERNAL_NODE_LAYOUT.len_offset == Self::LEAF_NODE_LAYOUT.len_offset,
                "INTERNAL_NODE_LAYOUT.key_values_len_offset != LEAF_NODE_LAYOUT.key_values_len_offset"
            );

            assert!(
                Self::INTERNAL_NODE_LAYOUT.values_offset == Self::LEAF_NODE_LAYOUT.values_offset,
                "INTERNAL_NODE_LAYOUT.values_offset != LEAF_NODE_LAYOUT.values_offset"
            );

            assert!(
                Self::INTERNAL_NODE_LAYOUT.parent_offset == Self::LEAF_NODE_LAYOUT.parent_offset,
                "INTERNAL_NODE_LAYOUT.parent_offset != LEAF_NODE_LAYOUT.parent_offset"
            );
        }

        Self {
            ptr,
            _phantom1: PhantomData,
            _phantom2: PhantomData,
        }
    }
}
