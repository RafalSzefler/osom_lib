/// A wrapper around any value that does not have to be used.
///
/// The purpose of this struct is to override `#[must_use]` attribute
/// set on `T`.
#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
#[repr(transparent)]
pub struct DoesNotHaveToBeUsed<T> {
    pub value: T,
}

impl<T> core::ops::Deref for DoesNotHaveToBeUsed<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> core::ops::DerefMut for DoesNotHaveToBeUsed<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T> AsRef<T> for DoesNotHaveToBeUsed<T> {
    fn as_ref(&self) -> &T {
        &self.value
    }
}

impl<T> AsMut<T> for DoesNotHaveToBeUsed<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<T> core::borrow::Borrow<T> for DoesNotHaveToBeUsed<T> {
    fn borrow(&self) -> &T {
        &self.value
    }
}

impl<T> From<T> for DoesNotHaveToBeUsed<T> {
    fn from(value: T) -> Self {
        Self { value }
    }
}

unsafe impl<T: Send> Send for DoesNotHaveToBeUsed<T> {}
unsafe impl<T: Sync> Sync for DoesNotHaveToBeUsed<T> {}
