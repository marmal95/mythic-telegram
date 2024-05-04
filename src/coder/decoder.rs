mod alpha_decoder;
mod decode;
mod header_decoder;
mod rgb_decoder;

use std::error::Error;

use image::RgbaImage;

use self::{alpha_decoder::AlphaDecoder, decode::Decode, rgb_decoder::RgbDecoder};

use super::header::{AlgHeader, Header};

pub fn decode(image: RgbaImage) -> Result<(String, Vec<u8>), Box<dyn Error>> {
    let mut buffer = image.into_raw();
    let header = header_decoder::decode(&buffer)?;
    let mut buffer = buffer.split_off(header.size() * 4);

    let decoder = create_decoder(&header, &mut buffer);
    let decoded = decoder.decode()?;
    Ok(decoded)
}

fn create_decoder<'a>(header: &Header, buffer: &'a Vec<u8>) -> Box<dyn Decode + 'a> {
    match &header.alg_header {
        AlgHeader::Alpha(_) => Box::new(AlphaDecoder::new(buffer)),
        AlgHeader::Rgb(rgb_header) => {
            Box::new(RgbDecoder::new(buffer, rgb_header.bits_per_channel))
        }
    }
}
