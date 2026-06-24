use crate::{
    stream_prng::StreamPRNG,
    streams::ChaChaStream,
    traits::{CryptographicallySecure, PRNGenerator, Splittable}
};

/// The `ChaCha` based PRNG. It is a cryptographically secure PRNG.
/// 
/// # Notes
/// 
/// * It is significantly slower than non-cryptographically secure PRNGS,
///   e.g. around ~10x slower than LCG.
/// * The recommended `ROUNDS` value is 20.
pub type ChaCha<const ROUNDS: u32> = StreamPRNG<ChaChaStream<ROUNDS>>;

unsafe impl<const ROUNDS: u32> CryptographicallySecure for ChaCha<ROUNDS> { }

impl<const ROUNDS: u32> Splittable for ChaCha<ROUNDS> {
    fn split(&mut self) -> Self {
        let new_key = self.generate::<[u8; 32]>();
        let new_nonce = self.generate::<[u8; 12]>();
        let new_chacha_stream = ChaChaStream::from_arrays(new_key, new_nonce);
        StreamPRNG::new(new_chacha_stream)
    }
}

/// The recommended [`ChaCha`] PRNG with 20 rounds.
pub type DefaultChaCha = ChaCha<20>;
