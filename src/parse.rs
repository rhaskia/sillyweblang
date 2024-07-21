use crate::lex::Token;
use crate::primitives::Glyph as Operator;
use crate::ast::{Node, Literal};
use crate::span::Span as Sp;
use crate::span::Position;

type Error = String;

pub fn parse(tokens: Vec<Sp<Token>>) -> Result<Sp<Node>, Error> {
    let index = tokens.len();
    Parser { tokens, index }.parse()
}

struct Parser {
    tokens: Vec<Sp<Token>>,
    index: usize,
}

impl Parser {
    pub fn parse(&mut self) -> Result<Sp<Node>, Error> {
        self.statement()
    }

    pub fn statement(&mut self) -> Result<Sp<Node>, Error> {
        let node = parse_array()?; 

        while let Token::Glyph(_) == self.peek() {

        }
    }

    pub fn expr(&mut self) -> Result<Sp<Node>, Error> {
        let left = self.simple()?;

        if let Token::Operator(op) = *self.next()? {
            let right = self.expr()?;

            // ordering switching
            if let Node::Dyad(ref r_left, ref r_op, ref r_right) = *right {
                if Self::op_order(&op) > Self::op_order(r_op) {
                    return Ok(Self::new_binary(
                        Self::new_binary(left, op, *r_left.clone()),
                        r_op.clone(),
                        *r_right.clone(),
                    ));
                }
            }

            return Ok(Self::new_binary(left, op, right));
        }

        Ok(left)
    }

//    pub fn parse_array() -> Result<Sp<Node>, Error> {}

    pub fn simple(&mut self) -> Result<Sp<Node>, Error> {
        let Sp { value, start, end } = self.next()?;
        Ok(match value {
            Token::Number(n) => Sp::new(Node::Literal(Literal::Num(n)), start, end),
            Token::Str(n) => Sp::new(Node::Literal(Literal::Str(n)), start, end),
            Token::Var(n) => Sp::new(Node::Variable(n), start, end),
            Token::OpenBrace => self.closure(start)?,
            Token::Omega => Sp::new(Node::Omega, start, end),
            _ => panic!("{value:?}"),
        })
    }

    pub fn closure(&mut self, start: Position) -> Result<Sp<Node>, Error> {
        let expr = self.expr()?;
        self.next()?;
        Ok(expr)
    }

    pub fn new_binary(left: Sp<Node>, op: Operator, right: Sp<Node>) -> Sp<Node> {
        let start = left.start.min(right.start);
        let end = left.start.max(right.end);
        Sp {
            start,
            value: Node::Dyad(Box::new(left), op, Box::new(right)),
            end,
        }
    }
    
    pub fn op_order(operator: &Operator) -> u8 {
        match operator {
            _ => 0,
        }
    }

    pub fn next(&mut self) -> Result<Sp<Token>, Error> {
        self.index -= 1;
        self.tokens.get(self.index).ok_or(String::from("EOF reached")).cloned()
    }
}
