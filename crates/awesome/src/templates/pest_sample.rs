// main.rs
use pest;
use pest::Parser;
use pest_derive;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "templates/pest/grammar.pest"]
pub struct ExpressionParser;

fn pest_sample() {
    let input = "1 + 2 * 3";
    let pairs = ExpressionParser::parse(Rule::expression, input).unwrap();

    for pair in pairs {
        println!("Rule: {:?}", pair.as_rule());
        println!("Span: {:?}", pair.as_span());
        println!("Text: {}", pair.as_str());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pest_sample() {
        pest_sample();
    }
}
