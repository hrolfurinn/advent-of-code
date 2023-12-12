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

fn main() -> Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    let test = true;

    let input = load_input(test);

    let mut p1: i64;
    let mut p2: i64;

    let mut lines = input.lines();

    let times = get_numbers(lines.next().unwrap());
    let records = get_numbers(lines.next().unwrap());

    let races = times.iter().zip(records.iter()).collect::<Vec<_>>();

    let mut possibilites: Vec<i64> = vec![];

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
            possibilites.push(0);
            continue;
        }
        println!(
            "Largest {:?}",
            ((*time as f64 + determinant.sqrt()) / 2.0).floor()
        );
        println!(
            "Lowest {:?}",
            ((*time as f64 - determinant.sqrt()) / 2.0).ceil()
        );
        possibilites.push(
            ((*time as f64 + determinant.sqrt()) / 2.0).floor() as i64 // largest int
                - ((*time as f64 - determinant.sqrt()) / 2.0).ceil() as i64 // lowest int
                + 1, // inclusive range
        );
    }

    println!("{:?}", possibilites);
    p1 = possibilites.iter().product();
    p2 = 0;

    println!("p1: {}", p1);
    println!("p2: {}", p2);

    Ok(())
}
