use std::cmp::Ordering;
use crate::webserver::prelude::{Deserialize, Serialize};

/// A singular position in a text document
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Point {
    /// The byte index
    pub byte: usize,

    /// 0-indexed line number
    pub line: usize,

    /// Position within the line
    pub column: usize,
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.byte.cmp(&other.byte)
    }
}

impl Point {
    pub fn new(byte: usize, line: usize, column: usize) -> Self {
        Self { byte, line, column }
    }

    pub fn from_byte(byte: usize, line_end_indices: &[u32]) -> Self {
        let line = line_end_indices
            .iter()
            .position(|&line_end_byte| (line_end_byte as usize) > byte)
            .unwrap_or(0);

        let column = line
            .checked_sub(1)
            .and_then(|idx| line_end_indices.get(idx))
            .map(|&prev_line_end| byte.saturating_sub(prev_line_end as usize))
            .unwrap_or(byte);

        Self::new(byte, line, column)
    }
}
