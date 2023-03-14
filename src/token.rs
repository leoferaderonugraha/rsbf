use crate::token_type::TokenType;

#[derive(Copy, Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: char,
    pub line: usize,
    pub pos: usize,
}
