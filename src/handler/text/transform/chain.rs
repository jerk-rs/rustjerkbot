use super::base::{TransformResult, TransformText};

pub struct Chain {
    transformers: Vec<Box<dyn TransformText + Send + Sync>>,
}

impl Chain {
    pub fn new(transformers: Vec<Box<dyn TransformText + Send + Sync>>) -> Self {
        Self { transformers }
    }
}

impl TransformText for Chain {
    fn transform(&self, input: &str) -> TransformResult<String> {
        let mut result = String::from(input);
        for i in &self.transformers {
            result = i.transform(&result)?;
        }
        Ok(result)
    }
}
