use std::fmt::Debug;

#[derive(Clone)]
pub struct Span<T> {
    start: Position,
    value: T,
    end: Position,
}

impl<T: Debug> Debug for Span<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Position {
    line: usize,
    col: usize,
}

impl Position {
    pub fn new() -> Self {
        Position { line: 0, col: 0 }
    }
}

impl<T> Span<T> {
    pub fn one(value: T, p: Position) -> Span<T> {
        Span { value, start: p, end: p }
    }
}
