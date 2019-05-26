use super::TransformResult;

pub fn transform(orig: &str) -> TransformResult<String> {
    Ok(orig.chars().rev().collect())
}
