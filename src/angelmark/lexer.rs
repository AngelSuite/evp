use pest_derive::Parser;

/// Lex AngelMark into a set of rules with this parser.
#[derive(Parser)]
#[grammar = "angelmark/angelmark.pest"]
pub struct AngelmarkParser;
