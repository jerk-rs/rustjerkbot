use super::base::{collect_uppercase_chars, validate_len, TransformResult, TransformText};

pub struct Square {
    min_len: usize,
    max_len: usize,
}

impl Square {
    pub fn new() -> Self {
        Self {
            min_len: 2,
            max_len: 100,
        }
    }
}
impl TransformText for Square {
    fn transform(&self, input: &str) -> TransformResult<String> {
        validate_len(self.min_len, self.max_len, input.len())?;
        let chars = collect_uppercase_chars(&input);
        let len = chars.len();
        let side = len * 2 - 1;
        let area = side * side;
        let mut buf = String::with_capacity(area * 2 - 1);
        let mut row_idx;
        let mut col_idx;
        for row in 0..side {
            row_idx = if row >= len { side - row - 1 } else { row };
            for col in 0..side {
                col_idx = if col >= len { side - col - 1 } else { col };
                buf.push(chars[len - 1 - if row_idx <= col_idx { row_idx } else { col_idx }]);
                if col != side - 1 {
                    buf.push(' ');
                }
            }
            if row != side - 1 {
                buf.push('\n');
            }
        }
        Ok(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok() {
        let t = Square::new();
        let transformed = t.transform("text").unwrap();
        let lines: Vec<&str> = transformed.lines().collect();
        assert_eq!(
            lines,
            vec![
                "T T T T T T T",
                "T X X X X X T",
                "T X E E E X T",
                "T X E T E X T",
                "T X E E E X T",
                "T X X X X X T",
                "T T T T T T T",
            ]
        );

        assert!(t.transform(&"a".repeat(3)).is_ok());
        assert!(t.transform(&"a".repeat(100)).is_ok());
    }

    #[test]
    fn err() {
        let expected = String::from("Text must contain from 2 up to 100 characters");
        let t = Square::new();

        let err = t.transform("").unwrap_err();
        assert_eq!(err.to_string(), expected);

        let err = t.transform("a").unwrap_err();
        assert_eq!(err.to_string(), expected);

        let err = t.transform(&"a".repeat(101)).unwrap_err();
        assert_eq!(err.to_string(), expected);
    }
}
