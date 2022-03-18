pub use decode::decode_silk;
pub use encode::encode_silk;
pub use error::SilkError;

mod decode;
mod encode;
mod error;

macro_rules! fast_check {
    ($call:expr) => {{
        let code = $call;
        if code != 0 {
            return Err(SilkError::from(code));
        }
    }};
}

pub(crate) use fast_check;

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
pub(crate) mod sdk {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[cfg(test)]
mod tests {
    use crate::decode_silk;
    use crate::encode_silk;

    #[test]
    fn test_encode() {
        let input = std::fs::read("input.pcm").unwrap();
        let output = encode_silk(input, 24000, 24000, true).unwrap();
        std::fs::write("output.silk", output).unwrap();
    }

    #[test]
    fn test_decode() {
        let input = std::fs::read("input.silk").unwrap();
        let output = decode_silk(input, 24000).unwrap();
        std::fs::write("output.pcm", output).unwrap();
    }
}
