use bytes::{Buf, BufMut, Bytes, BytesMut};

use crate::{sdk, CMemory};

pub fn decode_silk(src: Vec<u8>, sampleRate: i32) -> Result<Vec<u8>, ()> {
    unsafe { _decode_silk(src, sampleRate) }
}

unsafe fn _decode_silk(src: Vec<u8>, sample_rate: i32) -> Result<Vec<u8>, ()> {
    let mut src = Bytes::from(src);
    if src.starts_with("#!SILK_V3".as_bytes()) {
        src.advance("#!SILK_V3".len());
    } else if src.starts_with("\x02#!SILK_V3".as_bytes()) {
        src.advance("\x02#!SILK_V3".len());
    } else {
        return Err(());
    }
    let mut dec_control = sdk::SKP_SILK_SDK_DecControlStruct {
        API_sampleRate: sample_rate,
        frameSize: 0,
        framesPerPacket: 1,
        moreInternalDecoderFrames: 0,
        inBandFECOffset: 0,
    };
    let mut dec_size = 0;
    sdk::SKP_Silk_SDK_Get_Decoder_Size(&mut dec_size);
    let dec = CMemory::new(dec_size as usize);
    if sdk::SKP_Silk_SDK_InitDecoder(dec.ptr) != 0 {
        return Err(());
    }
    let frame_size = sample_rate as usize / 1000 * 40;
    let mut buf: Vec<u8> = vec![0; frame_size];
    let mut out = BytesMut::new();
    loop {
        if src.remaining() < 2 {
            break;
        }
        let mut n_bytes = src.get_i16_le();
        if n_bytes > frame_size as i16 {
            return Err(());
        }
        if src.remaining() < n_bytes as usize {
            return Err(());
        }
        let input = src.copy_to_bytes(n_bytes as usize);
        let ret = sdk::SKP_Silk_SDK_Decode(
            dec.ptr,
            &mut dec_control,
            0,
            input.as_ptr(),
            n_bytes as i32,
            buf.as_mut_ptr() as *mut i16,
            &mut n_bytes,
        );
        if ret != 0 {
            return Err(());
        }
        let l = n_bytes as usize * 2;
        out.put_slice(&buf[0..l])
    }
    Ok(out.to_vec())
}
