use bytes::{BufMut, BytesMut};

use crate::{sdk, CMemory, SilkError};

pub fn encode_silk(
    src: Vec<u8>,
    sample_rate: i32,
    bit_rate: i32,
    tencent: bool,
) -> Result<Vec<u8>, SilkError> {
    unsafe { _encode_silk(src, sample_rate, bit_rate, tencent) }
}

unsafe fn _encode_silk(
    src: Vec<u8>,
    sample_rate: i32,
    bit_rate: i32,
    tencent: bool,
) -> Result<Vec<u8>, SilkError> {
    let mut enc_control = sdk::SKP_SILK_SDK_EncControlStruct {
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

    let mut enc_size_bytes: i32 = 0;
    let code = sdk::SKP_Silk_SDK_Get_Encoder_Size(&mut enc_size_bytes);
    if code != 0 {
        return Err(SilkError::from(code));
    }
    let enc = CMemory::new(enc_size_bytes as usize);

    let code = sdk::SKP_Silk_SDK_InitEncoder(
        enc.ptr,
        &mut enc_status as *mut sdk::SKP_SILK_SDK_EncControlStruct,
    );
    if code != 0 {
        return Err(SilkError::from(code));
    }
    let frame_size = sample_rate / 1000 * 40;
    let mut out = BytesMut::new();
    if tencent {
        out.put_slice("\x02#!SILK_V3".as_bytes())
    } else {
        out.put_slice("#!SILK_V3".as_bytes())
    }
    let mut n_bytes: i16 = 1250;
    let mut payload = vec![0; n_bytes as usize];
    for chunk in src.chunks(frame_size as usize) {
        n_bytes = 1250;
        if chunk.len() < frame_size as usize {
            break;
        }
        let code = sdk::SKP_Silk_SDK_Encode(
            enc.ptr,
            &mut enc_control,
            chunk.as_ptr() as *const i16,
            chunk.len() as i32 / 2,
            payload.as_mut_ptr(),
            &mut n_bytes,
        );
        if code != 0 {
            return Err(SilkError::from(code));
        }
        out.put_i16_le(n_bytes);
        let l = n_bytes as usize;
        out.put_slice(&payload[0..l]);
    }

    Ok(out.to_vec())
}
