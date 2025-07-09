use crate::{error::SelectorError, Selector};
use rand::{rng, seq::IndexedRandom};

impl Selector {
    pub fn select_random(&mut self, size: usize) -> Result<(), SelectorError> {
        if self.group.is_empty() {
            return Err(SelectorError::EmptyGroupError);
        }
        if size == 0 || size > self.group.len() {
            return Err(SelectorError::OverflowGroupError);
        }

        let mut rng = rng();
        self.group = self
            .group
            .choose_multiple(&mut rng, size)
            .cloned()
            .collect();
        Ok(())
    }
}
