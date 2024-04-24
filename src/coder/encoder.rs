use image::{DynamicImage, RgbImage, RgbaImage};

use crate::config::{Algorithm, RgbAlgorithmConfig};

use super::{alpha_encoder::AlphaEncoder, rgb_encoder::RgbEncoder};

pub fn encode(
    algorithm: &Algorithm,
    image: &DynamicImage,
    data: Vec<u8>,
    message_filename: String,
) -> DynamicImage {
    match algorithm {
        Algorithm::Rgb(rgb_alg_config) => {
            DynamicImage::ImageRgb8(encode_rgb(&rgb_alg_config, &image, data, message_filename))
        }
        Algorithm::Alpha => DynamicImage::ImageRgba8(encode_alpha(&image, data, message_filename)),
    }
}

fn encode_rgb(
    config: &RgbAlgorithmConfig,
    image: &DynamicImage,
    buffer: Vec<u8>,
    message_filename: String,
) -> RgbImage {
    let image_data = image.to_rgb8().into_raw();
    let encoder = RgbEncoder::new(
        image_data,
        buffer,
        config.bits_per_channel,
        message_filename,
    );
    RgbImage::from_vec(image.width(), image.height(), encoder.encode()).unwrap()
}

fn encode_alpha(image: &DynamicImage, buffer: Vec<u8>, message_filename: String) -> RgbaImage {
    let image_data = image.to_rgba8().into_raw();
    let encoder = AlphaEncoder::new(image_data, buffer, message_filename);
    RgbaImage::from_vec(image.width(), image.height(), encoder.encode()).unwrap()
}
