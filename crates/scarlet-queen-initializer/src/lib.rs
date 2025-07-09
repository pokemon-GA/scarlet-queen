pub mod group;

pub fn dummy() {
    scarlet_queen_core::dummy();
    println!("Hello, from initializer.");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_initializer() {
        let mut init = group::Initializer::default();
        init.gen_random(10);
        assert_eq!(init.group.len(), 10);
    }
}
