use std::error::Error;

use image::{DynamicImage, RgbImage, RgbaImage};

use crate::config::{Algorithm, RgbAlgorithmConfig};

use super::{alpha_encoder::AlphaEncoder, rgb_encoder::RgbEncoder};

pub fn encode(
    algorithm: &Algorithm,
    image: &DynamicImage,
    data: Vec<u8>,
    message_filename: String,
) -> Result<DynamicImage, Box<dyn Error>> {
    match algorithm {
        Algorithm::Rgb(rgb_alg_config) => {
            let encoded = encode_rgb(&rgb_alg_config, &image, data, message_filename)?;
            Ok(DynamicImage::ImageRgb8(encoded))
        }
        Algorithm::Alpha => {
            let encoded = encode_alpha(&image, data, message_filename)?;
            Ok(DynamicImage::ImageRgba8(encoded))
        }
    }
}

fn encode_rgb(
    config: &RgbAlgorithmConfig,
    image: &DynamicImage,
    buffer: Vec<u8>,
    message_filename: String,
) -> Result<RgbImage, Box<dyn Error>> {
    let encoder = RgbEncoder::new(
        image.to_rgb8().into_raw(),
        buffer,
        config.bits_per_channel,
        message_filename,
    );
    let encoded = encoder.encode()?;
    Ok(RgbImage::from_vec(image.width(), image.height(), encoded).unwrap())
}

fn encode_alpha(
    image: &DynamicImage,
    buffer: Vec<u8>,
    message_filename: String,
) -> Result<RgbaImage, Box<dyn Error>> {
    let encoder = AlphaEncoder::new(image.to_rgba8().into_raw(), buffer, message_filename);
    let encoded = encoder.encode()?;
    Ok(RgbaImage::from_vec(image.width(), image.height(), encoded).unwrap())
}
