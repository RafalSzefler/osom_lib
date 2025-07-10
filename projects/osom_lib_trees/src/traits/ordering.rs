/// The ordering of the result of query on a tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Ordering {
    /// The ordering is unspecified. It is up to the implementation to decide
    /// the ordering, or whether there is no ordering at all.
    Unspecified,

    /// The ordering is ascending.
    Ascending,

    /// The ordering is descending.
    Descending,
}
