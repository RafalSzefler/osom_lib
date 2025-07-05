use core::sync::atomic::Ordering;

use osom_lib_alloc::Allocator;

#[cfg(feature = "std_alloc")]
use osom_lib_alloc::StdAllocator;

use super::ImmutableArray;
use super::internal_array::{HeapData, InternalArray};

/// A weak reference to an [`ImmutableArray`].
///
/// It doesn't provide direct access to the data itself,
/// but is useful for tracking whether the associated [`ImmutableArray`]
/// is still alive or not. Through [`ImmutableWeakArray::upgrade`] method.
#[repr(transparent)]
pub struct ImmutableWeakArray<
    T: Sized,
    #[cfg(feature = "std_alloc")] TAllocator = StdAllocator,
    #[cfg(not(feature = "std_alloc"))] TAllocator,
> where
    TAllocator: Allocator,
{
    internal: InternalArray<T, TAllocator>,
}

impl<T: Sized, TAllocator: Allocator> ImmutableWeakArray<T, TAllocator> {
    /// Upgrades the weak reference to a strong [`ImmutableArray`] reference.
    ///
    /// Returns `None` if the array has been deallocated. Otherwise, returns a strong reference.
    pub fn upgrade(&self) -> Option<ImmutableArray<T, TAllocator>> {
        let mut strong_counter = self.internal.heap_data().strong_counter().load(Ordering::SeqCst);
        if strong_counter == 0 {
            return None;
        }

        let new_strong_reference = ImmutableArray::from(self.internal.clone());

        loop {
            let result = self.internal.heap_data().strong_counter().compare_exchange_weak(
                strong_counter,
                strong_counter + 1,
                Ordering::SeqCst,
                Ordering::SeqCst,
            );
            match result {
                Ok(_) => return Some(new_strong_reference),
                Err(_) => {
                    strong_counter = self.internal.heap_data().strong_counter().load(Ordering::SeqCst);
                }
            }
        }
    }

    /// Returns the number of strong references to the string.
    #[must_use]
    pub fn strong_count(&self) -> usize {
        self.internal.heap_data().strong_counter().load(Ordering::SeqCst) as usize
    }

    /// Returns the number of weak references to the string.
    #[must_use]
    pub fn weak_count(&self) -> usize {
        self.internal.heap_data().weak_counter().load(Ordering::SeqCst) as usize
    }

    /// Returns a reference to the allocator of the [`ImmutableWeakArray`].
    #[inline(always)]
    pub const fn allocator(&self) -> &TAllocator {
        &self.internal.allocator()
    }

    /// Releases the weak reference.
    ///
    /// Returns `true` if it was the last weak reference and the memory was deallocated.
    /// Otherwise, returns `false`.
    #[inline(always)]
    pub fn release(mut self) -> bool {
        let result = self.internal_release();
        core::mem::forget(self);
        result
    }

    pub(crate) fn internal_release(&mut self) -> bool {
        let weak_counter = self.internal.heap_data().weak_counter().fetch_sub(1, Ordering::SeqCst);
        if weak_counter == 1 {
            if core::mem::needs_drop::<T>() {
                let slice = self.internal.as_slice();
                let mut start = slice.as_ptr();
                let end = unsafe { start.add(slice.len()) };
                while start < end {
                    unsafe {
                        core::mem::drop(start.read());
                        start = start.add(1);
                    }
                }
            }

            let internal = unsafe { core::ptr::read(&self.internal) };
            internal.deallocate();
            true
        } else {
            false
        }
    }
}

impl<T: Sized, TAllocator: Allocator> Clone for ImmutableWeakArray<T, TAllocator> {
    fn clone(&self) -> Self {
        self.internal.heap_data().weak_counter().fetch_add(1, Ordering::SeqCst);
        Self {
            internal: self.internal.clone(),
        }
    }
}

impl<T: Sized, TAllocator: Allocator> Drop for ImmutableWeakArray<T, TAllocator> {
    fn drop(&mut self) {
        self.internal_release();
    }
}

impl<T: Sized, TAllocator: Allocator> From<InternalArray<T, TAllocator>> for ImmutableWeakArray<T, TAllocator> {
    fn from(internal: InternalArray<T, TAllocator>) -> Self {
        Self { internal }
    }
}

impl<T: Sized, TAllocator: Allocator> core::fmt::Debug for ImmutableWeakArray<T, TAllocator> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let ptr = core::ptr::from_ref::<HeapData<T>>(self.internal.heap_data());
        f.debug_struct("ImmutableWeakArray")
            .field("strong_count", &self.strong_count())
            .field("weak_count", &self.weak_count())
            .field("len", &self.internal.len())
            .field("capacity", &self.internal.capacity())
            .field("raw_ptr", &ptr.addr())
            .finish()
    }
}
