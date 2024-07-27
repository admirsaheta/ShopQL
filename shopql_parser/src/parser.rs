use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "../grammar.pest"]
pub struct ShopQLParser;

pub type RuleError = pest::error::Error<Rule>;
