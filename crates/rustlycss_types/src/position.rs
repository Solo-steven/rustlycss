use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Position {
    pub col: usize,
    pub row: usize,
}

impl Default for Position {
    fn default() -> Self {
        Self::new()
    }
}

impl Position {
    pub fn new() -> Self {
        Position { col: 0, row: 0}
    }
    pub fn from(row: usize, col: usize) -> Self {
        Position { row, col }
    }
}
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Location {
    pub start: Position,
    pub finish: Position,
}

impl Default for Location {
    fn default() -> Self {
        Self::new()
    }
}

impl Location {
    pub fn new() -> Self {
        Self { start: Position::new(), finish: Position::new() }
    }
    pub fn from(start: Position, finish: Position) -> Self {
        Self { start, finish }
    }
}
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct  Span {
    pub start: usize,
    pub finish: usize,
}

impl Default for Span {
    fn default() -> Self {
        Self::new()
    }
}

impl Span {
    pub fn new() -> Self {
        Span { start: 0, finish: 0 }
    }
    pub fn from(start: usize, finish: usize) -> Self {
        Span { start, finish }
    }
}