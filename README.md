# Silk-rs

## How to use

**Encode**

```rust
        let input = std::fs::read("input.pcm").unwrap();
        let output = encode_silk(input, 24000, 24000, true).unwrap();
        std::fs::write("output.silk", output).unwrap();
```

**Decode**

```rust
        let input = std::fs::read("input.silk").unwrap();
        let output = decode_silk(input, 24000).unwrap();
        std::fs::write("output.pcm", output).unwrap();
```