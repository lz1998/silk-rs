use std::ffi::c_void;

use bytes::Buf;

use crate::{fast_check, sdk, SilkError};

pub fn decode_silk(src: impl AsRef<[u8]>, sample_rate: i32) -> Result<Vec<u8>, SilkError> {
    unsafe { _decode_silk(src.as_ref(), sample_rate) }
}

unsafe fn _decode_silk(mut src: &[u8], sample_rate: i32) -> Result<Vec<u8>, SilkError> {
    // skip tencent flag
    if src.starts_with(b"\x02") {
        src.advance(1);
    };

    const SILK_HEADER: &[u8] = b"#!SILK_V3";
    if src.starts_with(SILK_HEADER) {
        src.advance(SILK_HEADER.len());
    } else {
        return Err(SilkError::Invalid);
    };

    let mut dec_control = sdk::SKP_SILK_SDK_DecControlStruct {
        API_sampleRate: sample_rate,
        frameSize: 0,
        framesPerPacket: 1,
        moreInternalDecoderFrames: 0,
        inBandFECOffset: 0,
    };

    let mut decoder_size = 0;
    fast_check!(sdk::SKP_Silk_SDK_Get_Decoder_Size(&mut decoder_size));

    let mut decoder = vec![0u8; decoder_size as usize];
    fast_check!(sdk::SKP_Silk_SDK_InitDecoder(
        decoder.as_mut_ptr() as *mut c_void
    ));

    let mut result = vec![];
    let frame_size = sample_rate as usize / 1000 * 40;
    let mut buf = vec![0u8; frame_size];
    loop {
        if src.remaining() < 2 {
            break;
        }
        let input_size = src.get_i16_le();
        if input_size > frame_size as i16 {
            return Err(SilkError::Invalid);
        }
        if src.remaining() < input_size as usize {
            return Err(SilkError::Invalid);
        }

        let input;
        (input, src) = src.split_at(input_size as usize);

        let mut output_size = 0i16;
        fast_check!(sdk::SKP_Silk_SDK_Decode(
            decoder.as_mut_ptr() as *mut c_void,
            &mut dec_control,
            0,
            input.as_ptr(),
            input_size as i32,
            buf.as_mut_ptr() as *mut i16,
            &mut output_size,
        ));

        result.extend_from_slice(&buf[0..output_size as usize * 2])
    }
    Ok(result)
}
