use std::{error::Error, fmt};

#[derive(Debug)]
pub enum TransformError {
    InvalidLength { min: usize, max: usize },
}

impl Error for TransformError {}

impl fmt::Display for TransformError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TransformError::InvalidLength { min, max } => {
                write!(out, "Text must contain from {} up to {} characters", min, max)
            }
        }
    }
}

pub type TransformResult<T> = Result<T, TransformError>;

pub trait TransformText {
    fn transform(&self, input: &str) -> TransformResult<String>;
}

pub(super) fn validate_len(min: usize, max: usize, len: usize) -> TransformResult<()> {
    if len < min || len > max {
        Err(TransformError::InvalidLength { min, max })
    } else {
        Ok(())
    }
}

pub(super) fn collect_uppercase_chars(s: &str) -> Vec<char> {
    s.chars().flat_map(char::to_uppercase).collect()
}
