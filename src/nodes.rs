pub struct Node {
    pub(crate) kind: NodeType,
}

pub enum NodeType {
    Operator(char, Vec<Node>),
    Literal(String, Vec<Node>),
}

impl Node {
    pub fn get_children(&self) -> &Vec<Node> {
        match &self.kind {
            NodeType::Operator(_, vec) => vec,
            NodeType::Literal(_, vec1) => vec1,
        }
    }
    pub fn traverse(&self, tabs: usize) {
        print!("{}", "  ".repeat(tabs));
        match &self.kind {
            NodeType::Operator(op, _) => println!("{}", op),
            NodeType::Literal(bytes, _) => println!("{}", bytes),
        }
        for child in self.get_children() {
            child.traverse(tabs+1);
        }
    }
}
