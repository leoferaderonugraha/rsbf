use crate::expression::{
    AddExpression, CloseSquareExpression, CommaExpression, DotExpression, MoveLeftExpression,
    MoveRightExpression, OpenSquareExpression, SubExpression, Visitor,
};
use crate::token::Token;
use crate::token_type::TokenType;

pub struct Interpreter {
    memory: [u8; 30000],
    pc: usize,
    tokens: Vec<Token>,
    sp: usize,
}

impl Interpreter {
    pub fn new(tokens: Vec<Token>) -> Self {
        return Self {
            memory: [0u8; 30000],
            pc: 0,
            tokens,
            sp: 0,
        };
    }

    pub fn run(&mut self) {
        while !self.is_at_end() {
            let token = self.peek();

            match token.token_type {
                TokenType::MoveRight => self.visit_move_right(&MoveRightExpression {}),
                TokenType::MoveLeft => self.visit_move_left(&MoveLeftExpression {}),
                TokenType::Add => self.visit_add(&AddExpression {}),
                TokenType::Sub => self.visit_sub(&SubExpression {}),
                TokenType::Dot => self.visit_dot(&DotExpression {}),
                TokenType::Comma => self.visit_comma(&CommaExpression {}),
                TokenType::OpenSquare => self.visit_open_square(&OpenSquareExpression {}),
                TokenType::CloseSquare => self.visit_close_square(&CloseSquareExpression {}),
                _ => (),
            }

            self.advance();
        }
    }

    fn is_at_end(&self) -> bool {
        return self.sp >= self.tokens.len();
    }

    fn peek(&self) -> Token {
        return self.tokens[self.sp];
    }

    fn advance(&mut self) -> Token {
        self.sp += 1;
        return self.tokens[self.sp - 1];
    }
}

impl Visitor for Interpreter {
    fn visit_move_right(&mut self, expr: &MoveRightExpression) {
        self.pc += 1;
    }

    fn visit_move_left(&mut self, expr: &MoveLeftExpression) {
        self.pc -= 1;
    }

    fn visit_add(&mut self, expr: &AddExpression) {
        self.memory[self.pc] += 1;
    }

    fn visit_sub(&mut self, expr: &SubExpression) {
        self.memory[self.pc] -= 1;
    }

    fn visit_dot(&mut self, expr: &DotExpression) {
        print!("{}", self.memory[self.pc] as char);
    }

    fn visit_comma(&mut self, expr: &CommaExpression) {
        todo!();
    }

    fn visit_open_square(&mut self, expr: &OpenSquareExpression) {
        if self.memory[self.pc] == 0 {
            let mut level = 1;
            while level > 0 {
                match self.tokens[self.sp].token_type {
                    TokenType::OpenSquare => level += 1,
                    TokenType::CloseSquare => level -= 1,
                    _ => (),
                }
                self.sp += 1;
            }
        }
    }

    fn visit_close_square(&mut self, expr: &CloseSquareExpression) {
        if self.memory[self.pc] != 0 {
            let mut level = 1;
            self.sp -= 1;
            while level > 0 {
                match self.tokens[self.sp].token_type {
                    TokenType::OpenSquare => level -= 1,
                    TokenType::CloseSquare => level += 1,
                    _ => (),
                }
                self.sp -= 1;
            }
        }
    }
}
