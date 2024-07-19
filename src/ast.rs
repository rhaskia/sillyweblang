use crates::primitives::Glyph as Operator;

enum Node {
    Binary(Box<Node>, Operator, Box<Node>),
    Literal(Literal),
    Element(String),
    Variable(String),
}

enum Literal {
    Str(String),
    Num(i64),
    Float(f64),
    Array(Vec<Literal>),
}
