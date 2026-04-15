use core::{
    alloc::Layout,
    mem::{ManuallyDrop, MaybeUninit},
    ops::DerefMut,
    ptr::NonNull,
};

use osom_lib_alloc::traits::Allocator;
use osom_lib_primitives::length::Length;
use osom_lib_reprc::traits::ReprC;

use crate::errors::ArrayError;

/// Represents a dynamic array, where first `TCAPACITY` items are inlined in the struct itself.
///
/// In other words, this array allocates memory only when its length exceeds `TCAPACITY`. In which
/// case it allocates data on heap, and becomes pretty much a [`DynamicArray`][crate::dynamic_array::DynamicArray].
#[repr(C)]
#[must_use]
pub struct InlineDynamicArray<const TCAPACITY: usize, T, TAllocator>
where
    T: Sized,
    TAllocator: Allocator,
{
    pub(super) internal: InlineArrayUnion<TCAPACITY, T>,
    pub(super) size: Length,
    pub(super) capacity: Length,
    pub(super) allocator: TAllocator,
}

unsafe impl<const TCAPACITY: usize, T, TAllocator> ReprC for InlineDynamicArray<TCAPACITY, T, TAllocator>
where
    T: Sized + ReprC,
    TAllocator: Allocator,
{
    const CHECK: () = const {
        let () = <T as ReprC>::CHECK;
        let () = <TAllocator as ReprC>::CHECK;
        let () = <Length as ReprC>::CHECK;
        let () = <InlineArrayUnion<TCAPACITY, T> as ReprC>::CHECK;
    };
}

#[repr(C)]
pub(super) union InlineArrayUnion<const TCAPACITY: usize, T>
where
    T: Sized,
{
    pub inlined: ManuallyDrop<MaybeUninit<[T; TCAPACITY]>>,
    pub ptr: *mut T,
}

unsafe impl<const TCAPACITY: usize, T> ReprC for InlineArrayUnion<TCAPACITY, T>
where
    T: ReprC,
{
    const CHECK: () = const {
        let () = <T as ReprC>::CHECK;
        let () = <*mut T as ReprC>::CHECK;
        let () = <ManuallyDrop<MaybeUninit<[T; TCAPACITY]>> as ReprC>::CHECK;
    };
}

