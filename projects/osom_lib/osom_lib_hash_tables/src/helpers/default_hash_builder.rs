use core::{hash::BuildHasher, marker::PhantomData};

use osom_lib_hashes::siphash::GeneralSipHash;
use osom_lib_reprc::macros::reprc;

cfg_select! {
    all(feature = "std", not(miri)) => {
        use std::sync::LazyLock;
        use osom_lib_hashes::siphash::GeneralSipHashBuilder;
    },
    _ => {},
}

/// A default hash builder for osom hash tables. Utilizes sip hash 1-3
/// under the hood.
#[reprc]
#[repr(transparent)]
#[derive(Default, Clone, Copy)]
#[must_use]
pub struct DefaultHashBuilder {
    _priv: PhantomData<()>,
}

impl DefaultHashBuilder {
    #[inline(always)]
    pub const fn new() -> Self {
        Self { _priv: PhantomData }
    }
}

#[cfg(all(feature = "std", not(miri)))]
static SIP_HASH: LazyLock<GeneralSipHashBuilder<1, 3>> = LazyLock::new(|| {
    use std::time::{SystemTime, UNIX_EPOCH};

    use osom_lib_entropy_cprng::DefaultEntropy;

    let key1 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Couldn't calculate current timestamp")
        .as_secs();

    let mut default_entropy = DefaultEntropy::new().expect("Couldn't initialize entropy source");
    let key2 = default_entropy.generate::<u64>();

    GeneralSipHashBuilder::<1, 3>::with_keys(key1, key2)
});

impl BuildHasher for DefaultHashBuilder {
    type Hasher = GeneralSipHash<1, 3>;

    #[inline(always)]
    fn build_hasher(&self) -> Self::Hasher {
        cfg_select! {
            all(feature = "std", not(miri)) => {
                SIP_HASH.create_hasher()
            },
            _ => GeneralSipHash::<1, 3>::for_keys(3, 4)
        }
    }
}
