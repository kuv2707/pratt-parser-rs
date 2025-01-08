use crate::{
    lexer::{lexemes::Token, Lexer},
    nodes::{Node, NodeType},
};

pub struct Parser {
    tokens: Vec<Token>,
    i: usize,
}

pub fn get_precedence(op: char) -> f32 {
    match op {
        '.' => 3.0,
        '+' => 2.0,
        '!' => 1.0,
        '=' => 0.1,
        _ => panic!("Unknown operator {}", op),
    }
}

impl Parser {
    pub fn new(v: &Vec<Token>) -> Self {
        Self {
            tokens: v.to_vec(),
            i: 0,
        }
    }
    pub fn parse_expression(&mut self, min_prec: f32) -> Node {
        if !self.has_tokens() {
            panic!("Unexpected end of input")
        }
        let mut left_node = self.prefix_parse_current_token();
        while self.has_tokens() {
            println!("Examining {}", self.current_token());
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
        &self.tokens[self.i - 1]
    }
    pub fn look_ahead_token(&self, offset: i32) -> &Token {
        let ind = (self.i as i32 + offset) as usize;
        if ind >= self.tokens.len() {
            &Token::None
        } else {
            &self.tokens[ind]
        }
    }
    pub fn prefix_parse_current_token(&mut self) -> Node {
        let t = self.current_token().clone();
        println!("parsing {}", t);
        match t {
            Token::StringLiteral(a, b) => parse_data_value(self, &a, b),
            Token::Identifier(a, b) => parse_data_value(self, &a, b),
            Token::Punctuation(_, _) => todo!(),
            Token::Bracket(a, b) => parse_bracket_expr(self, a, b),
            Token::Operator(a, b) => parse_prefix_op(self, a, b),
            Token::None => panic!("Can't parse {}", self.current_token()),
        }
    }
    // returns postfix operator node if success, else the passed node
    pub fn try_postfix_parse_current_token(&self, left: Node) -> Result<Node, Node> {
        if let Token::Operator(op, ln) = self.current_token() {
            if is_postfix_operator(*op) {
                return Ok(Node{
                    kind: NodeType::Operator(*op, vec![left])
                })
            }
        }
        Err(left)
    }
}

fn parse_data_value(p: &mut Parser, s: &str, ln: i32) -> Node {
    p.consume_token();
    Node {
        kind: NodeType::Literal(s.to_string(), vec![])
    }
}

fn parse_bracket_expr(p: &mut Parser, brac: char, ln: i32) -> Node {
    p.consume_token();
    println!("parsing bracket {}", p.current_token());
    let inner = p.parse_expression(0.0);
    p.consume_token();
    inner
}

fn parse_prefix_op(p: &mut Parser, op: char, ln: i32) -> Node {
    println!("{}", p.current_token());
    p.consume_token();
    let right = p.parse_expression(get_precedence(op));
    Node {
        kind: NodeType::Operator(op, vec![right])
    }
}

fn parse_infix(p: &mut Parser, op: char, left: Node) -> Node {
    p.consume_token();
    let right = p.parse_expression(get_precedence(op));
    Node {
        kind: NodeType::Operator(op, vec![left, right])
    }
}

fn is_postfix_operator(op: char) -> bool {
    //todo: add postfix operators
    return false;
}

#[test]
fn test_parse() {
    let mut lx = Lexer::new();
    let toks = lx.lexify("a.b+c");
    println!("{:?}", toks);
    let mut p = Parser::new(toks);
    let n = p.parse_expression(0.0);
    n.traverse(0);
}
