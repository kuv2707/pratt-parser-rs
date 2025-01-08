use std::fmt::Display;


#[derive(Debug, Clone)]
pub enum Token {
    StringLiteral(String, i32),
    Identifier(String, i32),
    Punctuation(char, i32),
    Bracket(char, i32),
    Operator(char, i32),
    None,
}

impl Token {
    pub fn is_punc(&self) -> bool {
        match self {
            Self::Punctuation(_, _) => true,
            _ => false,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::StringLiteral(s, i) => write!(f, "StringLiteral({}, {})", s, i),
            Token::Identifier(s, i) => write!(f, "Identifier({}, {})", s, i),
            Token::Punctuation(c, i) => write!(f, "Punctuation({}, {})", c, i),
            Token::Bracket(c, i) => write!(f, "Bracket({}, {})", c, i),
            Token::Operator(c, i) => write!(f, "Operator({}, {})", c, i),
            Token::None => write!(f, "None"),
        }
    }
}
