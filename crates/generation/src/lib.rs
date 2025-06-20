pub fn dummy() {
    fitness::dummy();
    selector::dummy();
    replenisher::dummy();
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
