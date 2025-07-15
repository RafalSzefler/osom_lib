#![allow(dead_code)]
#![allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap, clippy::cast_sign_loss)]

use core::ptr::NonNull;
use core::{alloc::Layout, marker::PhantomData, mem::size_of};

use osom_lib_alloc::{AllocationError, Allocator};
use osom_lib_primitives::Length;

pub(crate) type AtomicCounter = core::sync::atomic::AtomicU32;
pub(crate) type StrongCounter = AtomicCounter;
pub(crate) type WeakCounter = AtomicCounter;

const MAX_HEAP_DATA_HEADER_SIZE: usize = 1024;

/// Represents compile time constants necessary for calculating the layout of the [`HeapData`] struct.
struct HeapDataLayout {
    pub strong_counter_offset: Length,
    pub weak_counter_offset: Length,
    pub data_offset: Length,
    pub alignment: Length,
}

/// Represents what we actually store on the heap.
///
/// This innocent looking struct is actually heavily unsafe,
/// because the actual `[u8]` data follows it in memory.
pub(crate) struct HeapData<T: Sized> {
    strong_counter: StrongCounter,
    weak_counter: WeakCounter,
    phantom: PhantomData<T>,
    // data: [T]  follows this struct in memory. This is not a field, because
    // of the difficulties of working with un-sized structs in Rust.
}

impl<T: Sized> HeapData<T> {
    const LAYOUT: &HeapDataLayout = const {
        macro_rules! unwrap {
            ($result:expr) => {
                match $result {
                    Ok((layout, offset)) => {
                        assert!(offset <= i32::MAX as usize, "Offset is too large for HeapData.");
                        (layout, offset as i32)
                    }
                    Err(_) => panic!("Couldn't extend layout for HeapData."),
                }
            };
        }
        let heap_data_layout = Layout::new::<StrongCounter>();
        let (heap_data_layout, weak_offset) = unwrap!(heap_data_layout.extend(Layout::new::<WeakCounter>()));
        let (heap_data_layout, data_offset) = unwrap!(heap_data_layout.extend(Layout::new::<T>()));

        let heap_data_alignment = heap_data_layout.align();
        assert!(
            heap_data_alignment > 0,
            "Alignment is 0 for HeapData? Something went seriously wrong."
        );
        assert!(
            heap_data_alignment <= MAX_HEAP_DATA_HEADER_SIZE,
            "Alignment is too large for HeapData."
        );
        assert!(
            data_offset as usize <= MAX_HEAP_DATA_HEADER_SIZE,
            "Data offset is too large for HeapData."
        );
        assert!(heap_data_alignment > 0, "Alignment is zero.");
        assert!(
            heap_data_alignment.is_power_of_two(),
            "Alignment is not a power of two for HeapData."
        );
        assert!(
            heap_data_alignment >= align_of::<T>(),
            "Alignment is less than the type's alignment."
        );
        assert!(
            data_offset as usize % heap_data_alignment == 0,
            "Data offset is not aligned to the alignment."
        );

        &HeapDataLayout {
            strong_counter_offset: Length::ZERO,
            weak_counter_offset: unsafe { Length::new_unchecked(weak_offset) },
            data_offset: unsafe { Length::new_unchecked(data_offset) },
            alignment: unsafe { Length::new_unchecked(heap_data_alignment as i32) },
        }
    };

    /// Returns the layout of the [`HeapData`] struct for a given data length.
    #[inline(always)]
    pub const fn layout(data_length: Length) -> Layout {
        let data_offset = Self::LAYOUT.data_offset.value() as usize;
        let binary_data_length = data_length.value() as usize * size_of::<T>();
        let total_size = data_offset + binary_data_length;
        unsafe { Layout::from_size_align_unchecked(total_size, Self::LAYOUT.alignment.value() as usize) }
    }

    /// Returns a reference to the strong counter.
    #[inline(always)]
    pub const fn strong_counter(&self) -> &StrongCounter {
        &self.strong_counter
    }

    /// Returns a mutable reference to the strong counter.
    #[inline(always)]
    pub const fn strong_counter_mut(&mut self) -> &mut StrongCounter {
        &mut self.strong_counter
    }

    /// Returns a reference to the weak counter.
    #[inline(always)]
    pub const fn weak_counter(&self) -> &WeakCounter {
        &self.weak_counter
    }

    /// Returns a mutable reference to the weak counter.
    #[inline(always)]
    pub const fn weak_counter_mut(&mut self) -> &mut WeakCounter {
        &mut self.weak_counter
    }

    /// Returns a pointer to the actual `[T]` data that follows this struct in memory.
    #[inline(always)]
    pub const fn data(&self) -> NonNull<T> {
        unsafe {
            let self_ptr = core::ptr::from_ref(self).cast_mut().cast::<u8>();
            let data_ptr = self_ptr.add(Self::LAYOUT.data_offset.value() as usize).cast::<T>();
            NonNull::new_unchecked(data_ptr)
        }
    }
}

/// The maximum length of an array that can be constructed.
/// We reserve some space for future usage. Regardless, one should
/// not generate 2Gb arrays in memory anyway.
pub const MAX_LENGTH: usize = const { (i32::MAX as usize) - MAX_HEAP_DATA_HEADER_SIZE };

/// The internal representation of an [`ImmutableArray`][`super::ImmutableArray`].
#[repr(C)]
pub(crate) struct InternalArray<T: Sized, TAllocator: Allocator> {
    /// The pointer to the [`HeapData`] struct that holds atomic counters and the actual `[u8]` data.
    data: NonNull<u8>,

    /// The length of the array.
    length: Length,

