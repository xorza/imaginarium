use crate::image_convertion::{
    u8_to_i8, i8_to_u8,
    u16_to_i16, i16_to_u16,
    u32_to_i32, i32_to_u32,
    u64_to_i64, i64_to_u64,
};

#[test]
fn round_trip_u8_i8() {
    let vals_u = [0u8, 128, 255];
    for &v in &vals_u {
        assert_eq!(v, i8_to_u8(u8_to_i8(v)));
    }
    let vals_i = [i8::MIN, 0, i8::MAX];
    for &v in &vals_i {
        assert_eq!(v, u8_to_i8(i8_to_u8(v)));
    }
}

#[test]
fn round_trip_u16_i16() {
    let vals_u = [0u16, 32768, 65535];
    for &v in &vals_u {
        assert_eq!(v, i16_to_u16(u16_to_i16(v)));
    }
    let vals_i = [i16::MIN, 0, i16::MAX];
    for &v in &vals_i {
        assert_eq!(v, u16_to_i16(i16_to_u16(v)));
    }
}

#[test]
fn round_trip_u32_i32() {
    let vals_u = [0u32, 2_147_483_648u32, u32::MAX];
    for &v in &vals_u {
        assert_eq!(v, i32_to_u32(u32_to_i32(v)));
    }
    let vals_i = [i32::MIN, 0, i32::MAX];
    for &v in &vals_i {
        assert_eq!(v, u32_to_i32(i32_to_u32(v)));
    }
}

#[test]
fn round_trip_u64_i64() {
    let vals_u = [0u64, 9_223_372_036_854_775_808u64, u64::MAX];
    for &v in &vals_u {
        assert_eq!(v, i64_to_u64(u64_to_i64(v)));
    }
    let vals_i = [i64::MIN, 0, i64::MAX];
    for &v in &vals_i {
        assert_eq!(v, u64_to_i64(i64_to_u64(v)));
    }
}
