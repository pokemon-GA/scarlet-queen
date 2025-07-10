use crate::{error::SelectorError, Selector};
use rand::{rng, seq::IndexedRandom};

impl Selector {
    pub fn select_random(&mut self, remain: usize) -> Result<(), SelectorError> {
        // Check if group is not empty
        if self.group.is_empty() {
            return Err(SelectorError::EmptyGroupError);
        }
        // Check if remain is valid
        if remain == 0 || remain > self.group.len() {
            return Err(SelectorError::OverflowGroupError);
        }

        // set size
        self.size = self.group.len();

        // set selected group
        let mut rng = rng();
        self.group = self
            .group
            .choose_multiple(&mut rng, remain)
            .cloned()
            .collect();
        Ok(())
    }
}
