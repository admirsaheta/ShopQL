use crate::parser::Rule;
use pest::iterators::Pair;
use pest::error::{Error, ErrorVariant};

pub(crate) fn unknown_rule_error(pair: Pair<Rule>, expected_str: &str) -> Box<dyn std::error::Error> {
    let rule = pair.as_rule();
    Box::new(create_unknown_rule_error(rule, expected_str, pair.as_span()))
}

fn create_unknown_rule_error(rule: Rule, expected_str: &str, span: pest::Span<'_>) -> Error<Rule> {
    let message = format!("cannot parse {:?} as {}", rule, expected_str);
    Error::new_from_span(
        ErrorVariant::CustomError { message },
        span,
    )
}
