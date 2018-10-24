mod tests;

pub trait ByteAddressable {
    fn read8(&self) -> u8;
    fn write8(&mut self, value: u8);
}

pub trait WordAddressable {
    fn read16(&self) -> u16;
    fn write16(&mut self, value: u16);
}

struct ByteReference<'a> {
    value: &'a mut u8
}

impl<'a> ByteReference<'a> {
    fn new(value: &'a mut u8) -> ByteReference<'a> {
        ByteReference { value }
    }
}

impl<'a> ByteAddressable for ByteReference<'a> {
    fn read8(&self) -> u8 {
        return *self.value;
    }

    fn write8(&mut self, value: u8) {
        *self.value = value
    }
}

struct WordReference<T: ByteAddressable, E: ByteAddressable> {
    lower: T,
    upper: E,
}

impl<T: ByteAddressable, E: ByteAddressable> WordReference<T, E> {
    fn new(lower: T, upper: E) -> WordReference<T, E> {
        WordReference { lower, upper }
    }
}

impl<T: ByteAddressable, E: ByteAddressable> WordAddressable for WordReference<T, E> {
    fn read16(&self) -> u16 {
        let low_byte = self.lower.read8() as u16;
        let high_byte = (self.upper.read8() as u16) << 8;
        return low_byte | high_byte;
    }

    fn write16(&mut self, value: u16) {
        unimplemented!();
    }
}
