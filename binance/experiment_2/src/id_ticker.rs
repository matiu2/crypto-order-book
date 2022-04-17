//! Generates incremental message ids

pub struct Ids {
    next: usize,
}

impl Ids {
    /// Ids start at 0
    pub fn new() -> Ids {
        Ids { next: 0 }
    }
    /// Returns the next ID
    pub fn next(&mut self) -> usize {
        let result = self.next;
        self.next += 1;
        result
    }
}
