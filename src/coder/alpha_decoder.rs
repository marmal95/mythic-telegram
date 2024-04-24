pub struct AlphaDecoder {
    buffer: Vec<u8>,
    index: usize,
}

impl AlphaDecoder {
    pub fn new(buffer: Vec<u8>) -> Self {
        AlphaDecoder { buffer, index: 3 }
    }

    pub fn decode(mut self) -> (String, Vec<u8>) {
        let file_name_length = self.decode_length();
        let file_name = String::from_utf8(self.decode_data(file_name_length)).unwrap();

        let data_length = self.decode_length();
        let data = self.decode_data(data_length);

        (file_name, data)
    }

    fn decode_length(&mut self) -> usize {
        u32::from_be_bytes([
            self.decode_byte(),
            self.decode_byte(),
            self.decode_byte(),
            self.decode_byte(),
        ]) as usize
    }

    fn decode_data(&mut self, length: usize) -> Vec<u8> {
        (0..length).map(|_| self.decode_byte()).collect()
    }

    fn decode_byte(&mut self) -> u8 {
        *self.next()
    }

    fn next(&mut self) -> &mut u8 {
        let byte = &mut self.buffer[self.index];
        self.index += 4;
        byte
    }
}
