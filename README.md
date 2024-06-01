Rust library for image decoding and encoding. Supports JPEG, PNG, and TIFF.

Usage example:
```rust
let img = Image::read_file("image.png").unwrap();
png
    .convert(ColorFormat::GRAY_U16)
    .unwrap()
    .save_file("gray.png")
.unwrap();
```

Supports wide variety of color formats and pixel types and can convert between them.
```rust
pub enum ChannelCount {
    Gray = 1,
    GrayAlpha = 2,
    Rgb = 3,
    Rgba = 4,
}
pub enum ChannelSize {
    _8bit = 1,
    _16bit = 2,
    _32bit = 4,
    _64bit = 8,
}
pub enum ChannelType {
    UInt,
    Float,
    Int,
}
```