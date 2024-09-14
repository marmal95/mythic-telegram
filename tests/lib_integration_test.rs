#[cfg(test)]
mod tests {
    use image::RgbaImage;
    use mythic_telegram::{
        coder::{decoder, encoder},
        config::{Algorithm, RgbAlgorithmConfig},
    };

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

        let image_buffer = RgbaImage::new(120, 120).into_vec();

        let encoded_data = encoder::encode(
            &algorithm,
            image_buffer,
            secret_message.to_vec(),
            secret_filename.to_string(),
        )
        .unwrap();

        let (decoded_filename, decoded_buffer) = decoder::decode(encoded_data).unwrap();
        assert_eq!(secret_filename, decoded_filename);
        assert_eq!(secret_message, decoded_buffer);
    }
}
