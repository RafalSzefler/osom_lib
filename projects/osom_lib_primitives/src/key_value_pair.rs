#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyValuePair<TKey, TValue> {
    key: TKey,
    value: TValue,
}

impl<TKey, TValue> KeyValuePair<TKey, TValue> {
    #[inline(always)]
    pub const fn new(key: TKey, value: TValue) -> Self {
        Self { key, value }
    }

    #[inline(always)]
    pub const fn from_tuple(tuple: (TKey, TValue)) -> Self {
        let key = unsafe { core::ptr::read(&tuple.0) };
        let value = unsafe { core::ptr::read(&tuple.1) };
        let result = Self::new(key, value);
        core::mem::forget(tuple);
        result
    }

    #[inline(always)]
    pub const fn key(&self) -> &TKey {
        &self.key
    }

    #[inline(always)]
    pub const fn key_mut(&mut self) -> &mut TKey {
        &mut self.key
    }

    #[inline(always)]
    pub const fn value(&self) -> &TValue {
        &self.value
    }

    #[inline(always)]
    pub const fn value_mut(&mut self) -> &mut TValue {
        &mut self.value
    }

    #[inline(always)]
    pub const fn into_tuple(self) -> (TKey, TValue) {
        let key = unsafe { core::ptr::read(&self.key) };
        let value = unsafe { core::ptr::read(&self.value) };
        core::mem::forget(self);
        (key, value)
    }
}

impl<TKey, TValue> From<(TKey, TValue)> for KeyValuePair<TKey, TValue> {
    fn from(tuple: (TKey, TValue)) -> Self {
        Self::from_tuple(tuple)
    }
}

impl<TKey, TValue> From<KeyValuePair<TKey, TValue>> for (TKey, TValue) {
    fn from(pair: KeyValuePair<TKey, TValue>) -> Self {
        pair.into_tuple()
    }
}
