use anyhow::Result;
use image::{io::Reader as ImageReader, GenericImageView, RgbaImage};
use mythic_telegram::{
    coder,
    config::{self, Config, DecodeConfig, EncodeConfig, Mode},
    file,
};
use std::path::Path;

fn encode(config: &EncodeConfig) -> Result<()> {
    let image_path = Path::new(&config.image_file);
    let secret_file_path = Path::new(&config.secret_file);

    let image = ImageReader::open(image_path)?.decode()?;
    let (image_width, image_height) = image.dimensions();

    let encoded_data = coder::encoder::encode(
        &config.algorithm,
        image.to_rgba8().into_vec(),
        file::read_bytes(&config.secret_file)?,
        file::extract_file_name(secret_file_path)?,
    )?;
    let encoded_image = RgbaImage::from_vec(image_width, image_height, encoded_data).unwrap();

    let image_filename = file::extract_file_name(image_path)?;
    encoded_image.save(image_path.with_file_name(format!("encoded_{}", image_filename)))?;
    Ok(())
}

fn decode(config: &DecodeConfig) -> Result<()> {
    let image_path = Path::new(&config.image_file);
    let image = ImageReader::open(image_path)?.decode()?;
    let image_data = image.to_rgba8().into_vec();

    let (file_name, decoded_data) = coder::decoder::decode(image_data)?;
    let secret_file_path = image_path.with_file_name(file_name);
    file::write_bytes(&secret_file_path, &decoded_data)
}

fn run(config: Config) -> Result<()> {
    match config.mode {
        Mode::Encode(encode_cfg) => encode(&encode_cfg)?,
        Mode::Decode(decode_cfg) => decode(&decode_cfg)?,
    }
    Ok(())
}

fn main() -> Result<()> {
    run(config::parse())
}
