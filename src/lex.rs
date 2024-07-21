use crate::span::{Span as Sp, Position};
use crate::primitives::{ToGlyph, Glyph};

pub fn lex(program: String) -> Vec<Sp<Token>> {
    Lexer { program, index: 0, pos: Position::new(), tokens: vec![] }.lex()
}

#[derive(Debug, Clone)]
pub enum Token {
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    OpenParen,
    CloseParen,
    Operator(Glyph),
    Number(i64),
    Str(String),
    Var(String),
    Omega,
}

struct Lexer {
    program: String,
    index: usize,
    tokens: Vec<Sp<Token>>,
    pos: Position,
}

impl Lexer {
    pub fn lex(&mut self) -> Vec<Sp<Token>> {
        while let Some(ch) = self.next() {
           match ch {
               ']' => self.push(Token::OpenBracket),
               '[' => self.push(Token::CloseBracket),
               ')' => self.push(Token::OpenParen),
               '(' => self.push(Token::CloseParen),
               '}' => self.push(Token::OpenBrace),
               '{' => self.push(Token::CloseBrace),
               '"' => self.string(),
               '⍵' => self.push(Token::Omega),
               _ if ch.is_alphabetic() => self.name(ch),
               _ if ch.is_glyph() => self.push(Token::Operator(ch.to_glyph())),
               _ if ch.is_numeric() => self.number(ch),
               _ => {},
           }
        } 

        self.tokens.clone()
    } 

    pub fn push(&mut self, token: Token) {
       self.tokens.push(Sp::one(token, self.pos))
    }

    pub fn name(&mut self, ch: char) {
        let mut s = String::from(ch);
        while let Some(ch) = self.peek() {
            if !ch.is_alphabetic() { break }
            s.push(self.next().unwrap());
        }
        self.push(Token::Var(s))
    } 

    pub fn number(&mut self, ch: char) {
        let mut raw = String::from(ch);
        while let Some(ch) = self.peek() {
            if !ch.is_numeric() { break }
            raw.push(self.next().unwrap());
        }
        self.push(Token::Number(raw.parse().unwrap()))
    }

    pub fn string(&mut self) {
        let mut s = String::new();
        while let Some(ch) = self.next() {
            if ch == '"' { break }
            s.push(ch);
        }
        self.push(Token::Str(s))
    } 

    pub fn next(&mut self) -> Option<char> {
        self.index += 1;
        self.program.chars().nth(self.index - 1)
    } 

    pub fn peek(&mut self) -> Option<char> {
        self.program.chars().nth(self.index)
    } 
}
