use super::base::{TransformResult, TransformText};

pub struct Reverse;

impl TransformText for Reverse {
    fn transform(&self, input: &str) -> TransformResult<String> {
        Ok(input.chars().rev().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok() {
        assert_eq!(Reverse.transform("test").unwrap(), "tset");
    }
}
