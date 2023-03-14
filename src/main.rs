use rs_bf::interpreter::Interpreter;
use rs_bf::lexer::Lexer;
use std::fs;

fn main() {
    let program = fs::read_to_string("scripts/hello.b").expect("Failed to open the file");
    let mut lexer = Lexer::new(&program);
    let tokens = lexer.scan_tokens();
    let mut interpreter = Interpreter::new(tokens.clone());
    interpreter.run();
}
