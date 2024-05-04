use image::io::Reader as ImageReader;
use mystic_telegram::{
    coder,
    config::{self, Config, DecodeConfig, EncodeConfig, Mode},
};
use std::{
    error::Error,
    fs::File,
    io::{Read, Write},
};

fn encode(config: &EncodeConfig) -> Result<(), Box<dyn Error>> {
    let image_filename = config.image_file.to_str().unwrap();
    let secret_filename = config.secret_file.to_str().unwrap();

    let mut secret_file = File::open(secret_filename)?;

    let mut data_buffer = Vec::new();
    secret_file.read_to_end(&mut data_buffer)?;

    let image = ImageReader::open(image_filename)?.decode()?;
    let encoded_filename = "encoded_".to_owned() + image_filename;

    let encoded_image = coder::encoder::encode(
        &config.algorithm,
        image.to_rgba8(),
        data_buffer,
        secret_filename.to_string(),
    )?;

    encoded_image.save(encoded_filename.as_str())?;
    Ok(())
}

fn decode(config: &DecodeConfig) -> Result<(), Box<dyn Error>> {
    let image_filename = config.image_file.to_str().unwrap();
    let image = ImageReader::open(image_filename)?.decode()?;

    let (file_name, decoded_data) = coder::decoder::decode(image.to_rgba8())?;

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
