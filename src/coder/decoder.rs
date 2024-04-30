mod alpha_decoder;
mod header_decoder;
mod rgb_decoder;

use std::error::Error;

use image::RgbaImage;

use self::{alpha_decoder::AlphaDecoder, rgb_decoder::RgbDecoder};

use super::{
    error::DecodeError,
    header::{AlgHeader, Header},
};

trait Decode {
    fn run(self: Box<Self>) -> Result<(String, Vec<u8>), DecodeError>;
}

impl Decode for AlphaDecoder {
    fn run(self: Box<Self>) -> Result<(String, Vec<u8>), DecodeError> {
        self.decode()
    }
}

impl Decode for RgbDecoder {
    fn run(self: Box<Self>) -> Result<(String, Vec<u8>), DecodeError> {
        self.decode()
    }
}

pub fn decode(image: RgbaImage) -> Result<(String, Vec<u8>), Box<dyn Error>> {
    let mut buffer = image.into_raw();
    let header = header_decoder::decode(&buffer)?;
    let buffer = buffer.split_off(header.size() * 4);

    let decoder = create_decoder(&header, buffer);
    let decoded = decoder.run()?;
    Ok(decoded)
}

fn create_decoder(header: &Header, buffer: Vec<u8>) -> Box<dyn Decode> {
    match &header.alg_header {
        AlgHeader::Alpha(_) => Box::new(AlphaDecoder::new(buffer)),
        AlgHeader::Rgb(rgb_header) => {
            Box::new(RgbDecoder::new(buffer, rgb_header.bits_per_channel))
        }
    }
}
