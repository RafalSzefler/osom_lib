#![allow(clippy::new_without_default)]

use core::marker::PhantomData;

use osom_lib_arrays::fixed_array::ConstBuffer;
use osom_lib_arrays::{const_helpers::subslice_mut_const, fixed_array::ConstBufferer};
use osom_lib_reprc::macros::reprc;

use crate::sha2::sha2_256::{
    portable::SHA2_256_Portable,
    sha2_256_shared::{INITIAL_STATE, calculate_final_blocks},
};

/// The actual update that the `SHA2_256` implementation has to do.
pub trait SHA2_256_Updater {
    fn update_state(state: &mut [u32; 8], bufferer: &mut ConstBufferer<'_, 64, u8>);
}

/// A template for various `SHA2_256` implementation variants.
#[reprc]
#[must_use]
pub struct SHA2_256_Template<TUpdater: SHA2_256_Updater> {
    // This field is used in the final block calculation.
    // It is first due to its size (we are using #[repr(C)]).
    total_length: u64,

    // The internal state of `SHA2_256` hasher.
    state: [u32; 8],

    // The buffered message block. We need to keep it because final block needs to be processed differently.
    bufferer: ConstBuffer<64, u8>,

    _phantom: PhantomData<TUpdater>,
}

impl<TUpdater: SHA2_256_Updater> SHA2_256_Template<TUpdater> {
    /// Creates a new [`SHA2_256_Template`] instance.
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            total_length: 0,
            state: INITIAL_STATE,
            bufferer: ConstBuffer::new(),
            _phantom: PhantomData,
        }
    }

    /// Writes a block of data to the underlying state.
    ///
    /// # Panics
    ///
    /// This function will panic if the length of the data is greater than `u32::MAX`,
    /// and when the total processed length exceeds `u64::MAX`.
    pub fn update(&mut self, data: impl AsRef<[u8]>) {
        let data = data.as_ref();
        let len = data.len();
        if len == 0 {
            return;
        }

        assert!(
            u32::try_from(len).is_ok(),
            "The max size of a chunk for SHA2_256 is u32::MAX."
        );

        let len: u64 = len as u64;

        assert!(
            self.total_length <= (u64::MAX / 8) - len,
            "The total length of the data that SHA2_256 can process is u64::MAX / 8. This limit is reached."
        );

        // This is safe due to previous assertions.
        self.total_length = unsafe { self.total_length.unchecked_add(len) };

        let mut iterator = self.bufferer.buffer_const(data);
        TUpdater::update_state(&mut self.state, &mut iterator);
    }

    /// Calculates the final hash value.
    ///
    /// # Notes
    ///
    /// This function does not update the internal state, and thus
    /// the processor can still be used afterwards.
    pub fn result(&self, output: &mut [u8; 32]) {
        // Build the final SHA2-256 block.
        let mut state = self.state;

        let final_blocks = calculate_final_blocks(self.total_length, self.bufferer.current_state_const());
        let mut tmp_buffer = ConstBuffer::<64, u8>::new();
        let mut iterator = tmp_buffer.buffer_const(final_blocks.as_slice());
        TUpdater::update_state(&mut state, &mut iterator);

        // Finally write the output via big-endian encoding.
        unsafe {
            subslice_mut_const(output, 0..4).copy_from_slice(&state[0].to_be_bytes());
            subslice_mut_const(output, 4..8).copy_from_slice(&state[1].to_be_bytes());
            subslice_mut_const(output, 8..12).copy_from_slice(&state[2].to_be_bytes());
            subslice_mut_const(output, 12..16).copy_from_slice(&state[3].to_be_bytes());
            subslice_mut_const(output, 16..20).copy_from_slice(&state[4].to_be_bytes());
            subslice_mut_const(output, 20..24).copy_from_slice(&state[5].to_be_bytes());
            subslice_mut_const(output, 24..28).copy_from_slice(&state[6].to_be_bytes());
            subslice_mut_const(output, 28..32).copy_from_slice(&state[7].to_be_bytes());
        }
    }
}

impl<TUpdater: SHA2_256_Updater> From<SHA2_256_Portable> for SHA2_256_Template<TUpdater> {
    #[inline(always)]
    fn from(portable: SHA2_256_Portable) -> Self {
        Self {
            total_length: portable.total_length,
            state: portable.state,
            bufferer: portable.bufferer,
            _phantom: PhantomData,
        }
    }
}

impl<TUpdater: SHA2_256_Updater> From<SHA2_256_Template<TUpdater>> for SHA2_256_Portable {
    #[inline(always)]
    fn from(template: SHA2_256_Template<TUpdater>) -> Self {
        SHA2_256_Portable::from_pieces(template.total_length, template.state, template.bufferer)
    }
}
