fn main() {
    scarlet_queen_initializer::dummy();
    scarlet_queen_generation::dummy();
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
