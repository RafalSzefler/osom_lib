use core::hash::{BuildHasher, Hasher};

const FNV_PRIME: u64 = 0x00000100000001B3;
const FNV_OFFSET_BASIS: u64 = 0xCBF29CE484222325;

#[must_use]
#[repr(transparent)]
pub struct Fnv1aHasher {
    state: u64,
}

impl Fnv1aHasher {
    #[inline(always)]
    pub const fn new(initial_state: u64) -> Self {
        Self { state: initial_state }
    }

    pub const fn update(&mut self, bytes: &[u8]) {
        let mut state = self.state;
        let start = bytes.as_ptr();
        let end = bytes.len();
        let mut idx = 0;
        while idx < end {
            let byte = unsafe { *start.add(idx) } as u64;
            state ^= byte;
            state = state.wrapping_mul(FNV_PRIME);
            idx += 1;
        }
        self.state = state;
    }

    #[inline(always)]
    pub const fn current_state(&self) -> u64 {
        self.state
    }
}

impl Default for Fnv1aHasher {
    #[inline(always)]
    fn default() -> Self {
        Self::new(FNV_OFFSET_BASIS)
    }
}

impl Hasher for Fnv1aHasher {
    fn finish(&self) -> u64 {
        self.current_state()
    }

    fn write(&mut self, bytes: &[u8]) {
        self.update(bytes);
    }
}

pub struct Fnv1aHasherBuilder;

impl BuildHasher for Fnv1aHasherBuilder {
    type Hasher = Fnv1aHasher;

    fn build_hasher(&self) -> Self::Hasher {
        Fnv1aHasher::default()
    }
}

impl Default for Fnv1aHasherBuilder {
    fn default() -> Self {
        Self
    }
}
