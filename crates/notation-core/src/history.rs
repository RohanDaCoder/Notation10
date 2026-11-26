//! Undo/redo history over document snapshots.

/// Classic two-stack undo/redo with a capacity bound.
///
/// The original exposes Undo/Redo in its toolbar; we mirror that with
/// pre-mutation snapshots of the whole document (documents are small).
#[derive(Debug, Clone)]
pub struct History<T> {
    present: T,
    undo: Vec<T>,
    redo: Vec<T>,
    capacity: usize,
}

impl<T: Clone> History<T> {
    pub fn new(initial: T) -> Self {
        Self {
            present: initial,
            undo: Vec::new(),
            redo: Vec::new(),
            capacity: 100,
        }
    }

    pub fn present(&self) -> &T {
        &self.present
    }

    /// Mutable access for live edits (dragging, typing) that were preceded by
    /// a [`History::checkpoint`] — the pre-edit snapshot is already recorded.
    pub fn present_mut(&mut self) -> &mut T {
        &mut self.present
    }

    /// Record the current state on the undo stack without changing it, so a
    /// following live-edit session (drag, stroke, erase) can be undone as one
    /// step once it commits its first mutation.
    pub fn checkpoint(&mut self) {
        self.undo.push(self.present.clone());
        self.redo.clear();
        if self.undo.len() > self.capacity {
            self.undo.remove(0);
        }
    }

    pub fn mutate<F: FnOnce(&mut T)>(&mut self, f: F) {
        let mut next = self.present.clone();
        f(&mut next);
        self.commit(next);
    }

    /// Replace the present value, pushing the old one onto the undo stack.
    pub fn commit(&mut self, next: T) {
        self.undo.push(std::mem::replace(&mut self.present, next));
        self.redo.clear();
        if self.undo.len() > self.capacity {
            self.undo.remove(0);
        }
    }

    pub fn undo(&mut self) -> bool {
        if let Some(prev) = self.undo.pop() {
            self.redo.push(std::mem::replace(&mut self.present, prev));
            true
        } else {
            false
        }
    }

    pub fn redo(&mut self) -> bool {
        if let Some(next) = self.redo.pop() {
            self.undo.push(std::mem::replace(&mut self.present, next));
            true
        } else {
            false
        }
    }

    pub fn can_undo(&self) -> bool {
        !self.undo.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn undo_redo_roundtrip() {
        let mut h = History::new(1);
        h.commit(2);
        h.commit(3);
        assert_eq!(*h.present(), 3);
        assert!(h.undo());
        assert_eq!(*h.present(), 2);
        assert!(h.undo());
        assert_eq!(*h.present(), 1);
        assert!(!h.undo());
        assert!(h.redo());
        assert_eq!(*h.present(), 2);
        assert!(h.redo());
        assert_eq!(*h.present(), 3);
        assert!(!h.redo());
    }

    #[test]
    fn fresh_commit_clears_redo() {
        let mut h = History::new(0);
        h.commit(1);
        h.undo();
        assert!(h.can_redo());
        h.commit(9);
        assert!(!h.can_redo());
        assert_eq!(*h.present(), 9);
    }

    #[test]
    fn capacity_is_bounded() {
        let mut h = History::new(0);
        h.capacity = 3;
        for i in 0..10 {
            h.commit(i);
        }
        assert!(h.undo.len() <= 3);
    }
}
