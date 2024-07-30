use std::fmt::{Display, Write};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Glyph {
    Arrow(Arrow),
    Arithmetic(Arithmetic),
    Bracket,
    Unknown(char),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Arrow {
    Up, Down, Left, Right,
    Vertical, Horizontal,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Arithmetic {
    Plus, Minus, Multiply, Divide
}

macro_rules! glyph_chars {
    ($($char:tt => $glyph:expr => $value:literal),*) => {
        impl From<char> for Glyph {
            fn from(ch: char) -> Glyph {
                match ch {
                    $(
                        $char => $glyph, 
                    )*
                    _ => Glyph::Unknown(ch)
                }
            }
        }

        impl From<Glyph> for char {
            fn from(gl: Glyph) -> char {
                $(
                    if gl == $glyph { return $char; }
                )*
                return ' ';
            }
        }

        impl Glyph {
            pub fn value(&self) -> &str {
                $(
                    if *self == $glyph { return $value; }
                )*
                ""
            }
        }
    }
}

glyph_chars! {
    '↑' => Glyph::Arrow(Arrow::Up) => "bottom",
    '↓' => Glyph::Arrow(Arrow::Down) => "top",
    '←' => Glyph::Arrow(Arrow::Left) => "left",
    '→' => Glyph::Arrow(Arrow::Right) => "right",
    '↔' => Glyph::Arrow(Arrow::Horizontal) => "width",
    '↕' => Glyph::Arrow(Arrow::Vertical) => "height",
    '[' => Glyph::Bracket => "",

    '+' => Glyph::Arithmetic(Arithmetic::Plus) => "plus",
    '-' => Glyph::Arithmetic(Arithmetic::Minus) => "minus",
    'x' => Glyph::Arithmetic(Arithmetic::Multiply) => "multiply",
    '/' => Glyph::Arithmetic(Arithmetic::Divide) => "height"
}


pub trait ToGlyph {
    fn to_glyph(self) -> Glyph;
    fn is_glyph(self) -> bool;
}

impl ToGlyph for char {
    fn to_glyph(self) -> Glyph {
        Glyph::from(self)
    }

    fn is_glyph(self) -> bool {
        match self.to_glyph() {
            Glyph::Unknown(_) => false,
            _ => true,
        }
    }
}

impl Display for Glyph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(char::from(*self))
    }
}
