#![allow(clippy::byte_char_slices)]

use osom_lib_alloc::traits::Allocator;
use osom_lib_arrays::{
    fixed_array::InlineFixedArray,
    traits::{ImmutableArray as _, MutableArray},
};

use crate::cvr::{CVR, CVRString};

/// Binarizes the given [`CVR`] value into a byte array stream.
///
/// # Notes
///
/// * This function traverses the value recursively and produces a unique binary
///   representation of the [`CVR`] chunked into pieces.
/// * The binary representation tries to be as compact as possible.
/// * No allocation occures during the process.
pub fn binarize(cvr: &CVR<impl Allocator>, mut action: impl FnMut(&[u8])) {
    binarize_inner(cvr, &mut action);
}

#[inline(always)]
fn new_buffer() -> InlineFixedArray<26, u8> {
    InlineFixedArray::new()
}

fn binarize_inner(cvr: &CVR<impl Allocator>, action: &mut impl FnMut(&[u8])) {
    let mut buffer = new_buffer();

    match cvr {
        CVR::Null => {
            action(&[b'N']);
        }
        CVR::Bool(cvrbool) => {
            let byte = if cvrbool.inner() { b'1' } else { b'0' };
            action(&[b'B', byte]);
        }
        CVR::Int(cvrint) => {
            buffer.try_push(b'I').expect("Failed to push int tag to buffer");

            let raw_value = cvrint.inner();
            if let Ok(i64_value) = i64::try_from(raw_value) {
                encode_i64(i64_value, &mut buffer);
                action(buffer.as_ref());
                return;
            }

            encode_i128(raw_value, &mut buffer);
            action(buffer.as_ref());
        }
        CVR::String(cvrstring) => {
            binarize_string(cvrstring, &mut buffer, action);
        }
        CVR::Float(cvrfloat) => {
            buffer.try_push(b'F').expect("Failed to push fraction tag to buffer");
            buffer
                .try_push_array(cvrfloat.inner().to_le_bytes())
                .expect("Failed to push float value to buffer");
            action(buffer.as_ref());
        }
        CVR::Array(cvrarray) => {
            let length = cvrarray.inner_ref().length().as_u32();
            buffer.try_push(b'A').expect("Failed to push array tag to buffer");
            encode_i64(i64::from(length), &mut buffer);
            action(buffer.as_ref());
            let inner = cvrarray.inner_ref().as_ref();
            for item in inner {
                binarize_inner(item, action);
            }
        }
        CVR::Object(cvrobject) => {
            let length = cvrobject.len().as_u32();
            buffer.try_push(b'O').expect("Failed to push object tag to buffer");
            encode_i64(i64::from(length), &mut buffer);
            action(buffer.as_ref());
            for (key, value) in cvrobject.iter() {
                buffer = new_buffer();
                binarize_string(key, &mut buffer, action);
                binarize_inner(value, action);
            }
        }
    }
}

fn binarize_string<TAllocator: Allocator>(
    cvrstring: &CVRString<TAllocator>,
    buffer: &mut impl MutableArray<u8>,
    action: &mut impl FnMut(&[u8]),
) {
    let imm = cvrstring.as_immutable_string();
    let text_len = imm.length().as_u32();
    buffer.try_push(b'S').expect("Failed to push string tag to buffer");
    encode_i64(i64::from(text_len), buffer);

    let buffer_capacity = buffer.capacity().as_u32();
    let remaining_capacity = buffer_capacity.saturating_sub(text_len);

    let imm_bytes = imm.as_str().as_bytes();

    if text_len < remaining_capacity {
        buffer
            .try_push_slice(imm_bytes)
            .expect("Failed to push string data to buffer");
        action(buffer.as_ref());
    } else {
        action(buffer.as_ref());
        action(imm_bytes);
    }
}

fn encode_i64(value: i64, buffer: &mut impl MutableArray<u8>) {
    let mut unsigned_value = osom_lib_numbers::zigzag::zigzag_encode64(value);

    let mut local_buffer = [0u8; 10];
    let mut offset = 0;
    loop {
        let final_byte = (unsigned_value & 0b0111_1111) as u8;
        let continuation_byte = final_byte | 0b1000_0000;
        unsigned_value >>= 7;
        let byte = core::hint::select_unpredictable(unsigned_value > 0, continuation_byte, final_byte);
        local_buffer[offset] = byte;
        offset += 1;

        if unsigned_value == 0 {
            break;
        }
    }

    buffer
        .try_push_slice(&local_buffer[..offset])
        .expect("Failed to push bytes to buffer");
}

fn encode_i128(value: i128, buffer: &mut impl MutableArray<u8>) {
    // This looks the same as `encode_i64`, but it is doing 128-bit
    // arithmetic. This is significantly slower.
    let mut unsigned_value = osom_lib_numbers::zigzag::zigzag_encode128(value);
    let mut local_buffer = [0u8; 20];
    let mut offset = 0;
    loop {
        let final_byte = (unsigned_value & 0b0111_1111) as u8;
        let continuation_byte = final_byte | 0b1000_0000;
        unsigned_value >>= 7;
        let byte = core::hint::select_unpredictable(unsigned_value > 0, continuation_byte, final_byte);
        local_buffer[offset] = byte;
        offset += 1;

        if unsigned_value == 0 {
            break;
        }
    }

    buffer
        .try_push_slice(&local_buffer[..offset])
        .expect("Failed to push bytes to buffer");
}
