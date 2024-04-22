use crate::coder::util::create_mask;

use super::util::BITS_IN_BYTE;

pub struct Encoder {
    buffer: Vec<u8>,
    data: Vec<u8>,
    index: usize,
    bits_per_channel: u8,
    file_name: String,
}

impl Encoder {
    pub fn new(buffer: Vec<u8>, data: Vec<u8>, bits_per_channel: u8, file_name: String) -> Self {
        Encoder {
            buffer,
            data,
            index: 0,
            bits_per_channel,
            file_name,
        }
    }

    pub fn encode(mut self) -> Vec<u8> {
        if self.bytes_to_encode() > self.max_bytes_to_encode() {
            panic!("Too much data to encode in the image.")
        }

        self.encode_file_name();
        self.encode_content();

        self.buffer
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
        let mask = create_mask(self.bits_per_channel);
        let bits = self.bits_per_channel;

        let mut shift = (BITS_IN_BYTE - bits) as i32;

        while shift >= 0 {
            let bits = (byte >> shift) & mask;
            let channel = self.next();
            *channel = (*channel & !mask) | bits;
            shift = shift - (self.bits_per_channel as i32);
        }
    }

    fn max_bytes_to_encode(&self) -> usize {
        (self.buffer.len() * self.bits_per_channel as usize) / 8
    }

    fn bytes_to_encode(&self) -> usize {
        self.data.len() + self.file_name.len() + 4 + 4
    }

    fn next(&mut self) -> &mut u8 {
        let byte = &mut self.buffer[self.index];
        self.index += 1;
        byte
    }
}

mod tests {
    #[test]
    #[should_panic]
    fn not_enough_buffer() {
        let buffer = vec![0; 63];
        let data = "xyz".as_bytes();
        let bits_per_channel = 2;
        let file_name = "x.png";

        // Needed buffer length:
        //   4 (filename size)
        // + 5 (filename)
        // + 4 (message size)
        // + 3 (data)
        // = 16 (bytes) = 128 (bites) / 2 (bits_per_channel) = 64 needed bytes

        let encoder = super::Encoder::new(
            buffer,
            data.to_vec(),
            bits_per_channel,
            file_name.to_string(),
        );
        encoder.encode();
    }

    #[test]
    fn encode_2bits() {
        let buffer = vec![0; 64];
        let data = "xyz".as_bytes();
        let bits_per_channel = 2;
        let file_name = "x.png";

        let encoder = super::Encoder::new(
            buffer,
            data.to_vec(),
            bits_per_channel,
            file_name.to_string(),
        );
        let encoded = encoder.encode();
        let mut encoded_it = encoded.iter();

        // Encoded layout:
        // Filename length = 4 bytes (encoded on 32 bits) = 0b00000000'00000000'00000000'00000101
        // Filename = 01111000 00101110 01110000 01101110 01100111
        // Message length = 4 bytes (encoded on 32 bits) = 0b00000000'00000000'00000000'00000011
        // Message bytes [...]

        // Using 2 bits per channel.
        // Thus first 14 channels are = 0

        for _ in 0..14 {
            assert_eq!(*encoded_it.next().unwrap(), 0b0000_0000);
        }

        // Filename length
        assert_eq!(*encoded_it.next().unwrap(), 0b0000_0001);
        assert_eq!(*encoded_it.next().unwrap(), 0b0000_0001);

        // x = 0111 1000
        assert_eq!(*encoded_it.next().unwrap(), 0b0000_0001);
        assert_eq!(*encoded_it.next().unwrap(), 0b0000_0011);
        assert_eq!(*encoded_it.next().unwrap(), 0b0000_0010);
        assert_eq!(*encoded_it.next().unwrap(), 0b0000_0000);

        // . = 0010 1110
        assert_eq!(*encoded_it.next().unwrap(), 0b0000_0000);
        assert_eq!(*encoded_it.next().unwrap(), 0b0000_0010);
        assert_eq!(*encoded_it.next().unwrap(), 0b0000_0011);
        assert_eq!(*encoded_it.next().unwrap(), 0b0000_0010);

        // p = 0111 0000
        assert_eq!(*encoded_it.next().unwrap(), 0b0000_0001);
        assert_eq!(*encoded_it.next().unwrap(), 0b0000_0011);
        assert_eq!(*encoded_it.next().unwrap(), 0b0000_0000);
        assert_eq!(*encoded_it.next().unwrap(), 0b0000_0000);

        // n = 0110 1110
        assert_eq!(*encoded_it.next().unwrap(), 0b0000_0001);
        assert_eq!(*encoded_it.next().unwrap(), 0b0000_0010);
        assert_eq!(*encoded_it.next().unwrap(), 0b0000_0011);
        assert_eq!(*encoded_it.next().unwrap(), 0b0000_0010);

        // g = 0110 0111
        assert_eq!(*encoded_it.next().unwrap(), 0b0000_0001);
        assert_eq!(*encoded_it.next().unwrap(), 0b0000_0010);
        assert_eq!(*encoded_it.next().unwrap(), 0b0000_0001);
        assert_eq!(*encoded_it.next().unwrap(), 0b0000_0011);

        // Message Length
        for _ in 0..15 {
            assert_eq!(*encoded_it.next().unwrap(), 0b0000_0000);
        }
        assert_eq!(*encoded_it.next().unwrap(), 0b0000_0011);

        // Data
        // x = 0111 1000
        assert_eq!(*encoded_it.next().unwrap(), 0b0000_0001);
        assert_eq!(*encoded_it.next().unwrap(), 0b0000_0011);
        assert_eq!(*encoded_it.next().unwrap(), 0b0000_0010);
        assert_eq!(*encoded_it.next().unwrap(), 0b0000_0000);

        // y = 0111 1001
        assert_eq!(*encoded_it.next().unwrap(), 0b0000_0001);
        assert_eq!(*encoded_it.next().unwrap(), 0b0000_0011);
        assert_eq!(*encoded_it.next().unwrap(), 0b0000_0010);
        assert_eq!(*encoded_it.next().unwrap(), 0b0000_0001);

        // z = 0111 1010
        assert_eq!(*encoded_it.next().unwrap(), 0b0000_0001);
        assert_eq!(*encoded_it.next().unwrap(), 0b0000_0011);
        assert_eq!(*encoded_it.next().unwrap(), 0b0000_0010);
        assert_eq!(*encoded_it.next().unwrap(), 0b0000_0010);
    }

