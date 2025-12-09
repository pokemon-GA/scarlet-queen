pub fn experiment_1() {
    use std::{
        collections::HashMap,
        fs::{self, File},
        io::{self, BufWriter, Empty, Write},
    };

    use plotters::style::BLACK;
    use scarlet_queen_entrypoint::{
        find_cycle::CycleType, function, initializer::RandomInitializer,
    };

    use crate::{
        general::{Chart, LoopRecordWrapper},
        pokemon_type::{PokemonTypeFWG, PokemonTypeGroup, PokemonTypeWrapper},
    };

    const N: usize = 100;
    const MAIN_LOOP: usize = 100;

    fn test_one<const R: usize>() -> OutJsonEachTest {
        let loop_record = function::main_loop::<
            RandomInitializer<PokemonTypeFWG, N>,
            PokemonTypeGroup<PokemonTypeFWG, N, R>,
            Empty,
            N,
            R,
        >(MAIN_LOOP, &mut io::empty())
        .unwrap();

        let init_group: Vec<PokemonTypeFWG> = loop_record.get_init().clone();
        let mut init_group_count: HashMap<PokemonTypeFWG, usize> = HashMap::new();
        for v in init_group.into_iter() {
            *init_group_count.entry(v).or_insert(0) += 1;
        }
        let init_group_count: HashMap<String, usize> = init_group_count
            .into_iter()
            .map(|(k, v)| (format!("{:?}", k), v))
            .collect::<HashMap<String, usize>>();

        let loop_record_wrapper: LoopRecordWrapper<
            PokemonTypeFWG,
            PokemonTypeWrapper<PokemonTypeFWG>,
            _,
        > = LoopRecordWrapper::from_loop_record(loop_record, |c| {
            <PokemonTypeWrapper<PokemonTypeFWG> as From<PokemonTypeFWG>>::from(c.clone())
        });

        let tail_cycle: CycleType = loop_record_wrapper
            .strict_find_tail_loop(&mut io::empty())
            .unwrap();
        OutJsonEachTest {
            init_group: init_group_count,
            is_divergence: match tail_cycle {
                CycleType::Divergence => true,
                _ => false,
            },
        }
    }

    fn eash_r_test<const R: usize>() -> OutJsonInner {
        let mut out_json_inner: Vec<OutJsonEachTest> = Vec::new();
        let mut count: usize = 0;
        println!("======== r: {:3} ========", R);
        for i in 0..100 {
            let out_json_each_test: OutJsonEachTest = test_one::<R>();
            if out_json_each_test.is_divergence {
                count += 1;
            }
            out_json_inner.push(out_json_each_test);
            println!("test{:03} is done.", i);
        }
        OutJsonInner {
            r: R,
            each_test: out_json_inner,
            divergence_count: count,
        }
    }

    macro_rules! each_r_count {
        ($( $x:expr ), *) => {
            {
                let mut result: Vec<OutJsonInner> = Vec::new();
                $(
                    result.push(eash_r_test::<$x>());
                )*
                result
            }
        };
    }

    #[derive(serde_derive::Serialize)]
    struct OutJsonEachTest {
        init_group: HashMap<String, usize>,
        is_divergence: bool,
    }

    #[derive(serde_derive::Serialize)]
    struct OutJsonInner {
        r: usize,
        each_test: Vec<OutJsonEachTest>,
        divergence_count: usize,
    }

    #[derive(serde_derive::Serialize)]
    struct OutJson {
        each_r: Vec<OutJsonInner>,
    }

    for i in 0..5 {
        println!("========== i: {} ==========\n", i);
        let out: OutJson = OutJson {
            each_r: each_r_count!(
                70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90,
                91, 92, 93, 94, 95, 96, 97, 98, 99
            ),
        };
        let data: Vec<(usize, usize)> = out
            .each_r
            .iter()
            .map(|v| (v.r, v.divergence_count))
            .collect::<Vec<(usize, usize)>>();

        let dir_path: String = format!("./out/test/test_1/{:02}", i);
        fs::create_dir_all(&dir_path).unwrap();

        let file_name = format!("{}/result.json", &dir_path);
        let mut file: BufWriter<File> = BufWriter::new(File::create(file_name).unwrap());

        let json_str: String = serde_json::to_string(&out).unwrap();
        file.write_all(json_str.as_bytes()).unwrap();

        let img_name = format!("{}/line.png", &dir_path);

        let mut chart = Chart::build_chart_xy("test_1_1", &img_name, 100, 101);
        chart.draw_line_graph(data, BLACK);
    }
}
