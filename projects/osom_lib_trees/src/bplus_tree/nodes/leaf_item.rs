#![allow(clippy::cast_sign_loss)]
use crate::bplus_tree::nodes::LeafNode;

pub struct LeafItem<const N: usize, TKey, TValue> {
    pub node: *mut LeafNode<N, TKey, TValue>,
    pub index: i32,
}

impl<const N: usize, TKey, TValue> LeafItem<N, TKey, TValue> {
    #[inline(always)]
    pub const fn null() -> Self {
        Self {
            node: core::ptr::null_mut(),
            index: -1,
        }
    }

    #[inline(always)]
    pub const unsafe fn key(&self) -> &TKey {
        let leaf = unsafe { &*self.node };
        &leaf.data().keys().as_slice()[self.index as usize]
    }

    #[inline(always)]
    pub const unsafe fn key_mut(&mut self) -> &mut TKey {
        let leaf = unsafe { &mut *self.node };
        &mut leaf.data_mut().keys_mut().as_mut_slice()[self.index as usize]
    }

    #[inline(always)]
    pub const unsafe fn key_ptr(&self) -> *mut TKey {
        let leaf = unsafe { &mut *self.node };
        unsafe {
            leaf.data_mut()
                .keys_mut()
                .as_mut_slice()
                .as_mut_ptr()
                .add(self.index as usize)
        }
    }

    #[inline(always)]
    pub const unsafe fn value(&self) -> &TValue {
        let leaf = unsafe { &*self.node };
        &leaf.values().as_slice()[self.index as usize]
    }

    #[inline(always)]
    pub const unsafe fn value_mut(&mut self) -> &mut TValue {
        let leaf = unsafe { &mut *self.node };
        &mut leaf.values_mut().as_mut_slice()[self.index as usize]
    }

    #[inline(always)]
    pub const unsafe fn value_ptr(&self) -> *mut TValue {
        let leaf = unsafe { &mut *self.node };
        unsafe { leaf.values_mut().as_mut_slice().as_mut_ptr().add(self.index as usize) }
    }

    #[inline(always)]
    pub const fn is_null(&self) -> bool {
        self.node.is_null()
    }

    pub const fn next(&self) -> Self {
        let leaf = unsafe { &*self.node };
        let data_len = leaf.data().keys().len().value();
        if self.index == data_len - 1 {
            Self {
                node: leaf.get_next(),
                index: 0,
            }
        } else {
            Self {
                node: self.node,
                index: self.index + 1,
            }
        }
    }

    pub const fn prev(&self) -> Self {
        let leaf = unsafe { &*self.node };
        if self.index == 0 {
            let prev = leaf.get_prev();
            let index = if prev.is_null() {
                -1
            } else {
                (unsafe { &*prev }).data().keys().len().value() - 1
            };
            Self { node: prev, index }
        } else {
            Self {
                node: self.node,
                index: self.index - 1,
            }
        }
    }

    #[inline(always)]
    pub fn is_equal(&self, other: &Self) -> bool {
        self.node == other.node && self.index == other.index
    }

    pub const fn clone(&self) -> Self {
        Self {
            node: self.node,
            index: self.index,
        }
    }
}

pub struct LeafItemRange<const N: usize, TKey, TValue> {
    pub start: LeafItem<N, TKey, TValue>,
    pub end: LeafItem<N, TKey, TValue>,
}

impl<const N: usize, TKey, TValue> LeafItemRange<N, TKey, TValue> {
    #[inline(always)]
    pub const fn null() -> Self {
        Self {
            start: LeafItem::null(),
            end: LeafItem::null(),
        }
    }

    #[inline(always)]
    pub const fn is_null(&self) -> bool {
        self.start.is_null()
    }
}
