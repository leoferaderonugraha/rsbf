use crate::token::Token;
use crate::token_type::TokenType;

pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    pos: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        return Self {
            source: source.clone().to_owned(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            pos: 0,
        };
    }

    pub fn scan_token(&mut self) {
        let ch = self.advance();

        match ch {
            '>' => self.add_token(TokenType::MoveRight, ch),
            '<' => self.add_token(TokenType::MoveLeft, ch),
            '+' => self.add_token(TokenType::Add, ch),
            '-' => self.add_token(TokenType::Sub, ch),
            '.' => self.add_token(TokenType::Dot, ch),
            ',' => self.add_token(TokenType::Comma, ch),
            '[' => self.add_token(TokenType::OpenSquare, ch),
            ']' => self.add_token(TokenType::CloseSquare, ch),
            '\n' => {
                self.line += 1;
                self.pos = 1;
            }
            _ => (),
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.add_token(TokenType::EOF, '\0');

        return self.tokens.clone();
    }

    pub fn get_tokens(&self) -> Vec<Token> {
        return self.tokens.clone();
    }

    fn add_token(&mut self, token_type: TokenType, lexeme: char) {
        self.tokens.push(Token {
            token_type,
            lexeme,
            line: self.line,
            pos: self.pos,
        });
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.pos += 1;

        return self.source.as_bytes()[self.current - 1] as char;
    }

    fn is_at_end(&self) -> bool {
        if self.current >= self.source.len() {
            return true;
        }

        return false;
    }
}
