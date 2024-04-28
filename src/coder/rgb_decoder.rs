use crate::coder::util::{create_mask, BITS_IN_BYTE};

pub struct RgbDecoder {
    buffer: Vec<u8>,
    bits_per_channel: u8,
    index: usize,
    mask: u8,
}

impl RgbDecoder {
    pub fn new(buffer: Vec<u8>, bits_per_channel: u8) -> Self {
        RgbDecoder {
            buffer,
            bits_per_channel,
            index: 0,
            mask: create_mask(bits_per_channel),
        }
    }

    pub fn decode(mut self) -> Result<(String, Vec<u8>), &'static str> {
        self.validate_data_available(4)?;
        let file_name_length = self.decode_length();

        self.validate_data_available(file_name_length)?;
        let file_name = String::from_utf8(self.decode_data(file_name_length)).unwrap();

        self.validate_data_available(4)?;
        let data_length = self.decode_length();

        self.validate_data_available(data_length)?;
        let data = self.decode_data(data_length);

        Ok((file_name, data))
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

    fn validate_data_available(&self, length: usize) -> Result<(), &'static str> {
        let all_bytes = self.buffer.len();
        let curr_byte = self.index;
        let left_bytes = all_bytes - curr_byte;
        let left_bits = left_bytes * self.bits_per_channel as usize;

        (left_bits >= length * BITS_IN_BYTE as usize)
            .then(|| {})
            .ok_or("Not enough data to decode")
    }
}

mod tests {
    use std::slice::IterMut;

    #[test]
    fn not_enough_data_to_decode_filename_length() {
        let bits_per_channel: u8 = 4;
        let min_required_data = (4 * super::BITS_IN_BYTE / bits_per_channel) as usize;
        let buffer = vec![0; min_required_data - 1];

        let decoder = super::RgbDecoder::new(buffer, bits_per_channel);
        let decoded = decoder.decode();

        assert_eq!(decoded, Err("Not enough data to decode"));
    }

    #[test]
    fn not_enough_data_to_decode_filename() {
        let filename_length = 12;
        let bits_per_channel: u8 = 4;

        let min_required_data =
            ((4 + filename_length) * super::BITS_IN_BYTE / bits_per_channel) as usize;
        let mut buffer = vec![0; min_required_data - 1];
        let mut iter = buffer.iter_mut();

        fill_encoded(&mut iter, &[0; 7]);
        fill_encoded(&mut iter, &[filename_length]);

        let decoder = super::RgbDecoder::new(buffer, bits_per_channel);
        let decoded = decoder.decode();
        assert_eq!(decoded, Err("Not enough data to decode"));
    }

    #[test]
    fn not_enough_data_to_decode_data_length() {
        let filename_length = 12;
        let bits_per_channel: u8 = 2;

        let min_required_data =
            ((4 + 4 + filename_length) * super::BITS_IN_BYTE / bits_per_channel) as usize;
        let mut buffer = vec![0; min_required_data - 1];
        let mut iter = buffer.iter_mut();

        fill_encoded(&mut iter, &[0; 14]);
        fill_encoded(&mut iter, &[0b0000_0011, 0b0000_0000]); // 1100 = 12

        let decoder = super::RgbDecoder::new(buffer, bits_per_channel);
        let decoded = decoder.decode();
        assert_eq!(decoded, Err("Not enough data to decode"));
    }

    #[test]
    fn not_enough_data_to_decode_data() {
        let filename_length: i32 = 3;
        let data_length: i32 = 13;
        let bits_per_channel: u8 = 4;

        let min_required_data = ((4 + 4 + filename_length + data_length)
            * super::BITS_IN_BYTE as i32
            / bits_per_channel as i32) as usize;
        let mut buffer = vec![0; min_required_data - 1];
        let mut iter = buffer.iter_mut();

        fill_encoded(&mut iter, &[0; 7]);
        fill_encoded(&mut iter, &[filename_length as u8]);

        // Filename
        fill_encoded(
            &mut iter,
            &[
                0b0000_0111,
                0b0000_0000,
                0b0000_0110,
                0b0000_1110,
                0b0000_0110,
                0b0000_0111,
            ],
        );

        fill_encoded(&mut iter, &[0; 7]);
        fill_encoded(&mut iter, &[data_length as u8]);

        let decoder = super::RgbDecoder::new(buffer, bits_per_channel);
        let decoded = decoder.decode();
        assert_eq!(decoded, Err("Not enough data to decode"));
    }

