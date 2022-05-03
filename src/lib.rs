use token_parser::parse;

pub mod token_parser;

pub fn evaluate(expression : &str) -> f64 {
    let tokens = parse(expression);
    println!("{:?}", tokens);
    0.0
}