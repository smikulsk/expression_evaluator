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