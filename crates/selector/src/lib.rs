pub fn dummy() {
    core::dummy();
    println!("Hello, from selector.");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dummy() {
        dummy();
    }
}
