use token_parser::parse;

mod token_parser;
use std::io;

fn main() {
    let mut expression = String::new();
    io::stdin().read_line(&mut expression).expect("Error reading line");
    let tokens = parse(&expression);
    println!("{:?}", tokens);
}

#[cfg(test)]
mod token_parser_tests;
