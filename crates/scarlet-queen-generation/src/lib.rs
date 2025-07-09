mod individual;
mod group;
mod tmp;

pub use individual::{GenerationIndividualTrait, GenerationIndividual};
pub use group::{GroupTrait, Group};

pub fn dummy() {
    scarlet_queen_fitness::dummy();
    scarlet_queen_selector::dummy();
    scarlet_queen_replenisher::dummy();
    println!("Hello, from generation.");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dummy() {
        dummy();
    }
}
