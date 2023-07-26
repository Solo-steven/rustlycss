use serde_derive::{Deserialize, Serialize};
// position should be immutable
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Position {
    pub col: usize,
    pub row: usize,
    pub index: usize
}

impl Position {
    pub fn new() -> Self {
        Position { col: 1, row: 1, index: 0 }
    }
}
