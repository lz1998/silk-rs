use std::ffi::c_void;

use bytes::BufMut;

use crate::{fast_check, sdk, SilkError};

pub fn encode_silk(
    src: impl AsRef<[u8]>,
    sample_rate: i32,
    bit_rate: i32,
    tencent: bool,
) -> Result<Vec<u8>, SilkError> {
    unsafe { _encode_silk(src.as_ref(), sample_rate, bit_rate, tencent) }
}

unsafe fn _encode_silk(
    src: &[u8],
    sample_rate: i32,
    bit_rate: i32,
    tencent: bool,
) -> Result<Vec<u8>, SilkError> {
    let enc_control = sdk::SKP_SILK_SDK_EncControlStruct {
        API_sampleRate: sample_rate,
        maxInternalSampleRate: 24000,
        packetSize: (20 * sample_rate) / 1000,
        bitRate: bit_rate,
        packetLossPercentage: 0,
        complexity: 2,
        useInBandFEC: 0,
        useDTX: 0,
    };

    let mut enc_status = sdk::SKP_SILK_SDK_EncControlStruct {
        API_sampleRate: 0,
        maxInternalSampleRate: 0,
        packetSize: 0,
        bitRate: bit_rate,
        packetLossPercentage: 0,
        complexity: 0,
        useInBandFEC: 0,
        useDTX: 0,
    };

    let mut encoder_size = 0;
    fast_check!(sdk::SKP_Silk_SDK_Get_Encoder_Size(&mut encoder_size));

    let mut encoder = vec![0u8; encoder_size as usize];

    fast_check!(sdk::SKP_Silk_SDK_InitEncoder(
        encoder.as_mut_ptr() as *mut c_void,
        &mut enc_status,
    ));

    let mut result = vec![];
    if tencent {
        result.put_u8(b'\x02');
    }
    result.extend_from_slice(b"#!SILK_V3");

    let frame_size = sample_rate as usize / 1000 * 40;
    let mut output_size = 1250i16;
    let mut buf = vec![0u8; output_size as usize];
    for chunk in src.chunks(frame_size) {
        output_size = 1250;
        if chunk.len() < frame_size {
            break;
        }
        fast_check!(sdk::SKP_Silk_SDK_Encode(
            encoder.as_mut_ptr() as *mut c_void,
            &enc_control,
            chunk.as_ptr() as *const i16,
            chunk.len() as i32 / 2,
            buf.as_mut_ptr(),
            &mut output_size,
        ));
        result.put_i16_le(output_size);
        result.extend_from_slice(&buf[0..output_size as usize]);
    }

    Ok(result)
}
