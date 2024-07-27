use crate::parser::{ShopQLParser, Rule, RuleError};
use pest::error::ErrorVariant;
use pest::iterators::Pair;
use pest::{Parser, Position};

#[allow(dead_code)]
pub fn parse_full_input<R>(
    input: &str,
    rule: Rule,
    parser: fn(Pair<Rule>, file: &str) -> Result<R, Box<RuleError>>,
) -> Result<R, Box<RuleError>> {
    let pair_or_err = ShopQLParser::parse(rule, input);
    if let Err(err) = &pair_or_err {
        eprintln!("{}", err);
    }

    let input = input.trim_end();
    let mut pair = pair_or_err?;
    let parsed = pair.next().ok_or_else(|| {
        let err = create_parse_error("no pair found", input, pair.as_str().len());
        eprintln!("{}", err);
        Box::new(err) as Box<RuleError>
    })?;

    if !is_whole_input_parsed(parsed.as_str(), input) {
        let err = create_parse_error(
            &format!("not everything was parsed: {}", &input[parsed.as_str().len()..]),
            input,
            pair.as_str().len(),
        );
        eprintln!("{}", err);
        return Err(Box::new(err));
    }

    let res = parser(parsed, "");
    if let Err(err) = &res {
        eprintln!("{}", err);
    }
    res
}

fn is_whole_input_parsed(parsed_str: &str, input: &str) -> bool {
    parsed_str.len() >= input.len()
}

fn create_parse_error(message: &str, input: &str, pos_len: usize) -> pest::error::Error<Rule> {
    pest::error::Error::new_from_pos(
        ErrorVariant::CustomError {
            message: message.to_string(),
        },
        Position::new(input, pos_len).unwrap(),
    )
}