    /// The capacity of the array. This field will be used by dynamic array builders.
    capacity: Length,

    /// The allocator used to allocate the [`HeapData`] struct.
    allocator: TAllocator,

    phantom: PhantomData<T>,
}

unsafe impl<T: Send + Sync, TAllocator: Allocator> Send for InternalArray<T, TAllocator> {}
unsafe impl<T: Send + Sync, TAllocator: Allocator> Sync for InternalArray<T, TAllocator> {}

impl<T: Sized, TAllocator: Allocator> InternalArray<T, TAllocator> {
    pub fn allocate(length: Length, capacity: Length, allocator: TAllocator) -> Result<Self, AllocationError> {
        let heap_data_layout = HeapData::<T>::layout(capacity);
        let new_memory = allocator.allocate(heap_data_layout)?;
        unsafe {
            // We fill only the initial `HeapData` segment, because the remaining
            // memory will likely be overwritten anyway.
            new_memory.as_ptr().write_bytes(0, size_of::<HeapData<T>>());
        }

        Ok(Self {
            data: new_memory,
            length,
            capacity: capacity,
            allocator: allocator,
            phantom: PhantomData,
        })
    }

    pub fn grow(&mut self, new_capacity: Length) -> Result<(), AllocationError> {
        assert!(
            new_capacity >= self.capacity,
            "New capacity is less than the current capacity."
        );
        let old_heap_data_layout = HeapData::<T>::layout(self.capacity);
        let new_heap_data_layout = HeapData::<T>::layout(new_capacity);
        self.data = unsafe {
            self.allocator
                .resize(self.data, old_heap_data_layout, new_heap_data_layout)?
        };
        self.capacity = new_capacity;
        Ok(())
    }

    #[inline(always)]
    pub fn deallocate(self) {
        let heap_data_layout = HeapData::<T>::layout(self.capacity);
        unsafe { self.allocator.deallocate(self.data, heap_data_layout) };
    }

    #[inline(always)]
    pub const fn allocator(&self) -> &TAllocator {
        &self.allocator
    }

    #[inline(always)]
    pub fn heap_data(&self) -> &HeapData<T> {
        unsafe { &*self.data.as_ptr().cast() }
    }

    #[inline(always)]
    pub fn heap_data_mut(&mut self) -> &mut HeapData<T> {
        unsafe { &mut *self.data.as_ptr().cast() }
    }

    #[inline(always)]
    pub const fn len(&self) -> Length {
        self.length
    }

    #[inline(always)]
    pub const fn len_mut(&mut self) -> &mut Length {
        &mut self.length
    }

    #[inline(always)]
    pub const fn capacity(&self) -> Length {
        self.capacity
    }

    #[inline(always)]
    pub fn as_slice(&self) -> &[T] {
        unsafe {
            let data_ptr = self.heap_data().data().as_ptr().cast::<T>();
            core::slice::from_raw_parts(data_ptr, self.length.value() as usize)
        }
    }

    #[inline(always)]
    pub fn as_slice_mut(&mut self) -> &mut [T] {
        unsafe {
            let data_ptr = self.heap_data().data().as_ptr().cast::<T>();
            core::slice::from_raw_parts_mut(data_ptr, self.length.value() as usize)
        }
    }
}

impl<T: Sized, TAllocator: Allocator> Clone for InternalArray<T, TAllocator> {
    fn clone(&self) -> Self {
        Self {
            data: self.data,
            length: self.length,
            capacity: self.capacity,
            allocator: self.allocator.clone(),
            phantom: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout_u8() {
        let layout = HeapData::<u8>::LAYOUT;
        unsafe {
            assert_eq!(layout.strong_counter_offset, Length::new_unchecked(0));
            assert_eq!(layout.weak_counter_offset, Length::new_unchecked(4));
            assert_eq!(layout.data_offset, Length::new_unchecked(8));
            assert_eq!(layout.alignment, Length::new_unchecked(4));
            let data_layout = HeapData::<u8>::layout(Length::new_unchecked(17));
            assert_eq!(data_layout.size(), 8 + 17);
            assert_eq!(data_layout.align(), 4);
        }
    }

    #[test]
    fn test_layout_struct() {
        struct CustomStruct {
            data: i32,
            second: &'static str,
        }
        let layout = HeapData::<CustomStruct>::LAYOUT;
        unsafe {
            assert_eq!(layout.strong_counter_offset, Length::new_unchecked(0));
            assert_eq!(layout.weak_counter_offset, Length::new_unchecked(4));
            assert_eq!(layout.data_offset, Length::new_unchecked(8));
            assert_eq!(layout.alignment, Length::new_unchecked(8));
            let data_layout = HeapData::<CustomStruct>::layout(Length::new_unchecked(3));
            assert_eq!(data_layout.size(), 8 + 3 * size_of::<CustomStruct>());
            assert_eq!(data_layout.align(), 8);
        }
    }
    #[test]
    fn test_layout_custom_align() {
        #[repr(align(16))]
        struct CustomStruct;

        let layout = HeapData::<CustomStruct>::LAYOUT;
        unsafe {
            assert_eq!(layout.strong_counter_offset, Length::new_unchecked(0));
            assert_eq!(layout.weak_counter_offset, Length::new_unchecked(4));
            assert_eq!(layout.data_offset, Length::new_unchecked(16));
            assert_eq!(layout.alignment, Length::new_unchecked(16));
            let data_layout = HeapData::<CustomStruct>::layout(Length::new_unchecked(25));
            assert_eq!(data_layout.size(), 16);
            assert_eq!(data_layout.align(), 16);
        }
    }
}
