use crate::parser::Rule;
use pest::iterators::Pair;
use pest::error::{Error, ErrorVariant};

pub(crate) fn custom_error(pair: Pair<Rule>, message: &str) -> Box<dyn std::error::Error> {
    Box::new(create_custom_error(message, pair.as_span()))
}

fn create_custom_error(message: &str, span: pest::Span<'_>) -> Error<Rule> {
    Error::new_from_span(
        ErrorVariant::CustomError { message: message.into() },
        span,
    )
}
