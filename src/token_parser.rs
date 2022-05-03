#[derive(Debug, PartialEq)]
pub enum TokenType {
    Numeric(f64),
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Sqrt,
    LeftBracket,
    RightBracket,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
}

pub fn parse(s: &str) -> Vec<Token> {
    let mut tokens = vec![];
    let mut current_token = String::new();
    for ch in s.to_lowercase().chars() {
        if ch.is_digit(10) || ch == '.' {
            current_token.push(ch);
        } else {
            if !current_token.is_empty() && !["s".to_owned(), "sq".to_owned(), "sqr".to_owned()].contains(&current_token) {
                tokens.push(Token {
                    token_type: TokenType::Numeric(current_token.parse().unwrap()),
                });
                current_token = String::new();
            }
            match ch {
                '+' => tokens.push(Token {
                    token_type: TokenType::Add,
                }),
                '-' => tokens.push(Token {
                    token_type: TokenType::Subtract,
                }),
                '*' => tokens.push(Token {
                    token_type: TokenType::Multiply,
                }),
                '/' => tokens.push(Token {
                    token_type: TokenType::Divide,
                }),
                '^' => tokens.push(Token {
                    token_type: TokenType::Power,
                }),
                '(' => tokens.push(Token {
                    token_type: TokenType::LeftBracket,
                }),
                ')' => tokens.push(Token {
                    token_type: TokenType::RightBracket,
                }),
                's' => current_token.push(ch),
                'q' => if current_token == "s" {
                    current_token.push(ch);
                },
                'r' => if current_token == "sq" {
                    current_token.push(ch);
                },
                't' => if current_token == "sqr" {
                    tokens.push(Token {
                        token_type: TokenType::Sqrt,
                    });
                    current_token = String::new();
                },
                _ => () //TODO: Add parsing error
            }
        }
    }
    tokens
}
