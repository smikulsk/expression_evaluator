
#[derive(Debug, PartialEq)]
pub enum TokenType {
    Numeric,
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Sqrt      
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type : TokenType   
}

pub fn parse(s : &str) -> Vec::<Token> {
    let tokens = Vec::<Token>::new();
    tokens
}

#[cfg(test)]
pub mod token_parser_tests {
    use super::*;

    #[test]
    pub fn parse_empty_string_returns_empty_list() {
        let expected : Vec::<Token> = vec![];
        let given_expression = String::new();

        let tokens = parse(&given_expression);

        assert_eq!(tokens, expected);
    }
}