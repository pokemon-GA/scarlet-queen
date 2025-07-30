use scarlet_queen_core::pokemon_type::PokemonTypeFWG;
use scarlet_queen_entrypoint::pokemon_type::test_and_draw;

fn main() {
    let test_name = "N=100_R=80_005";
    test_and_draw::<PokemonTypeFWG, 100, 80>(test_name);
}
