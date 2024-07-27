use crate::parser::Rule;
use pest::iterators::Pair;
use pest::error::{Error, ErrorVariant};

pub(crate) fn defined_error(pair: Pair<Rule>, kind: &str, name: &str) -> Error<Rule> {
    create_defined_error(kind, name, pair.as_span())
}

fn create_defined_error(kind: &str, name: &str, span: pest::Span<'_>) -> Error<Rule> {
    let message = format!("{} \"{}\" is already defined", kind, name);
    Error::new_from_span(
        ErrorVariant::CustomError { message },
        span,
    )
}
