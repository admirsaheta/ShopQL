use pest::Span;
use crate::parser::RuleError;

#[derive(Clone, Debug)]
pub struct Owned {
    #[allow(dead_code)]
    pub err_placeholder: RuleError,
    #[allow(dead_code)]
    pub file: String,
    #[allow(dead_code)]
    pub line: usize,
    #[allow(dead_code)]
    pub col: usize,
    #[allow(dead_code)]
    pub input: String,
    #[allow(dead_code)]
    pub start: usize,
    #[allow(dead_code)]
    pub end: usize,
}

impl Default for Owned {
    fn default() -> Self {
        let err = create_default_error();
        Self {
            err_placeholder: err,
            input: "".to_string(),
            file: "".to_string(),
            line: 0,
            col: 0,
            start: 0,
            end: 0,
        }
    }
}

impl Owned {
    pub fn make_error(&self, msg: &str) -> Box<RuleError> {
        let mut err = self.err_placeholder.clone();
        err.variant = pest::error::ErrorVariant::CustomError {
            message: format!("{}:{} {}", self.file, self.line, msg),
        };
        Box::new(err)
    }

    pub fn from(span: Span<'_>, file: &str) -> Self {
        let (line, col) = span.start_pos().line_col();
        Self {
            err_placeholder: create_default_error(),
            file: file.to_string(),
            line,
            col,
            input: span.as_str().to_string(),
            start: span.start(),
            end: span.end(),
        }
    }
}

fn create_default_error() -> RuleError {
    *Box::new(pest::error::Error::new_from_span(
        pest::error::ErrorVariant::CustomError {
            message: "".to_string(),
        },
        Span::new("", 0, 0).unwrap(),
    ))
}

#[cfg(test)]
impl PartialEq for Owned {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}
