#[derive(Debug, Clone, Copy)]
pub enum Glyph {
    Arrow(Arrow),
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

            _ => Glyph::Unknown(ch)
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
