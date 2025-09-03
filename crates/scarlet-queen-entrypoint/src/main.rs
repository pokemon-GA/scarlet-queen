use scarlet_queen_core::PokemonTypeFWG;
use scarlet_queen_entrypoint::pokemon_type::test_and_draw;

fn main() {
    // let test_name = "N=100_R=75_001";
    // test_and_draw::<PokemonTypeFWG, 100, 75>(test_name).unwrap();
    for i in 0..5 {
        let test_name: String = format!("N=100_R=80_{i:03}");
        test_and_draw::<PokemonTypeFWG, 100, 80>(&test_name).unwrap();
    }
}
