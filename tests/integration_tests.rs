#[cfg(test)]
mod tests {
    use fictional_telegram::{
        config::{Algorithm, RgbAlgorithmConfig},
        coder::{decoder, encoder},
    };
    use image::DynamicImage;

    #[test]
    fn encode_decode_rgb_1bit() {
        test_encode_decode(Algorithm::Rgb(RgbAlgorithmConfig {
            bits_per_channel: 1,
        }));
    }

    #[test]
    fn encode_decode_rgb_2bit() {
        test_encode_decode(Algorithm::Rgb(RgbAlgorithmConfig {
            bits_per_channel: 2,
        }));
    }

    #[test]
    fn encode_decode_rgb_4bit() {
        test_encode_decode(Algorithm::Rgb(RgbAlgorithmConfig {
            bits_per_channel: 4,
        }));
    }

    #[test]
    fn encode_decode_rgb_8bit() {
        test_encode_decode(Algorithm::Rgb(RgbAlgorithmConfig {
            bits_per_channel: 8,
        }));
    }

    #[test]
    fn encode_decode_alpha() {
        test_encode_decode(Algorithm::Alpha);
    }

    fn test_encode_decode(algorithm: Algorithm) {
        let secret_message = "The quick brown fox jumps over the lazy dog".as_bytes();
        let secret_filename = "secret.txt";

        let image = DynamicImage::new_rgb8(120, 120);

        let encoded_image = encoder::encode(
            &algorithm,
            &image,
            secret_message.to_vec(),
            secret_filename.to_string(),
        );

        let (decoded_filename, decoded_buffer) = decoder::decode(&algorithm, &encoded_image);

        assert_eq!(secret_filename, decoded_filename);
        assert_eq!(secret_message, decoded_buffer);
    }
}
