#![allow(
    clippy::needless_borrow,
    clippy::uninit_assumed_init,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss
)]

use core::alloc::Layout;
use core::mem::ManuallyDrop;
use core::ops::Deref;

use osom_lib_alloc::{AllocatedMemory, AllocationError, Allocator};
use osom_lib_primitives::Length;

#[cfg(feature = "std_alloc")]
use osom_lib_alloc::StdAllocator;

/// Represents an error that occurs when constructing new [`InlineDynamicArray`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[must_use]
#[repr(u8)]
pub enum InlineDynamicArrayConstructionError {
    /// The allocator failed to allocate memory.
    AllocationError,

    /// The passed array is too long, it exceeds [`MAX_SIZE`][`InlineDynamicArray::MAX_SIZE`].
    ArrayTooLong,
}

impl From<AllocationError> for InlineDynamicArrayConstructionError {
    fn from(_: AllocationError) -> Self {
        InlineDynamicArrayConstructionError::AllocationError
    }
}
union InlineDynamicArrayUnion<T, const N: usize> {
    stack_data: ManuallyDrop<[T; N]>,
    heap_data: *mut T,
}

/// A struct similar to [`DynamicArray`][`super::DynamicArray`],
/// but holds `N` items inlined. Meaning it won't allocate memory
/// until it exceeds `N` elements.
///
/// Note that this struct can only grow, and will never shrink.
#[must_use]
pub struct InlineDynamicArray<
    const N: usize,
    T,
    #[cfg(feature = "std_alloc")] TAllocator = StdAllocator,
    #[cfg(not(feature = "std_alloc"))] TAllocator,
> where
    TAllocator: Allocator,
{
    data: InlineDynamicArrayUnion<T, N>,
    length: Length,
    capacity: Length,
    allocator: TAllocator,
}

impl<const N: usize, T, TAllocator: Allocator> InlineDynamicArray<N, T, TAllocator> {
    pub const MAX_SIZE: usize = Length::MAX;

    const fn validate() {
        assert!(
            N > 0,
            "N in InlineVec must be greater than 0. InlineVec with N == 0 is just Vec. Use Vec instead."
        );

        // Note: 2147482623 is (i32::MAX - 1024). This is definitely way too much,
        // but we reserve some space, just in case.
        assert!(
            N < Self::MAX_SIZE,
            "N in InlineVec must be at most 2147482623. Which likely already is waaaay too much."
        );
    }

    #[inline(always)]
    const fn layout(size: usize) -> Layout {
        let real_size = size * size_of::<T>();
        let alignment = align_of::<T>();
        unsafe { Layout::from_size_align_unchecked(real_size, alignment) }
    }

    #[inline(always)]
    fn allocate_memory(&self, size: usize) -> Result<*mut T, AllocationError> {
        let new_memory = self.allocator.allocate(Self::layout(size))?;
        let result: *mut T = unsafe { new_memory.as_ptr() };
        assert!(result.is_aligned(), "Newly allocated memory is not aligned correctly.");
        Ok(result)
    }

    #[inline(always)]
    const fn is_inlined(&self) -> bool {
        self.capacity.value() == N as i32
    }

    #[inline(always)]
    fn data_ptr(&self) -> *mut T {
        unsafe {
            if self.is_inlined() {
                (&self.data.stack_data).as_ptr().cast_mut()
            } else {
                self.data.heap_data
            }
        }
    }

    /// Pushes a value to the end of the [`InlineDynamicArray`].
    ///
    /// Note that the [`InlineDynamicArray`] data will be moved to the heap
    /// only when length exceeds `N`. It won't come back from the
    /// heap though.
    ///
    /// Also note that the [`InlineDynamicArray`] will never shrink, it will
    /// only keep growing.
    ///
    /// # Errors
    ///
    /// For details see [`InlineDynamicArrayConstructionError`].
    pub fn push(&mut self, value: T) -> Result<(), InlineDynamicArrayConstructionError> {
        let capacity: usize = self.capacity.into();
        unsafe {
            if self.is_inlined() {
                if self.length.value() < N as i32 {
                    let data = &mut self.data.stack_data;
                    data.as_mut_ptr().add(self.length.into()).write(value);
                    self.length += 1;
                    return Ok(());
                }
                let new_capacity = capacity * 2;
                if new_capacity > Self::MAX_SIZE {
                    return Err(InlineDynamicArrayConstructionError::ArrayTooLong);
                }
                let new_memory = self.allocate_memory(new_capacity)?;
                let stack_data = (&self.data.stack_data).as_ptr();
                new_memory.copy_from_nonoverlapping(stack_data, self.length.into());
                new_memory.add(self.length.into()).write(value);
                self.length += 1;
                self.data = InlineDynamicArrayUnion { heap_data: new_memory };
                self.capacity = Length::new_unchecked(new_capacity as i32);
                return Ok(());
            }

            if self.length == self.capacity {
                let new_capacity = capacity * 2;
                if new_capacity > Self::MAX_SIZE {
                    return Err(InlineDynamicArrayConstructionError::ArrayTooLong);
                }
                let old_layout = Self::layout(self.capacity.into());
                let new_layout = Self::layout(new_capacity);
                let old_memory = self.allocator.convert_raw_ptr(self.data.heap_data);
                let new_memory = old_memory.resize(old_layout, new_layout)?;
                self.data.heap_data = new_memory.as_ptr();
                self.capacity = Length::new_unchecked(new_capacity as i32);
            }

            self.data.heap_data.add(self.length.into()).write(value);
            self.length += 1;
        }
        Ok(())
    }

