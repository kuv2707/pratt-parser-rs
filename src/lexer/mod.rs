use lexemes::Token;

mod lexemes;

pub struct Lexer {
    pub toks: Vec<Token>,
    linecnt: i32,
    word: String,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            toks: vec![],
            linecnt: 1,
            word: String::new(),
        }
    }
    pub fn lexify(&mut self, src: &str) {
        let mut it = src.chars();
        while let Some(c) = it.next() {
            if is_delimiter(c) {
                self.parse_word();
                if c == '\n' {
                    self.linecnt += 1;
                }
            } else if is_punctuation(c) {
                self.parse_word();
                self.add_token(Token::Punctuation(c, self.linecnt));
            } else if is_lexical_quote(c) {
                self.parse_word();
                while let Some(c) = it.next() {
                    if is_lexical_quote(c) {
                        self.add_token(Token::StringLiteral(self.word.clone(), self.linecnt));
                        self.word.clear();
                        break;
                    } else {
                        if c == '\n' {
                            self.linecnt += 1;
                        }
                        self.word.push(c);
                    }
                }
            } else if is_bracket(c) {
                self.parse_word();
                self.add_token(Token::Bracket(c, self.linecnt));
            } else if is_operator(c) {
                self.parse_word();
                self.add_operator(c);
            } else {
                self.word.push(c);
            }
        }
        self.parse_word();
    }
    fn parse_word(&mut self) {
        if self.word.len() == 0 {
            return;
        }
        self.add_token(Token::Identifier(self.word.clone(), self.linecnt));
        self.word.clear();
    }
    fn add_token(&mut self, t: Token) {
        self.toks.push(t);
    }
    fn add_operator(&mut self, c: char) {
        // todo: multi-character operators
        self.add_token(Token::Operator(c, self.linecnt));
    }
}

fn is_delimiter(c: char) -> bool {
    c == ' ' || c == '\n' || c == '\t' || c == '\r'
}

fn is_punctuation(c: char) -> bool {
    c == ',' || c == ';' || c == ':'
}

fn is_lexical_quote(c: char) -> bool {
    c == '"' || c == '`'
}

fn is_bracket(c: char) -> bool {
    c == '(' || c == ')' || c == '{' || c == '}' || c == '[' || c == ']' || c == '<' || c == '>'
}

fn is_operator(c: char) -> bool {
    matches!(
        c,
        '+' | '-' | '*' | '/' | '%' | '<' | '>' | '!' | '|' | '&' | '=' | '#' | '.' | '?'
    )
}
