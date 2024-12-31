use lexer::Lexer;

mod lexer;

fn main() {
    let mut lx = Lexer::new();
    lx.lexify("A=(B&!C)|D");
    for k in lx.toks {
        println!("{:?}",k);
    }
}
