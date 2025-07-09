pub fn dummy() {
    scarlet_queen_core::dummy();
    println!("Hello, from replenisher.");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dummy() {
        dummy();
    }
}
