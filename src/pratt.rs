use crate::{
    lexer::{lexemes::Token, Lexer},
    nodes::NodeType,
};

pub struct Parser {
    tokens: Vec<Token>,
    i: usize,
}

pub fn get_precedence(op: char) -> f32 {
    match op {
        '*' | '/' => 4.0,
        '.' => 3.0,
        '+' | '-' => 2.0,
        '!' => 1.0,
        '=' => 0.1,
        _ => -0.5, // panic!("Unknown operator {}", op),
    }
}

impl Parser {
    pub fn new(v: &Vec<Token>) -> Self {
        Self {
            tokens: v.to_vec(),
            i: 0,
        }
    }
    pub fn parse(&mut self) -> Vec<NodeType> {
        let mut k = Vec::new();
        while self.has_tokens() {
            k.push(self.parse_expression(0.0));
            self.consume_token();
        }
        k
    }
    fn parse_expression(&mut self, min_prec: f32) -> NodeType {
        let mut left_node = self.prefix_parse_current_token();
        'out: while self.has_tokens() {
            match self.current_token() {
                Token::Operator(op, _) => {
                    if get_precedence(*op) <= min_prec {
                        break;
                    }
                    left_node = match self.try_postfix_parse_current_token(left_node) {
                        Ok(left) => left,
                        Err(left) => parse_infix(self, *op, left),
                    };
                }
                Token::Bracket(br, _) => {
                    break;
                }
                _ => {
                    if self.current_token().is_punc() {
                        break 'out;
                    }
                    panic!("Malformed expression! {}", self.current_token());
                }
            }
        }
        return left_node;
    }
    pub fn has_tokens(&self) -> bool {
        self.i < self.tokens.len()
    }
    pub fn current_token(&self) -> &Token {
        &self.tokens[self.i]
    }
    pub fn consume_token(&mut self) -> &Token {
        self.i += 1;
        match self.tokens.get(self.i - 1) {
            Some(tok) => tok,
            None => &Token::None,
        }
    }
    pub fn look_ahead_token(&self, offset: i32) -> &Token {
        let ind = (self.i as i32 + offset) as usize;
        if ind >= self.tokens.len() {
            &Token::None
        } else {
            &self.tokens[ind]
        }
    }
    pub fn prefix_parse_current_token(&mut self) -> NodeType {
        let t = self.current_token().clone();
        match t {
            Token::StringLiteral(a, b) => parse_data_value(self, &a, b),
            Token::Identifier(a, b) => parse_data_value(self, &a, b),
            Token::Punctuation(a, b) => parse_prefix_op(self, a, b),
            Token::Bracket(a, b) => parse_bracket_expr(self, a, b),
            Token::Operator(a, b) => parse_prefix_op(self, a, b),
            Token::None => panic!("Can't parse {}", self.current_token()),
        }
    }
    // returns postfix operator node if success, else the passed node
    pub fn try_postfix_parse_current_token(&self, left: NodeType) -> Result<NodeType, NodeType> {
        if let Token::Operator(op, ln) = self.current_token() {
            if is_postfix_operator(*op) {
                return Ok(NodeType::Operator(*op, vec![left]));
            }
        }
        Err(left)
    }
}

fn parse_data_value(p: &mut Parser, s: &str, ln: i32) -> NodeType {
    p.consume_token();
    NodeType::Literal(s.to_string(), vec![])
}

fn parse_bracket_expr(p: &mut Parser, brac: char, ln: i32) -> NodeType {
    p.consume_token();
    let inner = p.parse_expression(0.0);
    p.consume_token();
    inner
}

fn parse_prefix_op(p: &mut Parser, op: char, ln: i32) -> NodeType {
    p.consume_token();
    let right = p.parse_expression(get_precedence(op));

    NodeType::Operator(op, vec![right])
}

fn parse_infix(p: &mut Parser, op: char, left: NodeType) -> NodeType {
    p.consume_token();
    let right = p.parse_expression(get_precedence(op));
    NodeType::Operator(op, vec![left, right])
}

fn is_postfix_operator(op: char) -> bool {
    //todo: add postfix operators
    return false;
}

#[test]
fn test_parse() {
    let mut lx = Lexer::new();
    let toks = lx.lexify("a = a * (b * (c+d)) - 4;");
    println!("{:?}", toks);
    let mut p = Parser::new(toks);
    let n = p.parse();
    for k in n {
        k.traverse(0);
        println!("----")
    }
}
