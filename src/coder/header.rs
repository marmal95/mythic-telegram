use std::mem::size_of_val;

pub const ALPHA_MODE: u8 = 1;
pub const RGB_MODE: u8 = 2;

#[derive(Debug, Clone, PartialEq)]
pub struct AlphaHeader {}

#[derive(Debug, Clone, PartialEq)]
pub struct RgbHeader {
    pub bits_per_channel: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AlgHeader {
    Alpha(AlphaHeader),
    Rgb(RgbHeader),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Header {
    pub mode: u8,
    pub alg_header: AlgHeader,
}

impl Header {
    pub fn new(mode: u8, alg_header: AlgHeader) -> Self {
        Self { mode, alg_header }
    }

    pub fn new_alpha() -> Self {
        Header {
            mode: ALPHA_MODE,
            alg_header: AlgHeader::Alpha(AlphaHeader {}),
        }
    }

    pub fn new_rgb(bits_per_channel: u8) -> Self {
        Header {
            mode: RGB_MODE,
            alg_header: AlgHeader::Rgb(RgbHeader { bits_per_channel }),
        }
    }

    pub fn size(&self) -> usize {
        let mut size: usize = 0;
        size += size_of_val(&self.mode);

        match &self.alg_header {
            AlgHeader::Alpha(_alg_header) => {}
            AlgHeader::Rgb(alg_header) => {
                size += size_of_val(&alg_header.bits_per_channel);
            }
        }

        size
    }
}

#[cfg(test)]
mod tests {
    use crate::coder::header::RGB_MODE;

    use super::ALPHA_MODE;

    #[test]
    fn new() {
        let mode = ALPHA_MODE;
        let alg_header = super::AlgHeader::Alpha(super::AlphaHeader {});
        let header = super::Header::new(mode, alg_header.clone());

        assert_eq!(header.mode, mode);
        assert_eq!(header.alg_header, alg_header);
    }

    #[test]
    fn new_alpha() {
        let header = super::Header::new_alpha();
        assert_eq!(header.mode, ALPHA_MODE);
        assert_eq!(
            header.alg_header,
            super::AlgHeader::Alpha(super::AlphaHeader {})
        );
    }

    #[test]
    fn new_rgb() {
        let bits_per_channel = 4;
        let header = super::Header::new_rgb(bits_per_channel);
        assert_eq!(header.mode, RGB_MODE);
        assert_eq!(
            header.alg_header,
            super::AlgHeader::Rgb(super::RgbHeader { bits_per_channel })
        );
    }

    #[test]
    fn size_alpha() {
        let header = super::Header::new_alpha();
        assert_eq!(header.size(), 1);
    }
    #[test]
    fn size_rgb() {
        let bits_per_channel = 4;
        let header = super::Header::new_rgb(bits_per_channel);
        assert_eq!(header.size(), 2);
    }
}
