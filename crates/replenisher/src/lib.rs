pub fn dummy() {
    core::dummy();
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
