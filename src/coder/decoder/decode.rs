use anyhow::Result;

use crate::coder::error::DecodeError;

pub trait Decode {
    fn decode(mut self: Box<Self>) -> Result<(String, Vec<u8>)> {
        let file_name_length = self
            .decode_length()
            .ok_or(self.not_available("filename length"))?;

        let file_name: Vec<u8> = self
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

    fn decode_byte(&mut self) -> Option<u8>;

    fn not_available(&self, data_to_check: &str) -> DecodeError {
        DecodeError(format!("Not enough data to decode {data_to_check}"))
    }
}
