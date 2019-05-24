use super::{TransformError, TransformResult};

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
