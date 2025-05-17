# Imaginarium

Imaginarium is a Rust library for decoding and encoding images. It supports JPEG, PNG and TIFF formats and provides convenient utilities for converting between many color formats.

## Installation

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
imaginarium = "0.1"
```

## Usage

```rust
use imaginarium::{Image, ColorFormat};

fn main() -> anyhow::Result<()> {
    let img = Image::read_file("image.png")?;
    img.convert(ColorFormat::GRAY_U16)?
        .save_file("gray.png")?;
    Ok(())
}
```

## Features

- JPEG, PNG and TIFF decoding and encoding
- Conversion between a variety of color formats
- Generic image descriptors and pixel types

## License

Imaginarium is licensed under the MIT License. See [LICENSE](LICENSE) for details.

