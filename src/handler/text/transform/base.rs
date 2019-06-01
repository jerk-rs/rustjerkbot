use failure::Fail;

#[derive(Fail, Debug)]
pub enum TransformError {
    #[fail(display = "Text must contain from {} up to {} characters", min, max)]
    InvalidLength { min: usize, max: usize },
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
