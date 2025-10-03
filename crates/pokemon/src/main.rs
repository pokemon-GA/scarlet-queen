use pokemon::pokemon_type::{test_and_draw, PokemonTypeFWG};

fn main() {
    const N: usize = 100;
    const R: usize = 75;
    // let test_name: String = format!("N={}_R={}_000", N, R);
    // test_and_draw::<PokemonTypeFWG, N, R>(&test_name).unwrap();
    for i in 0..5 {
        let test_name: String = format!("N={}_R={}_{i:03}", N, R);
        test_and_draw::<PokemonTypeFWG, N, R>(&test_name).unwrap();
    }
}
