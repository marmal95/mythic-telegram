use core::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct EncodeError(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct DecodeError(pub String);

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

impl std::error::Error for EncodeError {}
impl std::error::Error for DecodeError {}
