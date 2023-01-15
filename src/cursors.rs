use serde::{Deserialize, Serialize};

use std::cmp;

use crate::prelude::*;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Cursor {
    index: usize,
}

impl Cursor {
    pub fn new_within(visibility: &VisibilityRange, target_index: usize) -> Self {
        let floor = cmp::max(visibility.min_index_within(), target_index);
        let ceiling = cmp::min(visibility.max_index_within(), target_index);

        let index = if target_index < floor {
            floor
        } else if target_index > ceiling {
            ceiling
        } else {
            target_index
        };

        Self { index }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn moved_forward_within(self, visibility: &VisibilityRange, amount: usize) -> Self {
        Self {
            index: cmp::min(visibility.max_index_within(), self.index + amount),
        }
    }

    pub fn moved_backward_within(self, visibility: &VisibilityRange, amount: usize) -> Self {
        let index = if self.index >= amount { self.index - amount } else { 0 };

        Self {
            index: cmp::max(index, visibility.min_index_within()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_visibility_range() -> VisibilityRange {
        VisibilityRange::new()
            .with_max_visible(40)
            .with_index(20)
            .with_total_items(100)
    }

    #[test]
    fn new_within_stays_within_the_lower_range() {
        let range = default_visibility_range();

        let cursor = Cursor::new_within(&range, 0);

        assert_eq!(cursor.index(), 20);
    }

    #[test]
    fn new_within_stays_within_the_upper_range() {
        let range = default_visibility_range();

        let cursor = Cursor::new_within(&range, 100);

        assert_eq!(cursor.index(), 59);
    }

    #[test]
    fn moved_forward_within_stays_within_upper_range() {
        let range = default_visibility_range();

        let cursor = Cursor::new_within(&range, 0)
            .moved_forward_within(&range, 100);

        assert_eq!(cursor.index(), 59);
    }

    #[test]
    fn moved_backward_within_stays_within_upper_range() {
        let range = default_visibility_range();

        let cursor = Cursor::new_within(&range, 100)
            .moved_backward_within(&range, 500);

        assert_eq!(cursor.index(), 20);
    }
}
