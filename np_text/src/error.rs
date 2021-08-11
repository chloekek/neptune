use std::result;

/// Result type for text layout and rendering operations.
pub type Result<T> = result::Result<T, Error>;

/// Error type for text layout and rendering operations.
#[derive(Debug)]
pub struct Error
{
    inner: Inner,
}

#[derive(Debug)]
enum Inner
{
    FaceParsingError(ttf_parser::FaceParsingError),
}

impl From<ttf_parser::FaceParsingError> for Error
{
    fn from(other: ttf_parser::FaceParsingError) -> Self
    {
        Self{inner: Inner::FaceParsingError(other)}
    }
}
