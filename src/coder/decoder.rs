use crate::coder::util::create_mask;

use super::util::BITS_IN_BYTE;

pub struct Decoder {
    buffer: Vec<u8>,
    bits_per_channel: u8,
    index: usize,
    mask: u8,
}

impl Decoder {
    pub fn new(buffer: Vec<u8>, bits_per_channel: u8) -> Self {
        Decoder {
            buffer,
            bits_per_channel,
            index: 0,
            mask: create_mask(bits_per_channel),
        }
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
        let mut byte: u8 = 0;
        let mut left = BITS_IN_BYTE;

        while left > 0 {
            let channel = *self.next();
            let bits = channel & self.mask;
            byte = byte.checked_shl(self.bits_per_channel as u32).unwrap_or(0);
            byte |= bits;

            left -= self.bits_per_channel;
        }

        byte
    }

    fn next(&mut self) -> &mut u8 {
        let byte = &mut self.buffer[self.index];
        self.index += 1;
        byte
    }
}

mod tests {
    #[test]
    fn decode_2bits() {
        let mut buffer = vec![0; 64];
        let mut iter = buffer.iter_mut();
        let bits_per_channel = 2;

        for _ in 0..14 {
            *iter.next().unwrap() = 0b0000_0000;
        }

        // Filename length
        *iter.next().unwrap() = 0b0000_0001;
        *iter.next().unwrap() = 0b0000_0001;

        // // x = 0111 1000
        *iter.next().unwrap() = 0b0000_0001;
        *iter.next().unwrap() = 0b0000_0011;
        *iter.next().unwrap() = 0b0000_0010;
        *iter.next().unwrap() = 0b0000_0000;

        // . = 0010 1110
        *iter.next().unwrap() = 0b0000_0000;
        *iter.next().unwrap() = 0b0000_0010;
        *iter.next().unwrap() = 0b0000_0011;
        *iter.next().unwrap() = 0b0000_0010;

        // p = 0111 0000
        *iter.next().unwrap() = 0b0000_0001;
        *iter.next().unwrap() = 0b0000_0011;
        *iter.next().unwrap() = 0b0000_0000;
        *iter.next().unwrap() = 0b0000_0000;

        // n = 0110 1110
        *iter.next().unwrap() = 0b0000_0001;
        *iter.next().unwrap() = 0b0000_0010;
        *iter.next().unwrap() = 0b0000_0011;
        *iter.next().unwrap() = 0b0000_0010;

        // g = 0110 0111
        *iter.next().unwrap() = 0b0000_0001;
        *iter.next().unwrap() = 0b0000_0010;
        *iter.next().unwrap() = 0b0000_0001;
        *iter.next().unwrap() = 0b0000_0011;

        // Message Length
        for _ in 0..15 {
            *iter.next().unwrap() = 0b0000_0000;
        }
        *iter.next().unwrap() = 0b0000_0011;

        // Data
        // x = 0111 1000
        *iter.next().unwrap() = 0b0000_0001;
        *iter.next().unwrap() = 0b0000_0011;
        *iter.next().unwrap() = 0b0000_0010;
        *iter.next().unwrap() = 0b0000_0000;

        // y = 0111 1001
        *iter.next().unwrap() = 0b0000_0001;
        *iter.next().unwrap() = 0b0000_0011;
        *iter.next().unwrap() = 0b0000_0010;
        *iter.next().unwrap() = 0b0000_0001;

        // z = 0111 1010
        *iter.next().unwrap() = 0b0000_0001;
        *iter.next().unwrap() = 0b0000_0011;
        *iter.next().unwrap() = 0b0000_0010;
        *iter.next().unwrap() = 0b0000_0010;

        let decoder = super::Decoder::new(buffer, bits_per_channel);
        let (filename, data) = decoder.decode();

        assert_eq!(filename, "x.png");
        assert_eq!(String::from_utf8(data).unwrap(), "xyz");
    }

    #[test]
    fn decode_4bits() {
        let mut buffer = vec![0; 64];
        let mut iter = buffer.iter_mut();
        let bits_per_channel = 4;

        for _ in 0..7 {
            *iter.next().unwrap() = 0b0000_0000;
        }

        // Filename length
        *iter.next().unwrap() = 0b0000_0101;

        // x = 0111 1000
        *iter.next().unwrap() = 0b0000_0111;
        *iter.next().unwrap() = 0b0000_1000;

        // . = 0010 1110
        *iter.next().unwrap() = 0b0000_0010;
        *iter.next().unwrap() = 0b0000_1110;

        // p = 0111 0000
        *iter.next().unwrap() = 0b0000_0111;
        *iter.next().unwrap() = 0b0000_0000;

        // n = 0110 1110
        *iter.next().unwrap() = 0b0000_0110;
        *iter.next().unwrap() = 0b0000_1110;

        // g = 0110 0111
        *iter.next().unwrap() = 0b0000_0110;
        *iter.next().unwrap() = 0b0000_0111;

        // Message Length
        for _ in 0..7 {
            *iter.next().unwrap() = 0b0000_0000;
        }
        *iter.next().unwrap() = 0b0000_0100;

        // Data
        // w = 0111 0111
        *iter.next().unwrap() = 0b0000_0111;
        *iter.next().unwrap() = 0b0000_0111;

        // o = 0110 1111
        *iter.next().unwrap() = 0b0000_0110;
        *iter.next().unwrap() = 0b0000_1111;

        // l = 0110 1100
        *iter.next().unwrap() = 0b0000_0110;
        *iter.next().unwrap() = 0b0000_1100;

        // f = 0110 0110
        *iter.next().unwrap() = 0b0000_0110;
        *iter.next().unwrap() = 0b0000_0110;

        let decoder = super::Decoder::new(buffer, bits_per_channel);
        let (filename, data) = decoder.decode();

        assert_eq!(filename, "x.png");
        assert_eq!(String::from_utf8(data).unwrap(), "wolf");
    }
}
