pub mod group;

pub fn dummy() {
    core::dummy();
    println!("Hello, from initializer.");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dummy() {
        dummy();
    }
}
