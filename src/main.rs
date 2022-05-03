use token_parser::parse;
use std::io;

mod token_parser;

fn main() {
    let mut expression = String::new();
    io::stdin().read_line(&mut expression).expect("Error reading line");
    evaluate(&expression);
}

fn evaluate(expression : &str) -> f64 {
    let tokens = parse(expression);
    println!("{:?}", tokens);
    0.0
}