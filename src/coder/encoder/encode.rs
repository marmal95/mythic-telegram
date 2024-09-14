use anyhow::{anyhow, Result};

use crate::coder::error::EncodeError;

pub trait Encode {
    fn encode(mut self: Box<Self>) -> Result<()> {
        self.validate()?;

        let file_name = self.file_name_bytes();
        self.encode_length(file_name.len() as u32);
        self.encode_data(file_name);

        let data = self.data_bytes();
        self.encode_length(data.len() as u32);
        self.encode_data(data);

        Ok(())
    }

    fn validate(&self) -> Result<()> {
        if self.bytes_to_encode() > self.max_bytes_to_encode() {
            return Err(anyhow!(EncodeError(
                "Too much data to encode in the image.".to_string(),
            )));
        }
        Ok(())
    }
    fn encode_length(&mut self, length: u32) {
        length
            .to_be_bytes()
            .into_iter()
            .for_each(|byte| self.encode_byte(byte));
    }
    fn encode_data(&mut self, data: Vec<u8>) {
        data.iter().for_each(|byte| self.encode_byte(*byte));
    }
    fn encode_byte(&mut self, byte: u8);
    fn max_bytes_to_encode(&self) -> usize;
    fn bytes_to_encode(&self) -> usize;
    fn file_name_bytes(&self) -> Vec<u8>;
    fn data_bytes(&self) -> Vec<u8>;
}
