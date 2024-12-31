
#[derive(Debug)]
pub enum Token {
    StringLiteral(String, i32),
    Identifier(String, i32),
    Punctuation(char, i32),
    Bracket(char, i32),
    Operator(char, i32)
}
