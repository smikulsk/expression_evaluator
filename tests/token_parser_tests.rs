use expression_evaluator::token_parser::{Token, TokenType, parse};

#[test]
pub fn parse_empty_string_returns_empty_list() {
    let expected : Vec::<Token> = vec![];
    let given_expression = String::new();

    let tokens = parse(&given_expression);

    assert_eq!(tokens, expected);
}

#[test]
pub fn parse_non_empty_string_returns_correct_tokens() {
    let expected = vec![
        Token { token_type: TokenType::Numeric(123.0)},
        Token { token_type: TokenType::Add},
        Token { token_type: TokenType::Numeric(4.56)},
        Token { token_type: TokenType::Multiply},
        Token { token_type: TokenType::LeftBracket},
        Token { token_type: TokenType::Sqrt},
        Token { token_type: TokenType::LeftBracket},
        Token { token_type: TokenType::Numeric(78.0)},
        Token { token_type: TokenType::Divide},
        Token { token_type: TokenType::Numeric(90.0)},
        Token { token_type: TokenType::RightBracket},
        Token { token_type: TokenType::Subtract},
        Token { token_type: TokenType::Numeric(2.0)},
        Token { token_type: TokenType::Power},
        Token { token_type: TokenType::Numeric(12.0)},
        Token { token_type: TokenType::RightBracket},
    ];
    let given_expression = "123+4.56*(sqrt(78/90)-2^12)";

    let tokens = parse(given_expression);

    assert_eq!(tokens, expected);
}