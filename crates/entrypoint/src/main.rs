fn main() {
    println!("This is entrypoint!!!");
    generation::dummy();
    initializer::dummy();
    fitness::dummy();
    selector::dummy();
    generator::dummy();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dummy() {
        main();
    }
}
