use crate::span::{Span as Sp, Position};
use crate::primitives::{GlyphLoader, Glyph};

pub fn lex(program: String) -> Vec<Sp<Token>> {
    let glyphs = GlyphLoader::setup();
    Lexer { program, glyphs, index: 0, pos: Position::new(), tokens: vec![] }.lex()
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
    Color(String),
    Omega,
}

impl Token {
    pub fn is_value(&self) -> bool {
        match self {
            Self::Number(_) | Self::Str(_) | Self::Omega | Self::Var(_) => true,
            _ => false
        }
    }
}

struct Lexer {
    program: String,
    index: usize,
    tokens: Vec<Sp<Token>>,
    pos: Position,
    glyphs: GlyphLoader,
}

impl Lexer {
    pub fn lex(&mut self) -> Vec<Sp<Token>> {
        while let Some(ch) = self.next() {
           match ch {
               //']' => self.push(Token::OpenBracket),
               //'[' => self.push(Token::CloseBracket),
               ')' => self.push(Token::OpenParen),
               '(' => self.push(Token::CloseParen),
               '}' => self.push(Token::OpenBrace),
               '{' => self.push(Token::CloseBrace),
               '"' => self.string(),
               '⍵' => self.push(Token::Omega),
               '#' => self.color(),
               _ if ch.is_alphabetic() => self.name(ch),
               _ if self.glyphs.contains_key(&ch) => self.glyph(ch),
               _ if ch.is_numeric() => self.number(ch),
               _ => {},
           }
        } 

        self.tokens.clone()
    } 

    pub fn push(&mut self, token: Token) {
       self.tokens.push(Sp::one(token, self.pos))
    }

    pub fn glyph(&mut self, ch: char) {
        let token = self.glyphs.match_tok(ch);
        self.push(Token::Operator(token));
    }

    pub fn color(&mut self) {
        let mut color = String::new();
        while let Some(ch) = self.peek() {
            if !matches!(ch, 'a'..='f' | 'A'..='F' | '0'..='9') {
                break
            }
            color.push(self.next().unwrap());
        }
        self.push(Token::Color(color))
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
