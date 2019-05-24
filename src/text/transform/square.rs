use super::{
    utils::{collect_uppercase_chars, validate_len},
    TransformResult,
};

pub fn transform(orig: &str) -> TransformResult<String> {
    validate_len(2, 100, orig.len())?;
    let chars = collect_uppercase_chars(&orig);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok() {
        let transformed = transform("text").unwrap();
        let mut lines = transformed.lines();
        assert_eq!(lines.next(), Some("T T T T T T T"));
        assert_eq!(lines.next(), Some("T X X X X X T"));
        assert_eq!(lines.next(), Some("T X E E E X T"));
        assert_eq!(lines.next(), Some("T X E T E X T"));
        assert_eq!(lines.next(), Some("T X E E E X T"));
        assert_eq!(lines.next(), Some("T X X X X X T"));
        assert_eq!(lines.next(), Some("T T T T T T T"));
    }

    #[test]
    fn err() {
        let expected = String::from("Text must contain from 2 up to 100 characters");

        let err = transform("").unwrap_err();
        assert_eq!(err.to_string(), expected);

        let err = transform("a").unwrap_err();
        assert_eq!(err.to_string(), expected);

        let err = transform(&"a".repeat(101)).unwrap_err();
        assert_eq!(err.to_string(), expected);

        assert_eq!(transform(&"a".repeat(3)).is_ok(), true);
        assert_eq!(transform(&"a".repeat(100)).is_ok(), true);
    }
}
