use crate::primitives::Glyph as Operator;
use crate::span::Span as Sp;

#[derive(Debug, Clone)]
pub enum Node {
    Dyad(Box<Sp<Node>>, Operator, Box<Sp<Node>>),
    Monad(Operator, Box<Sp<Node>>),
    MoOp(Operator, Box<Sp<Node>>),
    Literal(Literal),
    Element(String),
    Variable(String),
    Closure(Vec<Sp<Node>>),
    Omega,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Str(String),
    Num(i64),
    Float(f64),
    Array(Vec<Literal>),
}
