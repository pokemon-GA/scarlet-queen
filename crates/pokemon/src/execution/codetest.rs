pub fn codetest_one() {
    use crate::pokemon_type::{test_and_draw, PokemonTypeFWG};

    const N: usize = 100;
    const R: usize = 70;
    const MAIN_LOOP: usize = 100;

    let test_name: String = format!("N={}_R={}_000", N, R);
    test_and_draw::<PokemonTypeFWG, N, R, MAIN_LOOP>(&test_name).unwrap();
}

pub fn codetest_all() {
    use crate::pokemon_type::{test_and_draw, PokemonTypeFWG};

    const N: usize = 100;
    const MAIN_LOOP: usize = 100;

    macro_rules! r_tests {
        ($( $x:expr ), *) => {
            $(
                for i in 0..5 {
                    let test_name: String = format!("N={}_R={}_{:03}", N, $x, i);
                    test_and_draw::<PokemonTypeFWG, N, $x, MAIN_LOOP>(&test_name).unwrap();
                }
            )*
        };
    }

    r_tests!(70, 80, 90);
}
