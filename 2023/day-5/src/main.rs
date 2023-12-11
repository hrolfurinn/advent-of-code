use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct MappingDescription {
    destination_start: i64,
    source_start: i64,
    range_length: i64,
}

fn extract_numbers(line: String) -> Vec<i64> {
    line.split_whitespace()
        .filter_map(|n| n.parse::<i64>().ok())
        .collect()
}

fn map_seed(mut seed: &i64, mut mappings: &Vec<Vec<i64>>) -> i64 {
    let mut new_seed = *seed;
    for mapping in mappings.iter() {
        if mapping[1] <= new_seed && new_seed < mapping[1] + mapping[2] {
            new_seed = mapping[0] + (new_seed - mapping[1]);
            return new_seed;
        }
    }
    new_seed
}

fn split_seeds<'a>(
    mut seed_descriptions: &Vec<Vec<i64>>,
    mut mappings: &Vec<Vec<i64>>,
) -> Vec<Vec<i64>> {
    println!("Seed descriptions {:?}", seed_descriptions);
    println!("Mappings {:?}", mappings);
    let mut new_seed_descriptions = seed_descriptions.clone();
    for mapping in mappings {
        let mut new_seeds = vec![];
        for seeds in new_seed_descriptions {
            println!("In seed {:?}", seeds);
            let seed_start = (seeds[0], "s");
            let seed_end = (seeds[0] + seeds[1] - 1, "s");
            let seed_start_loc = seeds[0];
            let seed_end_loc = seeds[0] + seeds[1];
            println!("In mapping {:?}", mapping);
            let mapping_start = (mapping[1], "m");
            let mapping_end = (mapping[1] + mapping[2]-1, "m");
            let destination_mapping_start = mapping[0];
            let destination_mapping_end = mapping[0] + mapping[2];
            let source_mapping_start = mapping[1];
            let source_mapping_end = mapping[1] + mapping[2];
            let seed_to_mapping = mapping[1] - seeds[0];
            let seed_dist_minus_seed_to_mapping = seeds[1] - seed_to_mapping;
            let mut ordered_boundaries =
                Vec::from([seed_start, seed_end, mapping_start, mapping_end]);
            ordered_boundaries.sort_by(|(a, _), (b, _)| a.cmp(b));
            let _ = match ordered_boundaries
                .iter()
                .map(|(_, a)| a.to_string())
                .collect::<String>()
                .as_str()
            {
                "ssmm" => {
                    println!("ssmm");
                    new_seeds.extend(vec![vec![seeds[0], seeds[1]]]);
                }
                "smsm" => {
                    println!("smsm");
                    new_seeds.extend(vec![
                        vec![seed_start_loc, seed_to_mapping],
                        vec![destination_mapping_start, seed_dist_minus_seed_to_mapping],
                    ]);
                }
                "smms" => {
                    println!("smms");
                    new_seeds.extend(vec![
                        vec![seeds[0], mapping[1] - seeds[0]],
                        vec![
                            destination_mapping_start,
                            destination_mapping_end - destination_mapping_start,
                        ],
                        vec![
                            mapping[1] + mapping[2],
                            seeds[0] + seeds[1] - mapping[1] - mapping[2],
                        ],
                    ]);
                }
                "mssm" => {
                    println!("mssm");
                    new_seeds.extend(vec![vec![
                        destination_mapping_start + (seeds[0] - mapping[1]),
                        seeds[1],
                    ]]);
                }
                "msms" => {
                    println!("msms");
                    new_seeds.extend(vec![
                        vec![
                            destination_mapping_start + (seeds[0] - mapping[1]),
                            mapping[1] + mapping[2] - seeds[0],
                        ],
                        vec![
                            source_mapping_end,
                            seeds[0] + seeds[1] - mapping[1] - mapping[2],
                        ],
                    ]);
                }
                "mmss" => {
                    println!("mmss");
                    new_seeds.extend(vec![vec![seeds[0], seeds[1]]]);
                }
                _ => {
                    println!("Something's awry in this match");
                }
            };
            println!("Temp new seeds: {:?}", new_seeds)
        }
        println!("New seeds: {:?}", new_seeds);
        new_seed_descriptions = new_seeds;
    }
    return new_seed_descriptions;
}

fn main() -> Result<(), Box<dyn Error>> {
    // env::set_var("RUST_BACKTRACE", "1");
    let input_path = "./input/input.txt";
    let sample_input_path = "./input/sample_input.txt";
    let file = File::open(sample_input_path)?;
    let reader = BufReader::new(file);

    let mut p1 = 0;
    let mut p2 = 0;

    let mut lines = reader.lines();

    let mut seeds = lines
        .next()
        .ok_or("No lines")?
        .map(extract_numbers)
        .unwrap();

    // let mut more_seeds: Vec<i64> = vec![];

    let mut more_seeds = seeds
        .chunks(2)
        .map(|c| c.iter().map(|s| *s).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut new_block = false;
    let mut mappings: Vec<Vec<i64>> = Vec::new();

    while let Some(Ok(line)) = lines.next() {
        if line.trim().is_empty() {
            for seed in &mut seeds {
                *seed = map_seed(seed, &mappings)
            }
            // for seed in &mut more_seeds {
            //     *seed = map_seed(seed, &mappings)
            // }
            println!("Before, more seed length {:?}", more_seeds.len());
            if !mappings.is_empty() && more_seeds.len() < 10 {
                more_seeds = split_seeds(&more_seeds, &mappings);
            }
            println!("After, more seed length {:?}", more_seeds.len());
            mappings.clear();
            continue;
        }
        let numbers = extract_numbers(line);
        if !numbers.is_empty() {
            mappings.push(numbers);
        }
    }
    for seed in &mut seeds {
        *seed = map_seed(seed, &mappings)
    }
    // for seed in &mut more_seeds {
    //     *seed = map_seed(seed, &mappings)
    // }
    mappings.clear();

    p1 = *seeds.iter().min().unwrap();
    p2 = more_seeds
        .iter()
        .filter_map(|v| if v.is_empty() { None } else { Some(v[0]) })
        .min()
        .unwrap();

    println!("p1: {p1}");
    println!("p2: {p2}");

    Ok(())
}
