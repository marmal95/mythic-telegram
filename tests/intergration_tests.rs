#[cfg(test)]
mod tests {

    use fictional_telegram::coder::{decoder::Decoder, encoder::Encoder};

    #[test]
    fn encode_decode_1bit() {
        encode_decode(1);
    }

    #[test]
    fn encode_decode_2bit() {
        encode_decode(2);
    }

    #[test]
    fn encode_decode_4bit() {
        encode_decode(4);
    }

    #[test]
    fn encode_decode_8bit() {
        encode_decode(8);
    }

    fn encode_decode(bits_per_channel: u8) {
        let secret_message = "The quick brown fox jumps over the lazy dog".as_bytes();
        let secret_fileame = "secret.txt";
        let buffer = vec![0; 1024];

        let encoder = Encoder::new(
            buffer,
            secret_message.to_vec(),
            bits_per_channel,
            secret_fileame.to_string(),
        );
        let encoded_data = encoder.encode();

        let decoder = Decoder::new(encoded_data, bits_per_channel);
        let (decoded_filename, decoded_buffer) = decoder.decode();

        assert_eq!(secret_fileame, decoded_filename);
        assert_eq!(secret_message, decoded_buffer);
    }
}
