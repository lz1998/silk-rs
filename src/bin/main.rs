use silk_rs::{decode_silk, encode_silk};

fn main() {
    let input = std::fs::read("test.pcm").unwrap();
    let output = encode_silk(input, 24000, 24000, true).unwrap();
    std::fs::write("output.silk", output).unwrap();

    let input = std::fs::read("output.silk").unwrap();
    let output = decode_silk(input, 24000).unwrap();
    std::fs::write("output.pcm", output).unwrap();
}
