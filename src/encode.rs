use bytes::{BufMut, BytesMut};

use crate::{sdk, CMemory};

pub fn encode_silk(
    src: Vec<u8>,
    sampleRate: i32,
    bitRate: i32,
    tencent: bool,
) -> Result<Vec<u8>, ()> {
    unsafe { _encode_silk(src, sampleRate, bitRate, tencent) }
}

unsafe fn _encode_silk(
    src: Vec<u8>,
    sampleRate: i32,
    bitRate: i32,
    tencent: bool,
) -> Result<Vec<u8>, ()> {
    let mut encControl = sdk::SKP_SILK_SDK_EncControlStruct {
        API_sampleRate: sampleRate,
        maxInternalSampleRate: 24000,
        packetSize: (20 * sampleRate) / 1000,
        bitRate,
        packetLossPercentage: 0,
        complexity: 2,
        useInBandFEC: 0,
        useDTX: 0,
    };

    let mut encStatus = sdk::SKP_SILK_SDK_EncControlStruct {
        API_sampleRate: 0,
        maxInternalSampleRate: 0,
        packetSize: 0,
        bitRate,
        packetLossPercentage: 0,
        complexity: 0,
        useInBandFEC: 0,
        useDTX: 0,
    };

    let mut encSizeBytes: i32 = 0;
    if sdk::SKP_Silk_SDK_Get_Encoder_Size(&mut encSizeBytes) != 0 {
        return Err(());
    }
    let psEnc = CMemory::new(encSizeBytes as usize);

    if sdk::SKP_Silk_SDK_InitEncoder(
        psEnc.ptr,
        &mut encStatus as *mut sdk::SKP_SILK_SDK_EncControlStruct,
    ) != 0
    {
        return Err(());
    }
    let frameSize = sampleRate / 1000 * 40;
    let mut out = BytesMut::new();
    if tencent {
        out.put_slice("\x02#!SILK_V3".as_bytes())
    } else {
        out.put_slice("#!SILK_V3".as_bytes())
    }
    let mut nBytes: i16 = 1250;
    let mut payload = vec![0; nBytes as usize];
    for chunk in src.chunks(frameSize as usize) {
        nBytes = 1250;
        if chunk.len() < frameSize as usize {
            break;
        }
        let ret = sdk::SKP_Silk_SDK_Encode(
            psEnc.ptr,
            &mut encControl,
            chunk.as_ptr() as *const i16,
            chunk.len() as i32 / 2,
            payload.as_mut_ptr(),
            &mut nBytes,
        );
        if ret != 0 {
            return Err(());
        }
        out.put_i16_le(nBytes);
        let l = nBytes as usize;
        out.put_slice(&payload[0..l]);
    }

    Ok(out.to_vec())
}
