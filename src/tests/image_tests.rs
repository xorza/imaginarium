use crate::color_format::*;
use crate::image::{Image, ImageDesc};
use bytemuck::{cast_slice, cast_vec};

#[test]
fn it_works() {
    let tiff = Image::read_file("./test_resources/rgb-sample-32bit.tiff").unwrap();
    assert_eq!(tiff.desc.width(), 256);
    assert_eq!(tiff.desc.height(), 1);
    assert_eq!(tiff.desc.stride(), 3072);
    assert_eq!(tiff.desc.color_format().channel_size, ChannelSize::_32bit);
    assert_eq!(tiff.desc.color_format().channel_count, ChannelCount::Rgb);

    let png = Image::read_file("./test_resources/rgba-sample-8bit.png").unwrap();
    assert_eq!(png.desc.width(), 864);
    assert_eq!(png.desc.height(), 409);
    assert_eq!(png.desc.stride(), 3456);
    assert_eq!(png.desc.color_format().channel_size, ChannelSize::_8bit);
    assert_eq!(png.desc.color_format().channel_count, ChannelCount::Rgba);

    let png = Image::read_file("./test_resources/rgb-sample-8bit.png").unwrap();
    assert_eq!(png.desc.width(), 331);
    assert_eq!(png.desc.height(), 126);
    assert_eq!(png.desc.stride(), 993);
    assert_eq!(png.desc.color_format().channel_size, ChannelSize::_8bit);
    assert_eq!(png.desc.color_format().channel_count, ChannelCount::Rgb);
}

#[test]
fn save_rgb_png() {
    let png = Image::read_file("./test_resources/rgb-sample-8bit.png").unwrap();

    png.save_file("./test_output/save_rgb.png").unwrap();
}

#[test]
fn image_convertion() {
    let png = Image::read_file("./test_resources/rgba-sample-8bit.png").unwrap();

    png.save_file("./test_output/rgba-sample-8bit.png")
        .unwrap();

    png.clone()
        .convert(ColorFormat::GRAY_U16)
        .unwrap()
        .save_file("./test_output/convertion-gray-u16.png")
        .unwrap();

    png.clone()
        .convert(ColorFormat::RGB_U16)
        .unwrap()
        .save_file("./test_output/convertion-rgb-u16.png")
        .unwrap();

    let tiff = Image::read_file("./test_resources/rgb-sample-32bit.tiff").unwrap();

    tiff.save_file("./test_output/rgb-sample-32bit.tiff")
        .unwrap();

    tiff.clone()
        .convert(ColorFormat::GRAY_U16)
        .unwrap()
        .save_file("./test_output/convertion-gray-u16.tiff")
        .unwrap();

    tiff.clone()
        .convert(ColorFormat::RGBA_U16)
        .unwrap()
        .save_file("./test_output/convertion-rgba-u16.tiff")
        .unwrap();

    tiff.clone()
        .convert(ColorFormat::RGBA_U8)
        .unwrap()
        .save_file("./test_output/convertion-rgba-u8.tiff")
        .unwrap();

    tiff.clone()
        .convert(ColorFormat::GRAY_I8)
        .unwrap()
        .save_file("./test_output/convertion-gray-i8.tiff")
        .unwrap();

    tiff.clone()
        .convert(ColorFormat::RGBA_F64)
        .unwrap()
        .save_file("./test_output/convertion-rgba-f64.tiff")
        .unwrap();

    tiff.clone()
        .convert(ColorFormat::RGBA_U64)
        .unwrap()
        .save_file("./test_output/convertion-rgba-u64.tiff")
        .unwrap();

    tiff.clone()
        .convert(ColorFormat::RGBA_I32)
        .unwrap()
        .save_file("./test_output/convertion-gray-i64.tiff")
        .unwrap();

    tiff.clone()
        .convert(ColorFormat::RGBA_F32)
        .unwrap()
        .save_file("./test_output/convertion-rgba-f32.tiff")
        .unwrap();

    tiff.clone()
        .convert(ColorFormat::GRAY_ALPHA_U8)
        .unwrap()
        .save_file("./test_output/convertion-ga-u8.tiff")
        .unwrap();

    tiff.clone()
        .convert(ColorFormat::RGB_I32)
        .unwrap()
        .save_file("./test_output/convertion-rgb-i32.tiff")
        .unwrap();

    tiff.clone()
        .convert(ColorFormat::RGBA_F32)
        .unwrap()
        .convert(ColorFormat::RGBA_U16)
        .unwrap()
        .save_file("./test_output/convertion-x2-rgba-u16.tiff")
        .unwrap();
}

