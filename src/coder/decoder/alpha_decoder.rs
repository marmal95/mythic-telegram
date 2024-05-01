use std::{
    iter::{Skip, StepBy},
    slice::Iter,
};

use crate::coder::error::DecodeError;

pub struct AlphaDecoder<'a> {
    iter: StepBy<Skip<Iter<'a, u8>>>,
}

impl<'a> AlphaDecoder<'a> {
    pub fn new(buffer: &'a Vec<u8>) -> Self {
        Self {
            iter: buffer.iter().skip(3).step_by(4),
        }
    }

    pub fn decode(mut self) -> Result<(String, Vec<u8>), DecodeError> {
        let file_name_length = self
            .decode_length()
            .ok_or(self.not_available("filename length"))?;

        let file_name = self
            .decode_data(file_name_length)
            .ok_or(self.not_available("filename"))?;
        let file_name = String::from_utf8(file_name)?;

        let data_length = self
            .decode_length()
            .ok_or(self.not_available("data length"))?;

        let data = self
            .decode_data(data_length)
            .ok_or(self.not_available("data"))?;

        Ok((file_name, data))
    }

    fn decode_length(&mut self) -> Option<usize> {
        Some(u32::from_be_bytes([
            self.decode_byte()?,
            self.decode_byte()?,
            self.decode_byte()?,
            self.decode_byte()?,
        ]) as usize)
    }

    fn decode_data(&mut self, length: usize) -> Option<Vec<u8>> {
        (0..length).map(|_| self.decode_byte()).collect()
    }

    fn decode_byte(&mut self) -> Option<u8> {
        let byte = self.iter.next()?;
        Some(*byte)
    }

    fn not_available(&self, data_to_check: &str) -> DecodeError {
        DecodeError(format!("Not enough data to decode {data_to_check}"))
    }
}

#[cfg(test)]
mod tests {
    use crate::coder::error::DecodeError;

    #[test]
    fn not_enough_data_to_decode_filename_length() {
        let buffer = vec![0; 1];
        let decoder = super::AlphaDecoder::new(&buffer);
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
        let filename_length = 12;
        let data_length = 0;
        let mut buffer =
            vec![0; min_required_data(filename_length, data_length) - filename_length * 4];
        dbg!(min_required_data(filename_length, data_length));
        let mut iter = buffer.iter_mut().skip(3).step_by(4);

        fill_encoded(
            &mut iter,
            &[0b0000_0000, 0b0000_0000, 0b0000_0000, filename_length as u8],
        );

        let decoder = super::AlphaDecoder::new(&buffer);
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
        let data_length = 0;
        let mut buffer = vec![0; min_required_data(filename_length, data_length) - 1];
        let mut iter = buffer.iter_mut().skip(3).step_by(4);

        fill_encoded(
            &mut iter,
            &[0b0000_0000, 0b0000_0000, 0b0000_0000, filename_length as u8],
        );

        let decoder = super::AlphaDecoder::new(&buffer);
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
        let filename_length = 12;
        let data_length = 8;
        let mut buffer = vec![0; min_required_data(filename_length, data_length) - 1];
        let mut iter = buffer.iter_mut().skip(3).step_by(4);

        fill_encoded(
            &mut iter,
            &[
                // Filename Length
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                filename_length as u8,
                // Filename
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                // Data length
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                data_length as u8,
            ],
        );

        let decoder = super::AlphaDecoder::new(&buffer);
        let decoded = decoder.decode();
        assert_eq!(
            decoded,
            Err(DecodeError("Not enough data to decode data".to_string()))
        );
    }

    #[test]
    fn decode() {
        let data_length = 4;
        let filename_length = 5;
        let mut buffer = vec![0; min_required_data(filename_length, data_length)];
        let mut iter = buffer.iter_mut().skip(3).step_by(4);

        // Filename length
        fill_encoded(
            &mut iter,
            &[0b0000_0000, 0b0000_0000, 0b0000_0000, filename_length as u8],
        );

        // Filename
        fill_encoded(
            &mut iter,
            &[0b001111000, 0b00101110, 0b01110000, 0b01101110, 0b01100111],
        );

        // Message length
        fill_encoded(
            &mut iter,
            &[0b0000_0000, 0b0000_0000, 0b0000_0000, data_length as u8],
        );
        // Message
        fill_encoded(
            &mut iter,
            &[0b0111_0111, 0b0110_1111, 0b0110_1100, 0b0110_0110],
        );

        let decoder = super::AlphaDecoder::new(&buffer);
        let (filename, data) = decoder.decode().unwrap();

        assert_eq!(filename, "x.png");
        assert_eq!(String::from_utf8(data).unwrap(), "wolf");
    }

    fn fill_encoded<'a>(iter: &mut impl Iterator<Item = &'a mut u8>, bytes: &[u8]) {
        for &byte in bytes {
            *iter.next().unwrap() = byte;
        }
    }

    fn min_required_data(filename_length: usize, data_length: usize) -> usize {
        (4 + filename_length + 4 + data_length) * 4
    }
}
