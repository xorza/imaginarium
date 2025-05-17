use crate::color_format::ChannelSize;

#[test]
fn from_bit_count_returns_64bit() {
    let size = ChannelSize::from_bit_count(64);
    assert_eq!(size, ChannelSize::_64bit);
}
