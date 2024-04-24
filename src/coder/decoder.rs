use image::DynamicImage;

use crate::cli::Algorithm;

use super::{alpha_decoder::AlphaDecoder, rgb_decoder::RgbDecoder};

pub fn decode(algorithm: &Algorithm, image: &DynamicImage) -> (String, Vec<u8>) {
    match algorithm {
        Algorithm::Rgb(rgb_alg_config) => {
            let decoder =
                RgbDecoder::new(image.to_rgb8().into_raw(), rgb_alg_config.bits_per_channel);
            decoder.decode()
        }
        Algorithm::Alpha => {
            let decoder = AlphaDecoder::new(image.to_rgba8().into_raw());
            decoder.decode()
        }
    }
}
