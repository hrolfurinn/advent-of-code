use std::env;
use std::fs::read_to_string;
use std::io::Result;

fn load_input(test: bool) -> String {
    let path = if test {
        "./input/sample_input.txt"
    } else {
        "./input/input.txt"
    };
    match read_to_string(path) {
        Ok(x) => x,
        Err(e) => {
            println!("{e:?}");
            "dummy_path".to_string()
        }
    }
}

fn get_numbers(text: &str) -> Vec<i64> {
    text.to_string()
        .split(|c: char| !c.is_numeric())
        .filter_map(|s| {
            if s.is_empty() {
                None
            } else {
                match s.trim().parse::<i64>() {
                    Ok(n) => Some(n),
                    Err(e) => {
                        println!("{e:?}");
                        None
                    }
                }
            }
        })
        .collect::<Vec<_>>()
}

fn largest_lower_int(num: f64) -> i64 {
    (num - 1.0).ceil() as i64
}

fn lowest_larger_int(num: f64) -> i64 {
    (num + 1.0).floor() as i64
}

fn get_possibilities(races: Vec<(&i64, &i64)>) -> Vec<i64> {
    let mut possibilities = vec![];
    for (time, record) in races {
        // dist = x * (time - x)
        // dist > rec <=> x^2 - time * x + rec < 0
        // <=> (x - r_1)(x - r_2) < 0
        // r_1 < x < r_2 (issue with x > 0 also, will always happen though)
        // given time^2 - 4*rec >= 0
        // where r_1,r_2 = (time +- sqrt(time^2 - 4*rec))/2
        let determinant = ((time.pow(2)) - (4 * record)) as f64;
        println!("{determinant:?}");
        if determinant < 0.0 {
            println!("Race with time {time} and record {record} has no solutions");
            possibilities.push(0);
            continue;
        }
        let upper_bound = largest_lower_int((*time as f64 + determinant.sqrt()) / 2.0);
        let lower_bound = lowest_larger_int((*time as f64 - determinant.sqrt()) / 2.0);

        if upper_bound < lower_bound {
            println!("Race with time {time} and record {record} has no integer solutions");
            possibilities.push(0);
            continue;
        }

        println!("Largest {:?}", upper_bound);
        println!("Lowest {:?}", lower_bound);
        possibilities.push(upper_bound - lower_bound + 1);
    }
    possibilities
}

fn main() -> Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    let test = false;

    let input = load_input(test);

    let mut p1: i64;
    let mut p2: i64;

    let mut lines = input.lines();

    let times = get_numbers(lines.next().unwrap());
    let records = get_numbers(lines.next().unwrap());

    let races = times.iter().zip(records.iter()).collect::<Vec<_>>();

    let possibilities1 = get_possibilities(races);
    println!("{:?}", possibilities1);

    let time = match times
        .iter()
        .map(|n| n.to_string())
        .collect::<String>()
        .parse::<i64>()
    {
        Ok(n) => n,
        Err(e) => {
            println!("{e:?}");
            0
        }
    };
    let record = match records
        .iter()
        .map(|n| n.to_string())
        .collect::<String>()
        .parse::<i64>()
    {
        Ok(n) => n,
        Err(e) => {
            println!("{e:?}");
            0
        }
    };

    let races = vec![(&time, &record)];

    let possibilities2 = get_possibilities(races);

    p1 = possibilities1.iter().product();
    p2 = possibilities2.iter().product();

    println!("p1: {}", p1);
    println!("p2: {}", p2);

    Ok(())
}
