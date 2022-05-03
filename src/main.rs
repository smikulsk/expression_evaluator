use std::io;
use expression_evaluator::evaluate;

fn main() {
    let mut expression = String::new();
    io::stdin().read_line(&mut expression).expect("Error reading line");
    evaluate(&expression);
}