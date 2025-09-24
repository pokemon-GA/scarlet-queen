use scarlet_queen_core::PokemonTypeFWG;
use scarlet_queen_entrypoint::pokemon_type::test_and_draw;

fn main() {
    // let test_name = "N=100_R=75_001";
    // test_and_draw::<PokemonTypeFWG, 100, 75>(test_name).unwrap();
    const N: usize = 100;
    const R: usize = 90;
    for i in 0..5 {
        let test_name: String = format!("N={}_R={}_{i:03}", N, R);
        test_and_draw::<PokemonTypeFWG, N, R>(&test_name).unwrap();
    }
}
