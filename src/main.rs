use fictional_telegram::{
    cli::{self, DecodeConfig, EncodeConfig, Mode},
    coder::{decoder, encoder},
};
use image::io::Reader as ImageReader;
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

    let encoded_filename = "encoded_".to_owned() + image_filename;
    let encoded_image = encoder::encode(
        &config.algorithm,
        &image,
        data_buffer,
        message_filename.to_string(),
    );

    encoded_image.save(encoded_filename.as_str()).unwrap();
}

fn decode(config: &DecodeConfig) {
    let image_filename = config.image_file.to_str().unwrap();

    let image = ImageReader::open(image_filename)
        .expect(format!("File not found: {}", image_filename).as_str())
        .decode()
        .unwrap();

    let (file_name, decoded_data) = decoder::decode(&config.algorithm, &image);

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
