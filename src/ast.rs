use crate::primitives::Glyph as Operator;
use crate::span::Span as Sp;
use std::fmt::Debug;

#[derive(Clone)]
pub enum Node {
    Dyad(Box<Sp<Node>>, Operator, Box<Sp<Node>>),
    Monad(Operator, Box<Sp<Node>>),
    MoOp(Operator, Box<Sp<Node>>),
    Literal(Literal),
    Element(String),
    Variable(String),
    Closure(Vec<Sp<Node>>),
    Array(Vec<Sp<Node>>),
    Omega,
}

#[derive(Clone)]
pub enum Literal {
    Str(String),
    Num(i64),
    Float(f64),
    Array(Vec<Literal>),
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dyad(l, op, r) => write!(f, "({l:?} {op} {r:?})"),
            Self::Monad(op, r) => write!(f, "({op} {r:?})"),
            Self::MoOp(arg0, arg1) => f.debug_tuple("MoOp").field(arg0).field(arg1).finish(),
            Self::Literal(lit) => lit.fmt(f),
            Self::Element(el) => f.write_str(el),
            Self::Variable(name) => f.write_str(name),
            Self::Closure(arg0) => f.debug_tuple("Closure").field(arg0).finish(),
            Self::Array(arr) => write!(f, "{arr:?}"),
            Self::Omega => write!(f, "⍵"),
        }
    }
}

impl Debug for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Str(str) => write!(f, "{str:?}"),
            Self::Num(num) => write!(f, "{num}"),
            Self::Float(fl) => write!(f, "{fl}"),
            Self::Array(arr) =>  write!(f, "{arr:?}"),
        }
    }
}
