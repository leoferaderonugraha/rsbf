#[derive(Copy, Clone, Debug)]
pub enum TokenType {
    MoveRight,
    MoveLeft,
    Add,
    Sub,
    Dot,
    Comma,
    OpenSquare,
    CloseSquare,
    EOF,
}
