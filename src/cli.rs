use clap::builder::TypedValueParser;
use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct Config {
    /// Mode to run: Encode or Decode
    #[command(subcommand)]
    pub mode: Mode,
}

#[derive(Debug, Subcommand)]
pub enum Mode {
    Encode(EncodeConfig),
    Decode(DecodeConfig),
}

#[derive(Debug, Args)]
pub struct EncodeConfig {
    #[arg(short, long, help = "Path to image file to be used to hide data.")]
    pub image_file: std::path::PathBuf,

    #[arg(short, long, help = "Path to data file to be hidden.")]
    pub message_file: std::path::PathBuf,

    #[arg(
        short,
        long,
        value_name = "1/2/4/8",
        help = "Number of bits to be used per channel.",
        value_parser = clap::builder::PossibleValuesParser::new(["1", "2", "4", "8"]).map(|s| s.parse::<u8>().unwrap())
    )]
    pub bits_per_channel: u8,
}

#[derive(Debug, Args)]
pub struct DecodeConfig {
    #[arg(short, long, help = "Path to image file holding hidden data.")]
    pub image_file: std::path::PathBuf,

    #[arg(
        short,
        long,
        value_name = "1/2/4/8",
        help = "Number of bits to be used per channel.",
        value_parser = clap::builder::PossibleValuesParser::new(["1", "2", "4", "8"]).map(|s| s.parse::<u8>().unwrap())
    )]
    pub bits_per_channel: u8,
}

pub fn parse() -> Config {
    Config::parse()
}
