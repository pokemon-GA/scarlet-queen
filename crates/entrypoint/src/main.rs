fn main() {
    initializer::dummy();
    generation::dummy();
    println!("This is entrypoint!!!");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dummy() {
        main();
    }
}
