use super::ByteAddressable;
use super::ByteReference;
use super::Memory;
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
    let word_ref = WordReference::new(Box::new(lower_ref), Box::new(upper_ref));
    assert_eq!(0x1234, word_ref.read16());
}

#[test]
fn word_reference_can_write_value() {
    let mut upper: u8 = 0x12;
    let mut lower: u8 = 0x34;
    {
        let upper_ref = ByteReference::new(&mut upper);
        let lower_ref = ByteReference::new(&mut lower);
        let mut word_ref = WordReference::new(Box::new(lower_ref), Box::new(upper_ref));
        word_ref.write16(0xABCD);
    }
    assert_eq!(0xAB, upper);
    assert_eq!(0xCD, lower);
}

#[test]
fn memory_can_make_read_write_references() {
    let mut memory = Memory::new();
    {
        let mut test_ref = memory.get_ref(0x0000);
        assert_eq!(0x00, test_ref.read8());
        assert_eq!(0x0000, test_ref.read16());

        test_ref.write16(0x1234);

        assert_eq!(0x34, test_ref.read8());
        assert_eq!(0x1234, test_ref.read16());
    }
}

#[test]
fn memory_refs_can_overlap() {
    let mut memory = Memory::new();
    {
        let mut test_ref = memory.get_ref(0x0000);
        test_ref.write16(0x1234);
    }
    {
        let overlapping_ref = memory.get_ref(0x0001);
        assert_eq!(0x12, overlapping_ref.read8());
        assert_eq!(0x0012, overlapping_ref.read16());
    }
}