    #[test]
    fn encode_4bits() {
        let buffer = vec![0; 64];
        let data = "wolf".as_bytes();
        let bits_per_channel = 4;
        let file_name = "x.png";

        let encoder = super::Encoder::new(
            buffer,
            data.to_vec(),
            bits_per_channel,
            file_name.to_string(),
        );
        let encoded = encoder.encode();
        let mut encoded_it = encoded.iter();

        // Encoded layout:
        // Filename length = 4 bytes (encoded on 32 bits) = 0b00000000'00000000'00000000'00000101
        // Filename = 01111000 00101110 01110000 01101110 01100111
        // Message length = 4 bytes (encoded on 32 bits) = 0b00000000'00000000'00000000'00000100
        // Message bytes [...]

        // Using 4 bits per channel.
        // Thus first 7 channels are = 0
        for _ in 0..7 {
            assert_eq!(*encoded_it.next().unwrap(), 0b0000_0000);
        }

        // Filename length
        assert_eq!(*encoded_it.next().unwrap(), 0b0000_0101);

        // x = 0111 1000
        assert_eq!(*encoded_it.next().unwrap(), 0b0000_0111);
        assert_eq!(*encoded_it.next().unwrap(), 0b0000_1000);

        // . = 0010 1110
        assert_eq!(*encoded_it.next().unwrap(), 0b0000_0010);
        assert_eq!(*encoded_it.next().unwrap(), 0b0000_1110);

        // p = 0111 0000
        assert_eq!(*encoded_it.next().unwrap(), 0b0000_0111);
        assert_eq!(*encoded_it.next().unwrap(), 0b0000_0000);

        // n = 0110 1110
        assert_eq!(*encoded_it.next().unwrap(), 0b0000_0110);
        assert_eq!(*encoded_it.next().unwrap(), 0b0000_1110);

        // g = 0110 0111
        assert_eq!(*encoded_it.next().unwrap(), 0b0000_0110);
        assert_eq!(*encoded_it.next().unwrap(), 0b0000_0111);

        // Message Length
        for _ in 0..7 {
            assert_eq!(*encoded_it.next().unwrap(), 0b0000_0000);
        }
        assert_eq!(*encoded_it.next().unwrap(), 0b0000_0100);

        // Data
        // w = 0111 0111
        assert_eq!(*encoded_it.next().unwrap(), 0b0000_0111);
        assert_eq!(*encoded_it.next().unwrap(), 0b0000_0111);

        // o = 0110 1111
        assert_eq!(*encoded_it.next().unwrap(), 0b0000_0110);
        assert_eq!(*encoded_it.next().unwrap(), 0b0000_1111);

        // l = 0110 1100
        assert_eq!(*encoded_it.next().unwrap(), 0b0000_0110);
        assert_eq!(*encoded_it.next().unwrap(), 0b0000_1100);

        // f = 0110 0110
        assert_eq!(*encoded_it.next().unwrap(), 0b0000_0110);
        assert_eq!(*encoded_it.next().unwrap(), 0b0000_0110);
    }
}
