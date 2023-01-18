use serde::{Deserialize, Serialize};

use std::cmp;
use std::ops::Range;

use crate::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VisibilityRange {
    max_visible: usize,
    total_items: usize,
    index: usize,
}

impl VisibilityRange {
    pub fn new() -> Self {
        Self {
            max_visible: 0,
            total_items: 0,
            index: 0,
        }
    }

    pub fn with_index(self, index: usize) -> Self {
        let floor = cmp::max(index, self.min_index());
        let ceiling = cmp::min(index, self.max_index());

        let index = if floor > index {
            floor
        } else if ceiling > index {
            ceiling
        } else {
            index
        };

        Self { index, ..self }
    }

    pub fn into_cursor_at(&self, index: usize) -> Cursor {
        Cursor::new_within(self, index)
    }

    pub fn with_max_visible(self, max_visible: usize) -> Self {
        Self {
            max_visible,
            ..self
        }
    }

    pub fn with_total_items(self, total_items: usize) -> Self {
        Self {
            total_items,
            ..self
        }
    }

    pub fn includes(&self, index: usize) -> bool {
        let start = self.min_index_within();
        let end = self.max_within();

        Range { start, end }.contains(&index)
    }

    fn min_index(&self) -> usize {
        0
    }

    fn max_index(&self) -> usize {
        if self.total_items > 0 {
            self.total_items - 1
        } else {
            0
        }
    }

    fn max_within(&self) -> usize {
        let delta = self.total_items - self.index;

        let absolute_max = cmp::min(self.total_items, self.max_visible + self.index);
        let relative_max = cmp::min(self.index + delta, absolute_max);

        relative_max
    }

    pub fn min_index_within(&self) -> usize {
        self.index
    }

    pub fn max_index_within(&self) -> usize {
        let max = self.max_within();

        if max > 0 {
            max - 1
        } else {
            0
        }
    }

    pub fn moved_forward(self, amount: usize) -> Self {
        Self {
            index: cmp::min(self.index + amount, self.max_index()),
            ..self
        }
    }

    pub fn moved_backward(self, amount: usize) -> Self {
        Self {
            index: if self.index >= amount {
                self.index - amount
            } else {
                0
            },
            ..self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_range_is_correct() {
        let range = VisibilityRange::new()
            .with_total_items(10)
            .with_max_visible(5)
            .with_index(0);

        assert_eq!(range.min_index_within(), 0);
        assert_eq!(range.max_index_within(), 4);
    }

    #[test]
    fn test_range_end_moves_correctly_when_moved_forward_within_range() {
        let range = VisibilityRange::new()
            .with_total_items(10)
            .with_max_visible(5)
            .with_index(0);

        let range = range.moved_forward(2);

        assert_eq!(range.min_index_within(), 2);
        assert_eq!(range.max_index_within(), 6);
    }

    #[test]
    fn test_range_end_moves_correctly_when_moved_forward_beyond_range() {
        let range = VisibilityRange::new()
            .with_total_items(10)
            .with_max_visible(5)
            .with_index(0);

        let range = range.moved_forward(20);

        assert_eq!(range.min_index_within(), 9);
        assert_eq!(range.max_index_within(), 9);
    }

    #[test]
    fn test_range_end_moves_correctly_when_moved_backward_within_range() {
        let range = VisibilityRange::new()
            .with_total_items(10)
            .with_max_visible(5)
            .with_index(5);

        let range = range.moved_backward(2);

        assert_eq!(range.min_index_within(), 3);
        assert_eq!(range.max_index_within(), 7);
    }

    #[test]
    fn test_range_end_moves_correctly_when_moved_backward_beyond_range() {
        let range = VisibilityRange::new()
            .with_total_items(10)
            .with_max_visible(5)
            .with_index(5);

        let range = range.moved_backward(20);

        assert_eq!(range.min_index_within(), 0);
        assert_eq!(range.max_index_within(), 4);
    }
}