    /// Pops last element from the [`InlineDynamicArray`],
    /// decreasing its size.
    ///
    /// # Returns
    ///
    /// * `Some(T)` if `self.len() > 0`
    /// * `None` otherwise
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            Some(unsafe { self.pop_unchecked() })
        }
    }

    /// Unsafe variant of [`pop`][`Self::pop`].
    ///
    /// # Safety
    ///
    /// Returns `T` if `self.len() > 0` and decrease the
    /// [`InlineDynamicArray`] size. The behaviour is undefined if
    /// `self.len() == 0`.
    #[inline]
    pub unsafe fn pop_unchecked(&mut self) -> T {
        debug_assert!(!self.is_empty(), "Tried pop_unchecked on length 0 InlineDynamicArray.");
        unsafe {
            let ptr = self.data_ptr();
            self.length -= 1;
            ptr.add(self.length.into()).read()
        }
    }

    /// Creates a new empty [`InlineDynamicArray`].
    #[inline(always)]
    pub fn new() -> Self {
        Self::with_allocator(TAllocator::default())
    }

    /// Creates a new empty [`InlineDynamicArray`] with the given allocator.
    #[inline]
    pub fn with_allocator(allocator: TAllocator) -> Self {
        const { Self::validate() };

        Self {
            data: InlineDynamicArrayUnion {
                heap_data: unsafe { allocator.dangling::<T>().as_ptr() },
            },
            length: Length::ZERO,
            capacity: unsafe { Length::new_unchecked(N as i32) },
            allocator: allocator,
        }
    }

    /// Returns the number of elements in the [`InlineDynamicArray`].
    #[inline(always)]
    pub const fn len(&self) -> Length {
        self.length
    }

    /// Returns `true` if the [`InlineDynamicArray`] is empty,
    /// otherwise `false`.
    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        self.length.value() == 0
    }

    /// Returns the capacity of the [`InlineDynamicArray`]. Note that
    /// this is always at least `N`.
    #[inline(always)]
    pub const fn capacity(&self) -> Length {
        self.capacity
    }

    /// Represents current [`InlineDynamicArray`] as a slice.
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        unsafe {
            let ptr = self.data_ptr();
            core::slice::from_raw_parts(ptr, self.len().into())
        }
    }
}

impl<const N: usize, T: Clone, TAllocator: Allocator> InlineDynamicArray<N, T, TAllocator> {
    /// Tries to clone the [`InlineDynamicArray`].
    ///
    /// # Errors
    ///
    /// For details see [`InlineDynamicArrayConstructionError`].
    pub fn try_clone(&self) -> Result<Self, InlineDynamicArrayConstructionError> {
        let mut new = Self::with_allocator(self.allocator.clone());
        let slice = self.as_slice();
        for item in slice {
            new.push(item.clone())?;
        }
        Ok(new)
    }
}

impl<const N: usize, T, TAllocator: Allocator> Drop for InlineDynamicArray<N, T, TAllocator> {
    fn drop(&mut self) {
        unsafe {
            if core::mem::needs_drop::<T>() {
                let mut ptr = self.data_ptr();
                let mut idx = 0;
                while idx < self.len().into() {
                    drop(ptr.read());
                    ptr = ptr.add(1);
                    idx += 1;
                }
            }

            if !self.is_inlined() {
                let layout = Self::layout(self.capacity.into());
                let memory = self.allocator.convert_raw_ptr(self.data.heap_data);
                memory.deallocate(layout);
            }
        }
    }
}

impl<const N: usize, T, TAllocator: Allocator> Default for InlineDynamicArray<N, T, TAllocator> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize, T: Clone, TAllocator: Allocator> Clone for InlineDynamicArray<N, T, TAllocator> {
    fn clone(&self) -> Self {
        self.try_clone().expect("Failed to clone the inline dynamic array")
    }
}

impl<const N: usize, T, TAllocator: Allocator> core::fmt::Debug for InlineDynamicArray<N, T, TAllocator> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("InlineDynamicArray")
            .field("N", &N)
            .field("len", &self.len())
            .field("capacity", &self.capacity())
            .field("is_inlined", &self.is_inlined())
            .field("raw_ptr", &self.data_ptr().addr())
            .finish()
    }
}

impl<const N1: usize, const N2: usize, T: PartialEq, TAllocator1: Allocator, TAllocator2: Allocator>
    PartialEq<InlineDynamicArray<N1, T, TAllocator1>> for InlineDynamicArray<N2, T, TAllocator2>
{
    fn eq(&self, other: &InlineDynamicArray<N1, T, TAllocator1>) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<const N: usize, T: Eq, TAllocator: Allocator> Eq for InlineDynamicArray<N, T, TAllocator> {}

impl<const N: usize, T, TAllocator: Allocator> Deref for InlineDynamicArray<N, T, TAllocator> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<const N: usize, T, TAllocator: Allocator> AsRef<[T]> for InlineDynamicArray<N, T, TAllocator> {
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

unsafe impl<const N: usize, T: Send, TAllocator: Allocator> Send for InlineDynamicArray<N, T, TAllocator> {}
unsafe impl<const N: usize, T: Sync, TAllocator: Allocator> Sync for InlineDynamicArray<N, T, TAllocator> {}
