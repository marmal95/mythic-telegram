use crate::coder::header::AlgHeader;

use super::header::{Header, RgbHeader};

pub fn encode(header: Header, buffer: &mut Vec<u8>) {
    let mut iter = buffer.iter_mut().skip(3).step_by(4);
    *iter.next().unwrap() = header.mode;

    match header.alg_header {
        AlgHeader::Alpha(_) => encode_alpha(&mut iter),
        AlgHeader::Rgb(alg_header) => encode_rgb(&mut iter, &alg_header),
    }
}

fn encode_alpha<'a, I>(_iter: &mut I)
where
    I: Iterator<Item = &'a mut u8>,
{
}

fn encode_rgb<'a, I>(iter: &mut I, header: &RgbHeader)
where
    I: Iterator<Item = &'a mut u8>,
{
    *iter.next().unwrap() = header.bits_per_channel;
}

#[cfg(test)]
mod tests {

    use crate::coder::header::{Header, ALPHA_MODE, RGB_MODE};

    #[test]
    fn encode_alpha() {
        let header = Header::new_alpha();
        let mut buffer = vec![0; 10];
        super::encode(header, &mut buffer);
        assert_eq!(buffer, vec![0, 0, 0, ALPHA_MODE, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn encode_rgb() {
        let bits_per_channel = 4;
        let header = Header::new_rgb(bits_per_channel);
        let mut buffer = vec![0; 10];
        super::encode(header, &mut buffer);
        assert_eq!(
            buffer,
            vec![0, 0, 0, RGB_MODE, 0, 0, 0, bits_per_channel, 0, 0]
        );
    }
}
