mod alpha_encoder;
mod encode;
mod header_encoder;
mod rgb_encoder;

use anyhow::{Ok, Result};

use crate::{coder::header::Header, config::Algorithm};

use self::{alpha_encoder::AlphaEncoder, encode::Encode, rgb_encoder::RgbEncoder};

pub fn encode(
    algorithm: &Algorithm,
    mut image_buffer: Vec<u8>,
    secret_data: Vec<u8>,
    secret_filename: String,
) -> Result<Vec<u8>> {
    let header: Header = create_header(algorithm);
    let (header_buffer, data_buffer) = image_buffer.split_at_mut(header.size() * 4);

    header_encoder::encode(header.clone(), header_buffer)?;
    create_encoder(algorithm, data_buffer, secret_data, secret_filename).encode()?;

    Ok(image_buffer)
}

fn create_header(algorithm: &Algorithm) -> Header {
    match algorithm {
        Algorithm::Alpha => Header::new_alpha(),
        Algorithm::Rgb(alg_config) => Header::new_rgb(alg_config.bits_per_channel),
    }
}

fn create_encoder<'a>(
    algorithm: &Algorithm,
    buffer: &'a mut [u8],
    secret_data: Vec<u8>,
    secret_filename: String,
) -> Box<dyn Encode + 'a> {
    match algorithm {
        Algorithm::Rgb(alg_config) => Box::new(RgbEncoder::new(
            buffer,
            secret_data,
            alg_config.bits_per_channel,
            secret_filename,
        )),
        Algorithm::Alpha => Box::new(AlphaEncoder::new(buffer, secret_data, secret_filename)),
    }
}
