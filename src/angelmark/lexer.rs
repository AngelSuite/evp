use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "angelmark/angelmark.pest"]
pub struct AngelmarkParser;
