use crate::coder::{
    error::HeaderEncodeError,
    header::{AlgHeader, Header, RgbHeader},
};

pub fn encode(header: Header, buffer: &mut [u8]) -> Result<(), HeaderEncodeError> {
    let mut iter = buffer.iter_mut().skip(3).step_by(4);
    let mode_byte = iter.next().ok_or(HeaderEncodeError(
        "Not enough to encode header mode.".to_string(),
    ))?;
    *mode_byte = header.mode;

    match header.alg_header {
        AlgHeader::Alpha(_) => encode_alpha(&mut iter)?,
        AlgHeader::Rgb(alg_header) => encode_rgb(&mut iter, &alg_header)?,
    }

    Ok(())
}

fn encode_alpha<'a, I>(_iter: &mut I) -> Result<(), HeaderEncodeError>
where
    I: Iterator<Item = &'a mut u8>,
{
    Ok(())
}

fn encode_rgb<'a, I>(iter: &mut I, header: &RgbHeader) -> Result<(), HeaderEncodeError>
where
    I: Iterator<Item = &'a mut u8>,
{
    let bits_per_channel_byte = iter.next().ok_or(HeaderEncodeError(
        "Not enough to encode header bits per channel.".to_string(),
    ))?;
    *bits_per_channel_byte = header.bits_per_channel;
    Ok(())
}

#[cfg(test)]
mod tests {

    use crate::coder::{
        error::HeaderEncodeError,
        header::{Header, ALPHA_MODE, RGB_MODE},
    };

    #[test]
    fn encode_alpha() {
        let header = Header::new_alpha();
        let mut buffer = vec![0; 10];
        assert_eq!(Ok(()), super::encode(header, &mut buffer));
        assert_eq!(buffer, vec![0, 0, 0, ALPHA_MODE, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn encode_rgb() {
        let bits_per_channel = 4;
        let header = Header::new_rgb(bits_per_channel);
        let mut buffer = vec![0; 10];
        assert_eq!(Ok(()), super::encode(header, &mut buffer));
        assert_eq!(
            buffer,
            vec![0, 0, 0, RGB_MODE, 0, 0, 0, bits_per_channel, 0, 0]
        );
    }

    #[test]
    fn encode_error_not_enough_data_for_mode() {
        let header = Header::new_alpha();
        let mut buffer = vec![0; 1];
        let encoded = super::encode(header, &mut buffer);
        assert_eq!(
            encoded,
            Err(HeaderEncodeError(
                "Not enough to encode header mode.".to_string()
            ))
        );
    }

    #[test]
    fn encode_error_not_enough_data_for_bits_per_channel() {
        let bits_per_channel = 1;
        let header = Header::new_rgb(bits_per_channel);
        let mut buffer = vec![0; 4];
        let encoded = super::encode(header, &mut buffer);
        assert_eq!(
            encoded,
            Err(HeaderEncodeError(
                "Not enough to encode header bits per channel.".to_string()
            ))
        );
    }
}
