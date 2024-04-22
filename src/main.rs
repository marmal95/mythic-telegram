use fictional_telegram::{
    cli::{self, DecodeConfig, EncodeConfig, Mode},
    coder::{decoder::Decoder, encoder::Encoder},
};
use image::{io::Reader as ImageReader, RgbImage};
use std::{
    fs::File,
    io::{Read, Write},
};

fn encode(config: &EncodeConfig) {
    let image_filename = config.image_file.to_str().unwrap();
    let message_filename = config.message_file.to_str().unwrap();

    let mut message_file = File::open(message_filename)
        .expect(format!("File not found: {}", message_filename).as_str());

    let mut data_buffer = Vec::new();
    message_file
        .read_to_end(&mut data_buffer)
        .expect(format!("Error reading file: {}", message_filename).as_str());

    let image = ImageReader::open(image_filename)
        .expect(format!("File not found: {}", image_filename).as_str())
        .decode()
        .unwrap();

    let image_data = image.to_rgb8().into_raw();
    let encoder = Encoder::new(
        image_data,
        data_buffer,
        config.bits_per_channel,
        message_filename.to_string(),
    );
    let encoded_data = encoder.encode();
    let encoded_image = RgbImage::from_vec(image.width(), image.height(), encoded_data).unwrap();

    encoded_image
        .save("encoded_".to_owned() + image_filename)
        .unwrap();
}

fn decode(config: &DecodeConfig) {
    let image_filename = config.image_file.to_str().unwrap();

    let image = ImageReader::open(image_filename)
        .expect(format!("File not found: {}", image_filename).as_str())
        .decode()
        .unwrap();

    let image_data = image.to_rgb8().into_raw();
    let decoder = Decoder::new(image_data, config.bits_per_channel);

    let (file_name, decoded_data) = decoder.decode();
    let mut data_file = File::create(file_name).unwrap();

    data_file.write_all(&decoded_data).unwrap();
}

fn main() {
    let cli = cli::parse();

    match cli.mode {
        Mode::Encode(encode_cfg) => {
            encode(&encode_cfg);
        }
        Mode::Decode(decode_cfg) => {
            decode(&decode_cfg);
        }
    }
}
