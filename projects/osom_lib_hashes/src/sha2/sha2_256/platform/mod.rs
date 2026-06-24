//! Holds the specialized, per arch implementations of `SHA2_256`

mod sha2_256_template;

mod sha2_256_x86;
pub use sha2_256_x86::*;

mod sha2_256_aarch64;
pub use sha2_256_aarch64::*;

// Ensure that all `SHA2_256` structs are of the same size.
const _: () = const {
    use crate::sha2::sha2_256::portable::SHA2_256_Portable;

    macro_rules! assert_sha_size {
        ( $id: ident ) => {
            assert!(
                size_of::<$id>() == size_of::<SHA2_256_Portable>(),
                concat!(stringify!($id), " has unexpected size.")
            );
        };
    }

    assert_sha_size!(SHA2_256_Portable);
    assert_sha_size!(SHA2_256_x86);
    assert_sha_size!(SHA2_256_aarch64);
};
