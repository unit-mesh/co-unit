use std::cmp::{Ord, Ordering};

use serde::{Deserialize, Serialize};
use crate::document::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TextRange {
    pub start: Point,
    pub end: Point,
}

impl PartialOrd for TextRange {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TextRange {
    fn cmp(&self, other: &Self) -> Ordering {
        let compare_start_byte = self.start.byte.cmp(&other.start.byte);
        let compare_size = self.size().cmp(&other.size());

        compare_start_byte.then(compare_size)
    }
}

impl TextRange {
    pub fn new(start: Point, end: Point) -> Self {
        assert!(start <= end);
        Self { start, end }
    }

    pub fn contains(&self, other: &TextRange) -> bool {
        // (self.start ... [other.start ... other.end] ... self.end)
        self.start <= other.start && other.end <= self.end
    }

    #[allow(unused)]
    pub fn contains_strict(&self, other: TextRange) -> bool {
        // (self.start ... (other.start ... other.end) ... self.end)
        self.start < other.start && other.end <= self.end
    }

    pub fn size(&self) -> usize {
        self.end.byte.saturating_sub(self.start.byte)
    }

    pub fn from_byte_range(range: std::ops::Range<usize>, line_end_indices: &[u32]) -> Self {
        let start = Point::from_byte(range.start, line_end_indices);
        let end = Point::from_byte(range.end, line_end_indices);
        Self::new(start, end)
    }
}

impl From<tree_sitter::Range> for TextRange {
    fn from(r: tree_sitter::Range) -> Self {
        Self {
            start: Point {
                byte: r.start_byte,
                line: r.start_point.row,
                column: r.start_point.column,
            },
            end: Point {
                byte: r.end_byte,
                line: r.end_point.row,
                column: r.end_point.column,
            },
        }
    }
}

impl From<TextRange> for std::ops::Range<usize> {
    fn from(r: TextRange) -> std::ops::Range<usize> {
        r.start.byte..r.end.byte
    }
}
