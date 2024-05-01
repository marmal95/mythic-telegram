mod alpha_encoder;
mod header_encoder;
mod rgb_encoder;

use std::error::Error;

use image::RgbaImage;

use crate::{
    coder::{error::EncodeError, header::Header},
    config::Algorithm,
};

use self::{alpha_encoder::AlphaEncoder, rgb_encoder::RgbEncoder};

pub fn encode(
    algorithm: &Algorithm,
    image: RgbaImage,
    data: Vec<u8>,
    secret_filename: String,
) -> Result<RgbaImage, Box<dyn Error>> {
    let (width, height) = image.dimensions();
    let mut image_data = image.into_raw();

    let header = create_header(algorithm);
    let mut data_buffer = image_data.split_off(header.size() * 4);
    header_encoder::encode(header.clone(), &mut image_data)?;

    let encoder = create_encoder(algorithm, &mut data_buffer, data, secret_filename);
    encoder.run()?;

    image_data.append(&mut data_buffer);
    Ok(RgbaImage::from_vec(width, height, image_data).unwrap())
}

trait Encode {
    fn run(self: Box<Self>) -> Result<(), EncodeError>;
}

impl<'a> Encode for AlphaEncoder<'a> {
    fn run(self: Box<Self>) -> Result<(), EncodeError> {
        self.encode()
    }
}

impl<'a> Encode for RgbEncoder<'a> {
    fn run(self: Box<Self>) -> Result<(), EncodeError> {
        self.encode()
    }
}

fn create_header(algorithm: &Algorithm) -> Header {
    match algorithm {
        Algorithm::Alpha => Header::new_alpha(),
        Algorithm::Rgb(alg_config) => Header::new_rgb(alg_config.bits_per_channel),
    }
}

fn create_encoder<'a>(
    algorithm: &Algorithm,
    buffer: &'a mut Vec<u8>,
    data: Vec<u8>,
    secret_filename: String,
) -> Box<dyn Encode + 'a> {
    match algorithm {
        Algorithm::Rgb(alg_config) => Box::new(RgbEncoder::new(
            buffer,
            data,
            alg_config.bits_per_channel,
            secret_filename,
        )),
        Algorithm::Alpha => Box::new(AlphaEncoder::new(buffer, data, secret_filename)),
    }
}
