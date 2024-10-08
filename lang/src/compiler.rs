use std::ops::Add;
use crate::element::Element;
use crate::span::Span as Sp;
use crate::primitives::{Glyph as Operator, GlyphType, GlyphLoader};
use crate::ast::{Node, Literal};

pub fn compile(tree: Node) -> Element {
    let dom = Compiler { tree, glyphs: GlyphLoader::setup() }.compile();
    println!("{dom:?}");
    dom
}

struct Compiler {
    tree: Node,
    glyphs: GlyphLoader,
}

impl Compiler {
    pub fn compile(&mut self) -> Element {
        let mut root = Element::new("html");

        root.add(self.eval(self.tree.clone()).as_el());

        root
    }

    pub fn eval(&self, node: Node) -> Value {
        match node {
            Node::Dyad(l, op, r) => self.eval_dyad(*l, op, *r),
            Node::Monad(op, l) => self.eval_monad(op, *l),
            Node::Literal(lit) => match lit {
                Literal::Str(s) => Value::Str(s),
                Literal::Num(n) => Value::Num(n),
                Literal::Float(f) => Value::Num(f as i64),
                _ => unreachable!(),
            } 
            Node::Array(a) => Value::Array(a.into_iter().map(|n| self.eval(n.value)).collect()),
            Node::Variable(name) => Value::El(Element::new(&name)),
            _ => panic!("{node:?}"),
        }

    }

    pub fn eval_dyad(&self, left: Sp<Node>, op: Operator, right: Sp<Node>) -> Value {
        let left = self.eval(left.value);
        let right = self.eval(right.value);

        match op.glyph_type {
            GlyphType::Css => Value::El(right.as_el().with_attr(&self.glyphs[&op.glyph].attr, left)),
            GlyphType::Bracket => Value::El(right.as_el().with_children(left.as_el_arr())),
            GlyphType::Math => Self::eval_arithmetic(left, op.glyph, right),
            _ => panic!("{op:?}")
        }
    } 

    pub fn eval_monad(&self, op: Operator, left: Sp<Node>) -> Value {
        let left = self.eval(left.value);

        match op {
            _ => panic!()
        }
    } 
    
    pub fn eval_arithmetic(left: Value, op: char, right: Value) -> Value {
        let left = left.as_num();
        let right = right.as_num();
        Value::Num(match op {
            '+' => left + right,
            '-' => left - right,
            '*' => left * right,
            '/' => left / right,
            _ => unreachable!(),
        })
    }
}

#[derive(Debug)]
pub enum Value {
    Num(i64),
    Str(String),
    El(Element),
    Array(Vec<Value>)
}

impl Value {
    pub fn as_el(self) -> Element {
        match self {
            Self::El(el) => el,
            _ => panic!("{self:?}") // throw error
        }
    }

    pub fn as_el_arr(self) -> Vec<Element> {
        match self {
            Self::Array(a) => a.into_iter().map(Value::as_el).collect(),
            _ => panic!()
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Value::Num(n) => n.to_string(),
            Value::Str(str) => str.to_string(),
            Value::El(el) => el.to_string(),
            Value::Array(arr) => arr.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "),
        }
    }

    pub fn as_num(&self) -> i64 {
        match self {
            Value::Num(n) => *n,
            _ => panic!("{self:?}")
        }
    }
}
