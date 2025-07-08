mod effective;
mod pokemon_type;

pub fn dummy() {
    core::dummy();
    println!("Hello, from fitness.");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dummy() {
        dummy();
    }
}
