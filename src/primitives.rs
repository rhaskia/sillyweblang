use std::fmt::{Display, Write};

#[derive(Debug, Clone, Copy)]
pub enum Glyph {
    Arrow(Arrow),
    Bracket,
    Unknown(char)
}

#[derive(Debug, Clone, Copy)]
pub enum Arrow {
    Up, Down, Left, Right,
    Vertical, Horizontal,
}

impl From<char> for Glyph {
    fn from(ch: char) -> Glyph {
        match ch {
            '↑' => Glyph::Arrow(Arrow::Up),
            '↓' => Glyph::Arrow(Arrow::Down),
            '←' => Glyph::Arrow(Arrow::Left),
            '→' => Glyph::Arrow(Arrow::Right),
            '↔' => Glyph::Arrow(Arrow::Horizontal),
            '↕' => Glyph::Arrow(Arrow::Vertical),
            '[' => Glyph::Bracket,

            _ => Glyph::Unknown(ch)
        }
    }
}

impl From<Glyph> for char {
    fn from(value: Glyph) -> Self {
        match value {
            Glyph::Arrow(arr) => char::from(arr),
            Glyph::Bracket => '[',
            Glyph::Unknown(ch) => ch,
        }
    }
}

impl From<Arrow> for char {
    fn from(value: Arrow) -> Self {
        match value {
            Arrow::Up => '↑',
            Arrow::Down => '↓',
            Arrow::Left => '←',
            Arrow::Right => '→',
            Arrow::Vertical => '↕',
            Arrow::Horizontal => '↔',
        }
    }
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
