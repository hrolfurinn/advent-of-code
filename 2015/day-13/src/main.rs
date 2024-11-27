use fxhash::FxHashMap;
use itertools::Itertools;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::io::Result;

fn main() -> Result<()> {
    let test = false;

    let input = load_input(test);

    let mut happiness_map: FxHashMap<(String, String), i32> = FxHashMap::default();
    let mut guests: HashSet<String> = HashSet::new();
    let first_guest = "Me".to_string();

    for line in input.lines() {
        let parts = line.split(" ").collect::<Vec<_>>();
        let (subject, object, sign, happiness) = (parts[0], parts[10], parts[2], parts[3]);
        let sign = match sign {
            "lose" => -1,
            "gain" => 1,
            _ => !unreachable!("Messed up sign {sign}"),
        };
        happiness_map.insert(
            (
                subject.to_string(),
                object.strip_suffix(".").unwrap().to_string(),
            ),
            sign * happiness.parse::<i32>().unwrap(),
        );
        guests.insert(subject.to_string());
    }
    for guest in guests.iter() {
        happiness_map.insert((first_guest.clone(), guest.to_string()), 0);
        happiness_map.insert((guest.to_string(), first_guest.clone()), 0);
    }

    let maximum = guests
        .iter()
        .permutations(guests.len())
        .map(|mut guest_list| {
            guest_list.push(&first_guest); // WLOG assume this guest is first, circular table
            guest_list
                .iter()
                .fold(
                    (&first_guest, 0),
                    |(prev_guest, cum_happiness), curr_guest| {
                        (
                            curr_guest,
                            cum_happiness
                                + happiness_map
                                    .get(&(prev_guest.to_string(), curr_guest.to_string()))
                                    .unwrap()
                                + happiness_map
                                    .get(&(curr_guest.to_string(), prev_guest.to_string()))
                                    .unwrap(),
                        )
                    },
                )
                .1
        })
        .max()
        .unwrap();

    println!("{maximum}");

    Ok(())
}

fn load_input(test: bool) -> String {
    let path = if test {
        "./input/sample_input.txt"
    } else {
        "./input/input.txt"
    };
    read_to_string(path).unwrap_or_else(|e| {
        eprintln!("Failed to read input file: {e}");
        std::process::exit(1);
    })
}
