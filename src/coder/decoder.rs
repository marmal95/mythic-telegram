use std::error::Error;

use image::DynamicImage;

use crate::config::Algorithm;

use super::{alpha_decoder::AlphaDecoder, rgb_decoder::RgbDecoder};

pub fn decode(
    algorithm: &Algorithm,
    image: &DynamicImage,
) -> Result<(String, Vec<u8>), Box<dyn Error>> {
    match algorithm {
        Algorithm::Rgb(rgb_alg_config) => {
            let decoder =
                RgbDecoder::new(image.to_rgb8().into_raw(), rgb_alg_config.bits_per_channel);
            let decoded = decoder.decode()?;
            Ok(decoded)
        }
        Algorithm::Alpha => {
            let decoder = AlphaDecoder::new(image.to_rgba8().into_raw());
            let decoded = decoder.decode()?;
            Ok(decoded)
        }
    }
}
