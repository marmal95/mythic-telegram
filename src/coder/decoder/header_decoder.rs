use crate::coder::{
    error::HeaderDecodeError,
    header::{AlgHeader, AlphaHeader, Header, RgbHeader, ALPHA_MODE, RGB_MODE},
};

pub fn decode(buffer: &Vec<u8>) -> Result<Header, HeaderDecodeError> {
    let mut iter = buffer.iter().skip(3).step_by(4);
    let mode = *iter.next().ok_or(HeaderDecodeError(
        "Header decode error: Not enough data to decode mode.".to_string(),
    ))?;

    let alg_header = decode_alg_header(mode, &mut iter)?;
    Ok(Header::new(mode, alg_header))
}

fn decode_alg_header<'a, I>(mode: u8, iter: &mut I) -> Result<AlgHeader, HeaderDecodeError>
where
    I: Iterator<Item = &'a u8>,
{
    match mode {
        ALPHA_MODE => Ok(AlgHeader::Alpha(decode_alpha())),
        RGB_MODE => Ok(AlgHeader::Rgb(decode_rgb(iter)?)),
        _ => Err(HeaderDecodeError("Unknown mode in header.".to_string())),
    }
}

fn decode_alpha() -> AlphaHeader {
    AlphaHeader {}
}

fn decode_rgb<'a, I>(iter: &mut I) -> Result<RgbHeader, HeaderDecodeError>
where
    I: Iterator<Item = &'a u8>,
{
    let bits_per_channel = *iter.next().ok_or(HeaderDecodeError(
        "Not enough data to decode bits per channel.".to_string(),
    ))?;

    Ok(RgbHeader { bits_per_channel })
}

#[cfg(test)]
mod tests {
    use crate::coder::{
        error::HeaderDecodeError,
        header::{Header, ALPHA_MODE, RGB_MODE},
    };

    #[test]
    fn decode_alpha() {
        let mut buffer = vec![0; 4];
        let mut iter = buffer.iter_mut().skip(3).step_by(4);
        *iter.next().unwrap() = ALPHA_MODE;

        let decoded = super::decode(&buffer).unwrap();
        assert_eq!(decoded, Header::new_alpha());
    }

    #[test]
    fn decode_rgb() {
        let bits_per_channel = 2;
        let mut buffer = vec![0; 8];
        let mut iter = buffer.iter_mut().skip(3).step_by(4);
        *iter.next().unwrap() = RGB_MODE;
        *iter.next().unwrap() = bits_per_channel;

        let decoded = super::decode(&buffer).unwrap();
        assert_eq!(decoded, Header::new_rgb(bits_per_channel));
    }

    #[test]
    fn decode_error_missing_mode_data() {
        let buffer = Vec::new();
        let decoded = super::decode(&buffer);
        assert_eq!(
            decoded,
            Err(HeaderDecodeError(
                "Header decode error: Not enough data to decode mode.".to_string()
            ))
        );
    }

    #[test]
    fn decode_error_missing_bits_per_channel_data() {
        let buffer = vec![0, 0, 0, RGB_MODE];
        let decoded = super::decode(&buffer);
        assert_eq!(
            decoded,
            Err(HeaderDecodeError(
                "Header decode error: Not enough data to decode bits per channel.".to_string()
            ))
        );
    }

    #[test]
    fn decode_error_unknown_mode() {
        let unknown_mode = 4;
        let mut buffer = vec![0; 4];
        let mut iter = buffer.iter_mut().skip(3).step_by(4);
        *iter.next().unwrap() = unknown_mode;

        let decoded = super::decode(&buffer);
        assert_eq!(
            decoded,
            Err(HeaderDecodeError("Unknown mode in header.".to_string()))
        );
    }
}
