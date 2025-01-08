pub enum NodeType {
    Operator(char, Vec<NodeType>),
    Literal(String, Vec<NodeType>),
}

impl NodeType {
    pub fn get_children(&self) -> &Vec<Self> {
        match &self {
            NodeType::Operator(_, vec) => vec,
            NodeType::Literal(_, vec1) => vec1,
        }
    }
    pub fn traverse(&self, tabs: usize) {
        print!("{}", "  ".repeat(tabs));
        match &self {
            NodeType::Operator(op, _) => println!("{}", op),
            NodeType::Literal(bytes, _) => println!("{}", bytes),
        }
        for child in self.get_children() {
            child.traverse(tabs + 1);
        }
    }
}