    #[test]
    fn decode_2bits() {
        let mut buffer = vec![0; 64];
        let mut iter = buffer.iter_mut();
        let bits_per_channel = 2;

        // Filename length
        fill_encoded(&mut iter, &[0; 14]);
        fill_encoded(&mut iter, &[0b0000_0001, 0b0000_0001]);

        // x = 0111 1000
        fill_encoded(
            &mut iter,
            &[0b0000_0001, 0b0000_0011, 0b0000_0010, 0b0000_0000],
        );
        // . = 0010 1110
        fill_encoded(
            &mut iter,
            &[0b0000_0000, 0b0000_0010, 0b0000_0011, 0b0000_0010],
        );
        // p = 0111 0000
        fill_encoded(
            &mut iter,
            &[0b0000_0001, 0b0000_0011, 0b0000_0000, 0b0000_0000],
        );
        // n = 0110 1110
        fill_encoded(
            &mut iter,
            &[0b0000_0001, 0b0000_0010, 0b0000_0011, 0b0000_0010],
        );
        // g = 0110 0111
        fill_encoded(
            &mut iter,
            &[0b0000_0001, 0b0000_0010, 0b0000_0001, 0b0000_0011],
        );

        // Message Length
        fill_encoded(&mut iter, &[0; 15]);
        fill_encoded(&mut iter, &[0b0000_0011]);

        // Data
        // x = 0111 1000
        fill_encoded(
            &mut iter,
            &[0b0000_0001, 0b0000_0011, 0b0000_0010, 0b0000_0000],
        );
        // y = 0111 1001
        fill_encoded(
            &mut iter,
            &[0b0000_0001, 0b0000_0011, 0b0000_0010, 0b0000_0001],
        );
        // z = 0111 1010
        fill_encoded(
            &mut iter,
            &[0b0000_0001, 0b0000_0011, 0b0000_0010, 0b0000_0010],
        );

        let decoder = super::RgbDecoder::new(buffer, bits_per_channel);
        let (filename, data) = decoder.decode().unwrap();

        assert_eq!(filename, "x.png");
        assert_eq!(String::from_utf8(data).unwrap(), "xyz");
    }

    #[test]
    fn decode_4bits() {
        let mut buffer = vec![0; 64];
        let mut iter = buffer.iter_mut();
        let bits_per_channel = 4;

        // Filename length
        fill_encoded(&mut iter, &[0; 7]);
        fill_encoded(&mut iter, &[0b0000_0101]);

        // x = 0111 1000
        fill_encoded(&mut iter, &[0b0000_0111, 0b0000_1000]);
        // . = 0010 1110
        fill_encoded(&mut iter, &[0b0000_0010, 0b0000_1110]);
        // p = 0111 0000
        fill_encoded(&mut iter, &[0b0000_0111, 0b0000_0000]);
        // n = 0110 1110
        fill_encoded(&mut iter, &[0b0000_0110, 0b0000_1110]);
        // g = 0110 0111
        fill_encoded(&mut iter, &[0b0000_0110, 0b0000_0111]);

        // Message Length
        fill_encoded(&mut iter, &[0; 7]);
        fill_encoded(&mut iter, &[0b0000_0100]);

        // Data
        // w = 0111 0111
        fill_encoded(&mut iter, &[0b0000_0111, 0b0000_0111]);
        // o = 0110 1111
        fill_encoded(&mut iter, &[0b0000_0110, 0b0000_1111]);
        // l = 0110 1100
        fill_encoded(&mut iter, &[0b0000_0110, 0b0000_1100]);
        // f = 0110 0110
        fill_encoded(&mut iter, &[0b0000_0110, 0b0000_0110]);

        let decoder = super::RgbDecoder::new(buffer, bits_per_channel);
        let (filename, data) = decoder.decode().unwrap();

        assert_eq!(filename, "x.png");
        assert_eq!(String::from_utf8(data).unwrap(), "wolf");
    }

    #[allow(dead_code)]
    fn fill_encoded(iter: &mut IterMut<u8>, bytes: &[u8]) {
        for &byte in bytes {
            *iter.next().unwrap() = byte;
        }
    }
}