#[test]
fn save_tiff_with_misaligned_bytes_returns_error() {
    use crate::image::ImageDesc;

    let desc = ImageDesc::new(1, 1, ColorFormat::GRAY_U16);
    let img = Image::new_with_data(desc, vec![0u8; 3]).unwrap();

    let result = img.save_file("./test_output/misaligned.tiff");
    assert!(result.is_err());
}

fn read_missing_png_propagates_error() {
    let result = Image::read_file("./test_resources/does_not_exist.png");
    assert!(result.is_err());
}

#[test]
fn save_tiff_invalid_bytes_propagates_error() {
    let desc = ImageDesc::new(1, 1, ColorFormat::GRAY_U16);
    // 3 bytes is not a multiple of u16 size, so cast_slice should fail
    let img = Image::new_with_data(desc, vec![0u8; 3]).unwrap();

    let result = img.save_file("./test_output/invalid.tiff");
    assert!(result.is_err());
}

fn save_rgba_int_tiffs() {
    let png = Image::read_file("./test_resources/rgba-sample-8bit.png").unwrap();

    png.clone()
        .convert(ColorFormat::RGBA_I8)
        .unwrap()
        .save_file("./test_output/save-rgba-i8.tiff")
        .unwrap();

    png.clone()
        .convert(ColorFormat::RGBA_I16)
        .unwrap()
        .save_file("./test_output/save-rgba-i16.tiff")
        .unwrap();

    png.clone()
        .convert(ColorFormat::RGBA_I32)
        .unwrap()
        .save_file("./test_output/save-rgba-i32.tiff")
        .unwrap();

    png
        .convert(ColorFormat::RGBA_I64)
        .unwrap()
        .save_file("./test_output/save-rgba-i64.tiff")
        .unwrap();
}

#[test]
fn convert_rgba_u8_to_rgba_i8() {
    let desc = ImageDesc::new(1, 1, ColorFormat::RGBA_U8);
    let src = Image::new_with_data(desc, vec![0, 128, 255, 64]).unwrap();
    let result = src.convert(ColorFormat::RGBA_I8).unwrap();
    assert_eq!(result.desc.color_format(), ColorFormat::RGBA_I8);
    let expected_vals = [
        crate::image_convertion::u8_to_i8(0),
        crate::image_convertion::u8_to_i8(128),
        crate::image_convertion::u8_to_i8(255),
        crate::image_convertion::u8_to_i8(64),
    ];
    let expected_bytes: Vec<u8> = bytemuck::cast_slice(&expected_vals).to_vec();
    assert_eq!(result.bytes, expected_bytes);
}

#[test]
fn convert_rgb_u16_to_rgb_i16() {
    let desc = ImageDesc::new(1, 1, ColorFormat::RGB_U16);
    let src = Image::new_with_data(desc, bytemuck::cast_vec(vec![0u16, 32768, 65535])).unwrap();
    let result = src.convert(ColorFormat::RGB_I16).unwrap();
    assert_eq!(result.desc.color_format(), ColorFormat::RGB_I16);
    let expected_vals = [
        crate::image_convertion::u16_to_i16(0),
        crate::image_convertion::u16_to_i16(32768),
        crate::image_convertion::u16_to_i16(65535),
    ];
    let expected_bytes: Vec<u8> = bytemuck::cast_slice(&expected_vals).to_vec();
    assert_eq!(result.bytes, expected_bytes);
}

#[test]
fn convert_gray_u8_to_gray_i8() {
    let desc = ImageDesc::new(1, 1, ColorFormat::GRAY_U8);
    let src = Image::new_with_data(desc, vec![200]).unwrap();
    let result = src.convert(ColorFormat::GRAY_I8).unwrap();
    assert_eq!(result.desc.color_format(), ColorFormat::GRAY_I8);
    let expected_val = crate::image_convertion::u8_to_i8(200);
    let expected_bytes: Vec<u8> = bytemuck::cast_slice(&[expected_val]).to_vec();
    assert_eq!(result.bytes, expected_bytes);
}
