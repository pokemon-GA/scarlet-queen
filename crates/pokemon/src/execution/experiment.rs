// for configuration of expriment, read scrapbox

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

    #[derive(serde_derive::Serialize)]
    struct LogEachTest {
        init_group: HashMap<String, usize>,
        is_divergence: bool,
    }

    #[derive(serde_derive::Serialize)]
    struct LogsEachR {
        r: usize,
        each_test: Vec<LogEachTest>,
        divergence_count: usize,
    }

    #[derive(serde_derive::Serialize)]
    struct Logs {
        each_r: Vec<LogsEachR>,
    }

    // excute a main loop only one time
    fn test_one<const R: usize>() -> LogEachTest {
        let loop_record = function::main_loop::<
            RandomInitializer<PokemonTypeFWG, N>,
            PokemonTypeGroup<PokemonTypeFWG, N, R>,
            Empty,
            N,
            R,
        >(MAIN_LOOP, &mut io::empty())
        .unwrap();

        // count init group emelemt
        let init_group: Vec<PokemonTypeFWG> = loop_record.get_init().clone();
        let mut init_group_count_pokemontype_key: HashMap<PokemonTypeFWG, usize> = HashMap::new();
        for v in init_group.into_iter() {
            *init_group_count_pokemontype_key.entry(v).or_insert(0) += 1;
        }
        let init_group_count_string_key: HashMap<String, usize> = init_group_count_pokemontype_key
            .into_iter()
            .map(|(k, v)| (format!("{:?}", k), v))
            .collect::<HashMap<String, usize>>();

        // analyze cycle
        let loop_record_wrapper: LoopRecordWrapper<
            PokemonTypeFWG,
            PokemonTypeWrapper<PokemonTypeFWG>,
            _,
        > = LoopRecordWrapper::from_loop_record(loop_record, |c| {
            <PokemonTypeWrapper<PokemonTypeFWG> as From<PokemonTypeFWG>>::from(c.clone())
        });
        let tail_cycle: CycleType = loop_record_wrapper
            .strict_find_tail_cycle(&mut io::empty())
            .unwrap();

        LogEachTest {
            init_group: init_group_count_string_key,
            is_divergence: match tail_cycle {
                CycleType::Divergence => true,
                _ => false,
            },
        }
    }

    // excute 100 main loops with same R
    fn each_r_test<const R: usize>() -> LogsEachR {
        let mut logs_each_test: Vec<LogEachTest> = Vec::new();
        let mut divergence_test_count: usize = 0;
        println!("======== r: {:3} ========", R);

        for i in 0..100 {
            let out_json_each_test: LogEachTest = test_one::<R>();
            if out_json_each_test.is_divergence {
                divergence_test_count += 1;
            }
            logs_each_test.push(out_json_each_test);
            println!("test{:03} is done.", i);
        }

        LogsEachR {
            r: R,
            each_test: logs_each_test,
            divergence_count: divergence_test_count,
        }
    }

    // excute each_r_test for all r
    macro_rules! each_r_count {
        ($( $x:expr ), *) => {
            {
                let mut result: Vec<LogsEachR> = Vec::new();
                $(
                    result.push(each_r_test::<$x>());
                )*
                result
            }
        };
    }

    for i in 0..5 {
        println!("========== i: {} ==========\n", i);
        let out: Logs = Logs {
            each_r: each_r_count!(
                70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90,
                91, 92, 93, 94, 95, 96, 97, 98, 99
            ),
        };
        let data: Vec<(usize, usize)> = out
            .each_r
            .iter()
            .map(|v: &LogsEachR| (v.r, v.divergence_count))
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
