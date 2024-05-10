use std::slice::Iter;

use crate::coder::util::{create_mask, BITS_IN_BYTE};

use super::decode::Decode;

pub struct RgbDecoder<'a> {
    buffer: Iter<'a, u8>,
    bits_per_channel: u8,
    mask: u8,
}

impl<'a> RgbDecoder<'a> {
    pub fn new(buffer: &'a [u8], bits_per_channel: u8) -> Self {
        RgbDecoder {
            buffer: buffer.iter(),
            bits_per_channel,
            mask: create_mask(bits_per_channel),
        }
    }
}

impl<'a> Decode for RgbDecoder<'a> {
    fn decode_byte(&mut self) -> Option<u8> {
        let mut byte: u8 = 0;
        let mut left = BITS_IN_BYTE;

        while left > 0 {
            let channel = *self.buffer.next()?;
            let bits = channel & self.mask;
            byte = byte.checked_shl(self.bits_per_channel as u32).unwrap_or(0);
            byte |= bits;

            left -= self.bits_per_channel;
        }

        Some(byte)
    }
}

#[cfg(test)]
mod tests {
    use std::slice::IterMut;

    use crate::coder::{decoder::decode::Decode, error::DecodeError};

    #[test]
    fn not_enough_data_to_decode_filename_length() {
        let bits_per_channel: u8 = 4;
        let buffer = vec![0; 1];

        let decoder = create_decoder(&buffer, bits_per_channel);
        let decoded = decoder.decode();

        assert_eq!(
            decoded,
            Err(DecodeError(
                "Not enough data to decode filename length".to_string()
            ))
        );
    }

    #[test]
    fn not_enough_data_to_decode_filename() {
        let filename_length = 10;
        let data_length = 0;
        let bits_per_channel: u8 = 4;

        let mut buffer = vec![
            0;
            min_required_data(filename_length, data_length, bits_per_channel)
                - filename_length
        ];
        let mut iter = buffer.iter_mut();

        fill_encoded(&mut iter, &[0; 7]);
        fill_encoded(&mut iter, &[filename_length as u8]);

        let decoder = create_decoder(&buffer, bits_per_channel);
        let decoded = decoder.decode();
        assert_eq!(
            decoded,
            Err(DecodeError(
                "Not enough data to decode filename".to_string()
            ))
        );
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

        let decoder = create_decoder(&buffer, bits_per_channel);
        let decoded = decoder.decode();
        assert_eq!(
            decoded,
            Err(DecodeError(
                "Not enough data to decode data length".to_string()
            ))
        );
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

        let decoder = create_decoder(&buffer, bits_per_channel);
        let decoded = decoder.decode();
        assert_eq!(
            decoded,
            Err(DecodeError("Not enough data to decode data".to_string()))
        );
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

        let decoder = create_decoder(&buffer, bits_per_channel);
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

        let decoder = create_decoder(&buffer, bits_per_channel);
        let (filename, data) = decoder.decode().unwrap();

        assert_eq!(filename, "x.png");
        assert_eq!(String::from_utf8(data).unwrap(), "wolf");
    }

    fn create_decoder<'a>(buffer: &'a Vec<u8>, bits_per_channel: u8) -> Box<dyn Decode + 'a> {
        Box::new(super::RgbDecoder::new(buffer, bits_per_channel))
    }

    fn fill_encoded(iter: &mut IterMut<u8>, bytes: &[u8]) {
        for &byte in bytes {
            *iter.next().unwrap() = byte;
        }
    }

    fn min_required_data(
        filename_length: usize,
        data_length: usize,
        bits_per_channel: u8,
    ) -> usize {
        (4 + 4 + filename_length + data_length) * super::BITS_IN_BYTE as usize
            / bits_per_channel as usize
    }
}
