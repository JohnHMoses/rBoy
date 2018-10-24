mod tests;

pub trait ByteAddressable {
    fn read8(&self) -> u8;
    fn write8(&mut self, value: u8);
}

pub trait WordAddressable {
    fn read8(&self) -> u8;
    fn write8(&mut self, value: u8);

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

struct WordReference<'a> {
    lower: Box<ByteAddressable + 'a>,
    upper: Box<ByteAddressable + 'a>,
}

impl<'a> WordReference<'a> {
    fn new(lower: Box<ByteAddressable + 'a>, upper: Box<ByteAddressable + 'a>) -> WordReference<'a> {
        WordReference { lower, upper }
    }
}

impl<'a> WordAddressable for WordReference<'a> {
    fn read8(&self) -> u8 {
        return (*self.lower).read8();
    }

    fn write8(&mut self, value: u8) {
        (*self.lower).write8(value);
    }

    fn read16(&self) -> u16 {
        let low_byte = (*self.lower).read8() as u16;
        let high_byte = ((*self.upper).read8() as u16) << 8;
        return low_byte | high_byte;
    }

    fn write16(&mut self, value: u16) {
        (*self.lower).write8(value as u8);
        (*self.upper).write8((value >> 8) as u8);
    }
}

struct Memory {
    mem: [u8; 0x10000],
}

impl<'a> Memory {
    pub fn new() -> Memory {
        Memory { mem: [0; 0x10000] }
    }

    pub fn get_ref(&'a mut self, address: u16) -> WordReference {
        let real_address = address as usize;
        assert!(real_address != 0xFFFF); // TODO: handle later
        let (low_ref, high_ref) = self.get_byte_refs(real_address);
        return WordReference::new(low_ref, high_ref);
    }

    fn get_byte_refs(&'a mut self, address: usize) -> (Box<ByteAddressable + 'a>, Box<ByteAddressable + 'a>) {
        assert!(address < self.mem.len());
        // Use some magic to grab two mutables references to the same slice
        let s: &'a mut[u8] = &mut self.mem;
        // Divide the slice into two mutable subslices
        // with the lower byte as the last elm of the first
        // and with the upper byte as the first elm of the second
        let (low, high) = s.split_at_mut(address + 1);
        let lower = low.last_mut().unwrap();
        let upper = high.first_mut().unwrap();
        return (Box::new(ByteReference::new(lower)), Box::new(ByteReference::new(upper)));
    }
}
