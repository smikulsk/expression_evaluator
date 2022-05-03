use crate::token_parser::Token;

use super::*;

#[test]
pub fn parse_empty_string_returns_empty_list() {
    let expected : Vec::<Token> = vec![];
    let given_expression = String::new();

    let tokens = parse(&given_expression);

    assert_eq!(tokens, expected);
}