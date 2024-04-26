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

mod tests {
    #[test]
    fn decode() {
        let mut buffer = vec![0; 68];
        let mut iter = buffer.iter_mut().skip(3).step_by(4);

        // Filename length
        fill_encoded(
            &mut iter,
            &[0b0000_0000, 0b0000_0000, 0b0000_0000, 0b00000101],
        );

        // Filename
        fill_encoded(
            &mut iter,
            &[0b001111000, 0b00101110, 0b01110000, 0b01101110, 0b01100111],
        );

        // Message length
        fill_encoded(
            &mut iter,
            &[0b0000_0000, 0b0000_0000, 0b0000_0000, 0b00000100],
        );
        // Message
        fill_encoded(
            &mut iter,
            &[0b0111_0111, 0b0110_1111, 0b0110_1100, 0b0110_0110],
        );

        let decoder = super::AlphaDecoder::new(buffer);
        let (filename, data) = decoder.decode();

        assert_eq!(filename, "x.png");
        assert_eq!(String::from_utf8(data).unwrap(), "wolf");
    }

    #[allow(dead_code)]
    fn fill_encoded<'a>(iter: &mut impl Iterator<Item = &'a mut u8>, bytes: &[u8]) {
        for &byte in bytes {
            *iter.next().unwrap() = byte;
        }
    }
}
