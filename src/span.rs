use std::fmt::Debug;
use std::ops::{Deref, DerefMut};
use std::cmp::Ordering;

#[derive(Clone)]
pub struct Span<T> {
    pub start: Position,
    pub value: T,
    pub end: Position,
}

impl<T: Debug> Debug for Span<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

    pub fn new(value: T, start: Position, end: Position) -> Span<T> {
        Span { value, start, end }
    }
}

impl<T> Deref for Span<T> {
    type Target = T;
    fn deref(&self) -> &T {
       &self.value
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.line != other.line {
            return if self.line > other.line { Ordering::Greater } else { Ordering::Less };
        }
        if self.col != other.col {
            return if self.col > other.col { Ordering::Greater } else { Ordering::Less };
        }
        Ordering::Equal
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.line.partial_cmp(&other.line) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.col.partial_cmp(&other.col)
    }
}
