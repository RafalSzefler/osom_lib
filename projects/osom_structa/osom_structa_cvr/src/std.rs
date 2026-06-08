//! This module defines the std aliases.
use osom_lib_alloc::std_allocator::StdAllocator;

use crate::cvr::{CVR, CVRArray, CVRObject, CVRString};

/// An alias for [`CVR`] with [`StdAllocator`]. Requires `std` feature.
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub type StdCVR = CVR<StdAllocator>;

/// An alias for [`CVRArray`] with [`StdAllocator`]. Requires `std` feature.
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub type StdCVRArray = CVRArray<StdAllocator>;

/// An alias for [`CVRString`] with [`StdAllocator`]. Requires `std` feature.
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub type StdCVRString = CVRString<StdAllocator>;

/// An alias for [`CVRObject`] with [`StdAllocator`]. Requires `std` feature.
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub type StdCVRObject = CVRObject<StdAllocator>;

cfg_select! {
    feature="serde" => {
        use crate::cvr::serde::CVRDeserializeContext;

        /// An alias for [`CVRDeserializeContext`] with [`StdAllocator`]. Requires `serde` feature.
        #[cfg_attr(docsrs, doc(cfg(all(feature = "serde", feature = "std"))))]
        pub type StdCVRDeserializeContext = CVRDeserializeContext<StdAllocator>;
    },
    _ => {
    }
}
