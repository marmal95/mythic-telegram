use core::fmt;
use std::string::FromUtf8Error;

#[derive(Debug, Clone, PartialEq)]
pub struct EncodeError(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct DecodeError(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct HeaderEncodeError(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct HeaderDecodeError(pub String);

impl fmt::Display for EncodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Encode error: {}", self.0)
    }
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Decode error: {}", self.0)
    }
}

impl fmt::Display for HeaderEncodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Header encode error: {}", self.0)
    }
}

impl fmt::Display for HeaderDecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Header decode error: {}", self.0)
    }
}

impl std::error::Error for EncodeError {}
impl std::error::Error for DecodeError {}
impl std::error::Error for HeaderEncodeError {}
impl std::error::Error for HeaderDecodeError {}

impl From<FromUtf8Error> for DecodeError {
    fn from(value: FromUtf8Error) -> Self {
        DecodeError(value.to_string())
    }
}

#[cfg(test)]
mod tests {

    use super::DecodeError;
    use super::EncodeError;
    use super::HeaderDecodeError;
    use super::HeaderEncodeError;

    #[test]
    fn display_encode_error() {
        let error = EncodeError("some failure".to_string());
        assert_eq!(error.to_string(), "Encode error: some failure");
    }

    #[test]
    fn display_decode_error() {
        let error = DecodeError("some failure".to_string());
        assert_eq!(error.to_string(), "Decode error: some failure");
    }

    #[test]
    fn display_header_encode_error() {
        let error = HeaderEncodeError("some failure".to_string());
        assert_eq!(error.to_string(), "Header encode error: some failure");
    }

    #[test]
    fn display_header_decode_error() {
        let error = HeaderDecodeError("some failure".to_string());
        assert_eq!(error.to_string(), "Header decode error: some failure");
    }
}
