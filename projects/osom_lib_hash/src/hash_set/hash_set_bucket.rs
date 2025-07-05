/// Represents a bucket kept inside [`HashSet`][super::HashSet].
#[derive(Debug, Default)]
#[repr(u8)]
pub enum Bucket<T> {
    #[default]
    Empty = 1,
    Deleted,
    Occupied {
        value: T,
        hash: usize,
    },
}

impl<T: PartialEq> PartialEq for Bucket<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Bucket::Empty, Bucket::Empty) => true,
            (Bucket::Deleted, Bucket::Deleted) => true,
            (
                Bucket::Occupied {
                    value: value1,
                    hash: hash1,
                },
                Bucket::Occupied {
                    value: value2,
                    hash: hash2,
                },
            ) => hash1 == hash2 && value1 == value2,
            _ => false,
        }
    }
}

impl<T: Eq> Eq for Bucket<T> {}

impl<T: Clone> Clone for Bucket<T> {
    fn clone(&self) -> Self {
        match self {
            Bucket::Empty => Bucket::Empty,
            Bucket::Deleted => Bucket::Deleted,
            Bucket::Occupied { value, hash } => Bucket::Occupied {
                value: value.clone(),
                hash: *hash,
            },
        }
    }
}

impl<T: core::hash::Hash> core::hash::Hash for Bucket<T> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Bucket::Empty => {
                1u8.hash(state);
                0u8.hash(state);
            }
            Bucket::Deleted => {
                2u8.hash(state);
                0u8.hash(state);
            }
            Bucket::Occupied { hash, .. } => {
                3u8.hash(state);
                hash.hash(state);
            }
        }
    }
}
