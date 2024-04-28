use fictional_telegram::{
    coder::{decoder, encoder},
    config::{self, Config, DecodeConfig, EncodeConfig, Mode},
};
use image::io::Reader as ImageReader;
use std::{
    error::Error,
    fs::File,
    io::{Read, Write},
};

fn encode(config: &EncodeConfig) -> Result<(), Box<dyn Error>> {
    let image_filename = config.image_file.to_str().unwrap();
    let message_filename = config.message_file.to_str().unwrap();

    let mut message_file = File::open(message_filename)?;

    let mut data_buffer = Vec::new();
    message_file.read_to_end(&mut data_buffer)?;

    let image = ImageReader::open(image_filename)?.decode()?;
    let encoded_filename = "encoded_".to_owned() + image_filename;

    let encoded_image = encoder::encode(
        &config.algorithm,
        &image,
        data_buffer,
        message_filename.to_string(),
    )?;

    encoded_image.save(encoded_filename.as_str())?;
    Ok(())
}

fn decode(config: &DecodeConfig) -> Result<(), Box<dyn Error>> {
    let image_filename = config.image_file.to_str().unwrap();
    let image = ImageReader::open(image_filename)?.decode()?;

    let (file_name, decoded_data) = decoder::decode(&config.algorithm, &image)?;

    let mut data_file = File::create(file_name)?;
    data_file.write_all(&decoded_data)?;
    Ok(())
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    match config.mode {
        Mode::Encode(encode_cfg) => encode(&encode_cfg)?,
        Mode::Decode(decode_cfg) => decode(&decode_cfg)?,
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    run(config::parse())
}
