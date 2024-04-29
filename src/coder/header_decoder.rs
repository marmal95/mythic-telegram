use super::header::{AlgHeader, AlphaHeader, Header, RgbHeader, ALPHA_MODE, RGB_MODE};

pub fn decode(buffer: &Vec<u8>) -> Header {
    let mut iter = buffer.iter().skip(3).step_by(4);
    let mode = *iter.next().unwrap();

    let alg_header = decode_alg_header(mode, &mut iter);
    Header::new(mode, alg_header)
}

fn decode_alg_header<'a, I>(mode: u8, iter: &mut I) -> AlgHeader
where
    I: Iterator<Item = &'a u8>,
{
    match mode {
        ALPHA_MODE => AlgHeader::Alpha(decode_alpha()),
        RGB_MODE => AlgHeader::Rgb(decode_rgb(iter)),
        _ => panic!("Unknown mode. Should never happen."),
    }
}

fn decode_alpha() -> AlphaHeader {
    AlphaHeader {}
}

fn decode_rgb<'a, I>(iter: &mut I) -> RgbHeader
where
    I: Iterator<Item = &'a u8>,
{
    RgbHeader {
        bits_per_channel: *iter.next().unwrap(),
    }
}

#[cfg(test)]
mod tests {

    use crate::coder::header::{Header, ALPHA_MODE, RGB_MODE};

    #[test]
    fn decode_alpha() {
        let mut buffer = vec![0; 4];
        let mut iter = buffer.iter_mut().skip(3).step_by(4);
        *iter.next().unwrap() = ALPHA_MODE;

        let decoded = super::decode(&buffer);
        assert_eq!(decoded, Header::new_alpha());
    }

    #[test]
    fn decode_rgb() {
        let bits_per_channel = 2;
        let mut buffer = vec![0; 8];
        let mut iter = buffer.iter_mut().skip(3).step_by(4);
        *iter.next().unwrap() = RGB_MODE;
        *iter.next().unwrap() = bits_per_channel;

        let decoded = super::decode(&buffer);
        assert_eq!(decoded, Header::new_rgb(bits_per_channel));
    }
}
