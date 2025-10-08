use pokemon::{
    global_const::{N, R},
    pokemon_type::{test_and_draw, PokemonTypeFWG},
};

fn main() {
    let test_name: String = format!("N={}_R={}_000", N, R);
    test_and_draw::<PokemonTypeFWG, N, R>(&test_name).unwrap();
}