impl<const TCAPACITY: usize, T, TAllocator> InlineDynamicArray<TCAPACITY, T, TAllocator>
where
    T: Sized,
    TAllocator: Allocator,
{
    /// Creates a new empty [`InlineDynamicArray`] with the default `TAllocator`.
    #[inline(always)]
    pub fn new() -> Self {
        Self::with_allocator(TAllocator::default())
    }

    /// Creates a new empty [`InlineDynamicArray`] with the given `TAllocator`.
    #[inline]
    pub const fn with_allocator(allocator: TAllocator) -> Self {
        let inlined = ManuallyDrop::new(MaybeUninit::uninit());
        Self {
            internal: InlineArrayUnion { inlined },
            size: Length::ZERO,
            capacity: static_capacity::<TCAPACITY>(),
            allocator,
        }
    }

    /// Creates a new [`InlineDynamicArray`] with the default `TAllocator`. This method allocates memory
    /// if `capacity` exceeds `TCAPACITY`.
    ///
    /// # Errors
    ///
    /// For details see [`ArrayError`].
    #[inline(always)]
    pub fn with_capacity(capacity: Length) -> Result<Self, ArrayError> {
        Self::with_capacity_and_allocator(capacity, TAllocator::default())
    }

    /// Creates a new [`InlineDynamicArray`] with the given `TAllocator`. This method allocates memory
    /// if `capacity` exceeds `TCAPACITY`.
    ///
    /// # Errors
    ///
    /// For details see [`ArrayError`].
    pub fn with_capacity_and_allocator(capacity: Length, allocator: TAllocator) -> Result<Self, ArrayError> {
        if capacity.as_usize() <= TCAPACITY {
            return Ok(Self::with_allocator(allocator));
        }

        if capacity.as_usize() > Length::MAX_LENGTH.as_usize() {
            return Err(ArrayError::LengthLimitExceeded);
        }

        let new_ptr = allocator
            .allocate(Self::layout_for_size(capacity))
            .map_err(Into::into)?
            .cast::<T>();

        Ok(Self {
            internal: InlineArrayUnion { ptr: new_ptr.as_ptr() },
            size: Length::ZERO,
            capacity: capacity,
            allocator: allocator,
        })
    }

    pub(super) const fn layout_for_size(size: Length) -> Layout {
        let tsize = size_of::<T>();
        let Some(real_size) = tsize.checked_mul(size.as_usize()) else {
            panic!("Tried to allocate array of size outside of usize range");
        };
        unsafe { Layout::from_size_align_unchecked(real_size, align_of::<T>()) }
    }

    #[inline(always)]
    pub(super) const fn current_layout(&self) -> Layout {
        Self::layout_for_size(self.capacity)
    }

    #[inline(always)]
    pub(super) fn is_inlined(&self) -> bool {
        self.capacity.as_usize() <= TCAPACITY
    }

    pub(super) fn current_ptr_mut(&mut self) -> *mut T {
        unsafe {
            if self.is_inlined() {
                self.internal.inlined.deref_mut().as_mut_ptr().cast::<T>()
            } else {
                self.internal.ptr
            }
        }
    }

    pub(super) fn as_slice_internal(&self) -> &[T] {
        unsafe {
            let ptr = if self.is_inlined() {
                self.internal.inlined.as_ptr().cast::<T>()
            } else {
                self.internal.ptr
            };

            core::slice::from_raw_parts(ptr, self.size.as_usize())
        }
    }

    pub(super) fn as_slice_mut_internal(&mut self) -> &mut [T] {
        unsafe { core::slice::from_raw_parts_mut(self.current_ptr_mut(), self.size.as_usize()) }
    }

    #[allow(clippy::cast_possible_truncation)]
    pub(super) fn reserve_if_needed(&mut self, new_length: u32) -> Result<(), ArrayError> {
        let capacity = self.capacity.as_u32();
        if new_length <= capacity {
            return Ok(());
        }

        let new_capacity = {
            let upper_bound = ((u64::from(new_length) * 3) / 2) + 1;
            let capped = core::cmp::min(upper_bound, Length::MAX_LENGTH.as_usize() as u64) as u32;
            unsafe { Length::new_unchecked(capped) }
        };

        let new_ptr = unsafe {
            let new_layout = Self::layout_for_size(new_capacity);
            if self.is_inlined() {
                let ptr = self
                    .allocator
                    .allocate(new_layout)
                    .map_err(Into::into)?
                    .cast::<T>()
                    .as_ptr();
                let current_length = self.size.as_usize();
                if current_length > 0 {
                    let inlined_ptr = self.internal.inlined.deref_mut().as_mut_ptr().cast::<T>();
                    ptr.copy_from_nonoverlapping(inlined_ptr, current_length);
                }
                ptr
            } else {
                let old_layout = self.current_layout();
                let raw_ptr = NonNull::new_unchecked(self.internal.ptr);
                self.allocator
                    .resize(raw_ptr.cast(), old_layout, new_layout)
                    .map_err(Into::into)?
                    .cast()
                    .as_ptr()
            }
        };

        self.internal = InlineArrayUnion { ptr: new_ptr };
        self.capacity = new_capacity;
        Ok(())
    }
}

impl<const TCAPACITY: usize, T, TAllocator> Drop for InlineDynamicArray<TCAPACITY, T, TAllocator>
where
    T: Sized,
    TAllocator: Allocator,
{
    fn drop(&mut self) {
        unsafe {
            if core::mem::needs_drop::<T>() {
                for item in self.as_slice_mut_internal() {
                    core::ptr::drop_in_place(item);
                }
            }

            if self.is_inlined() {
                return;
            }

            let ptr = NonNull::new_unchecked(self.internal.ptr);
            self.allocator.deallocate(ptr.cast(), self.current_layout());
        }
    }
}

#[inline(always)]
#[allow(clippy::cast_possible_truncation)]
pub(super) const fn static_capacity<const TCAPACITY: usize>() -> Length {
    const {
        assert!(
            TCAPACITY > 0,
            "TCAPACITY cannot be zero. It reduces the InlineArray into a less efficient DynamicArray. Use latter instead."
        );
        assert!(
            TCAPACITY <= Length::MAX_LENGTH.as_usize(),
            "TCAPACITY cannot exceed Length::MAX_LENGTH"
        );
    }

    unsafe { Length::new_unchecked(TCAPACITY as u32) }
}
