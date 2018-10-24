use super::ByteAddressable;
use super::ByteReference;
use super::WordAddressable;
use super::WordReference;

#[test]
fn byte_reference_can_read_value() {
    let mut value: u8 = 42;
    let byte_ref = ByteReference::new(&mut value);
    assert_eq!(42, byte_ref.read8());
}

#[test]
fn byte_reference_can_write_value() {
    let mut value: u8 = 42;
    {
        let mut byte_ref = ByteReference::new(&mut value);
        byte_ref.write8(41);
    }
    assert_eq!(41, value);
}

#[test]
fn word_reference_can_read_value() {
    let mut upper: u8 = 0x12;
    let mut lower: u8 = 0x34;
    let upper_ref = ByteReference::new(&mut upper);
    let lower_ref = ByteReference::new(&mut lower);
    let word_ref = WordReference::new(lower_ref, upper_ref);
    assert_eq!(0x1234, word_ref.read16());
}
