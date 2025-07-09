mod group;
mod individual;
mod tmp;

pub use group::{Group, GroupTrait};
pub use individual::{GenerationIndividual, GenerationIndividualTrait};

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
