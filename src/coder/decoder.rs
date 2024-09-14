mod alpha_decoder;
mod decode;
mod header_decoder;
mod rgb_decoder;

use anyhow::Result;

use self::{alpha_decoder::AlphaDecoder, decode::Decode, rgb_decoder::RgbDecoder};

use super::header::{AlgHeader, Header};

pub fn decode(mut image_buffer: Vec<u8>) -> Result<(String, Vec<u8>)> {
    let header = header_decoder::decode(&image_buffer)?;
    let buffer = image_buffer.split_off(header.size() * 4);

    let decoder = create_decoder(&header, &buffer);
    let decoded = decoder.decode()?;
    Ok(decoded)
}

fn create_decoder<'a>(header: &Header, buffer: &'a [u8]) -> Box<dyn Decode + 'a> {
    match &header.alg_header {
        AlgHeader::Alpha(_) => Box::new(AlphaDecoder::new(buffer)),
        AlgHeader::Rgb(rgb_header) => {
            Box::new(RgbDecoder::new(buffer, rgb_header.bits_per_channel))
        }
    }
}
