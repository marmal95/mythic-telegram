pub struct AlphaEncoder {
    buffer: Vec<u8>,
    data: Vec<u8>,
    index: usize,
    file_name: String,
}

impl AlphaEncoder {
    pub fn new(buffer: Vec<u8>, data: Vec<u8>, file_name: String) -> Self {
        AlphaEncoder {
            buffer,
            data,
            index: 3,
            file_name,
        }
    }

    pub fn encode(mut self) -> Result<Vec<u8>, &'static str> {
        if self.bytes_to_encode() > self.max_bytes_to_encode() {
            return Err("Too much data to encode in the image.");
        }

        self.encode_file_name();
        self.encode_content();

        Ok(self.buffer)
    }

    fn encode_file_name(&mut self) {
        self.encode_length(self.file_name.len() as u32);
        self.encode_data(self.file_name.clone().as_bytes().to_vec());
    }

    fn encode_content(&mut self) {
        self.encode_length(self.data.len() as u32);
        self.encode_data(self.data.clone());
    }

    fn encode_length(&mut self, length: u32) {
        length
            .to_be_bytes()
            .into_iter()
            .for_each(|byte| self.encode_byte(byte));
    }

    fn encode_data(&mut self, data: Vec<u8>) {
        data.iter().for_each(|byte| self.encode_byte(*byte));
    }

    fn encode_byte(&mut self, byte: u8) {
        let channel = self.next();
        *channel = byte;
    }

    fn max_bytes_to_encode(&self) -> usize {
        self.buffer.len() / 4
    }

    fn bytes_to_encode(&self) -> usize {
        self.data.len() + self.file_name.len() + 4 + 4
    }

    fn next(&mut self) -> &mut u8 {
        let byte = &mut self.buffer[self.index];
        self.index += 4;
        byte
    }
}

mod tests {
    use std::slice::Iter;

    #[test]
    fn not_enough_buffer() {
        let buffer = vec![0; 63];
        let data = "xyz".as_bytes();
        let file_name = "x.png";

        // Needed buffer length:
        //   4 (filename size)
        // + 5 (filename)
        // + 4 (message size)
        // + 3 (data)
        // = 16 (bytes) = 16 * 4 (only alpha channel use from rgba) = 64 needed channels/bytes

        let encoder = super::AlphaEncoder::new(buffer, data.to_vec(), file_name.to_string());
        assert_eq!(
            encoder.encode(),
            Err("Too much data to encode in the image.")
        )
    }

    #[test]
    fn encode() {
        let buffer = vec![0; 68];
        let data = "wolf".as_bytes();
        let file_name = "x.png";

        let encoder = super::AlphaEncoder::new(buffer, data.to_vec(), file_name.to_string());
        let encoded = encoder.encode().unwrap();
        let mut encoded_it = encoded.iter();

        // Encoded layout:
        // Filename length = 4 bytes (encoded on 32 bits) = 0b00000000'00000000'00000000'00000101
        // Filename = 01111000 00101110 01110000 01101110 01100111
        // Message length = 4 bytes (encoded on 32 bits) = 0b00000000'00000000'00000000'00000100
        // Message bytes [...]

        // Filename Length
        verify_encoded(
            &mut encoded_it,
            &[0b0000_0000, 0b0000_0000, 0b0000_0000, 0b00000101],
        );

        // Filename
        verify_encoded(
            &mut encoded_it,
            &[0b001111000, 0b00101110, 0b01110000, 0b01101110, 0b01100111],
        );

        // Message length
        verify_encoded(
            &mut encoded_it,
            &[0b0000_0000, 0b0000_0000, 0b0000_0000, 0b00000100],
        );

        // Message
        verify_encoded(
            &mut encoded_it,
            &[0b0111_0111, 0b0110_1111, 0b0110_1100, 0b0110_0110],
        );
    }

    #[allow(dead_code)]
    fn verify_encoded(iter: &mut Iter<u8>, bytes: &[u8]) {
        for &byte in bytes {
            (0..3).for_each(|_| assert_eq!(*iter.next().unwrap(), 0b0000_0000));
            assert_eq!(*iter.next().unwrap(), byte);
        }
    }
}
