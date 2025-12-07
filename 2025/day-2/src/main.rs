use itertools::Itertools;
use std::cmp::Ordering;
use std::fs::read_to_string;
use std::io::{Read, Result};

fn get_half(string: &str) -> u64 {
    // 1010 -> 10
    assert_ne!(string.len() % 2, 1);
    string[0..(string.len() / 2)].parse().unwrap()
}

fn get_faultline_half(string: &str) -> u64 {
    // Finds the first half of the next possible faulty code. "Next" is defined as starting with
    // the same first half as the input string, or higher. So "1013" -> "10" but also "999" -> "10"
    let string_length = string.len() as u32;
    if string_length == 1 {
        return 1;
    }; // represents 11, the first faultline
    if string_length % 2 == 0 {
        return get_half(string);
    };
    10_u64.pow(string_length / 2)
}

fn add_faultlines_inclusive(first: u64, last: u64) -> u64 {
    // Given a number i concatenated it with itself, yields
    // i \cdot ^ \left( \floor{\log_10(i)} + 1 \right) + i
    //
    // So the sum of all such numbers between n and m, inclusive, is:
    // \sum_{i=n}^m \left(i \cdot 10 ^ {\floor{\log_10(i)} + 1}\right) + i
    //
    // First of all, we can split it into two:
    // \sum_{i=n}^m \left(i \cdot 10 ^ {\floor{\log_10(i)} + 1}\right)
    // +
    // \sum_{i=n}^m i
    //
    // The latter is a simple arithmetic sum with value \frac{(m-n+1)(n + m)}{2}
    //
    // The former we split based on the value of the log function, i.e. digit count
    // Let's say we have n=55 and m=1324342. We get several buckets of digit count
    // 55,...,99           (the first leg)
    // 100,...,999         (complete legs)
    // 1000,...,9999
    // 10000,...,99999
    // 100000,...,999999
    // 1000000,...,1324342 (the last leg)
    // There could be no complete legs, and the first and last leg could be one
    //
    //
    // Within a single leg w/ log result c, we have a simple arithmetic sum:
    // \sum_{i=n'}^{m'} i \cdot 10 ^ {c}
    // = 10^{c} \cdot \sum_{i=n'}^{m'} i
    // = 10^{c} \cdot \frac{(m'-n'+1)(m' + n')}{2}
    //
    // This already gives us a good solution to the problem, we iterate through all the legs and
    // just calculate this sum in O(1) time within each leg. But we can do better
    //
    // A complete leg has complete information, and we know how many they are, so we can find a
    // formula for their value.
    //
    // I'll not bother deriving it here. If you can express it as sums, you can use a math solver
    // to find the value, or do it by hand. It boils down to:
    // (9/2) * (((55/111)*(1000^(m+1) - 1000^n)) - (5/11)*(100^(m+1) - 100^n))
    // where n is the lowest digit count, and m is the highest. In the example above this would be
    // n = 3 and m = 6
    //
    // The first/last legs will just be calculated with a simple arithmetic sum

    println!("first {first}");
    println!("last {last}");

    // The 10 log of each number 55.ilog10() = 1
    let first_log = first.ilog10();
    let last_log = last.ilog10();

    // The latter half above, representing the sum of the latter halves:
    let latter = ((last - first + 1) * (last + first)) / 2;

    println!("first_log {first_log}");
    println!("last_log {last_log}");
    println!("latter {latter}");

    // There is only one leg, so only one power of ten to consider:
    if first_log == last_log {
        return (latter * (10_u64.pow(first_log + 1))) + latter;
    };

    // The first leg: first ... 10^(first_log + 1), e.g. 55...99. Multiplying the sum by
    // 10^(first_log + 1), recall, this adds zeros to the numbers e.g. 5500 ... 9900
    let first_leg = 10_u64.pow(first_log + 1)
        * (((10_u64.pow(first_log + 1) - 1) - first + 1)
            * ((10_u64.pow(first_log + 1) - 1) + first)
            / 2);

    // The last leg: 10^(last_log) ... 10^(first_log + 1), e.g. 55...100
    let last_leg = 10_u64.pow(last_log + 1)
        * ((last - 10_u64.pow(last_log) + 1) * (last + 10_u64.pow(last_log)) / 2);

    println!("first_leg {first_leg}");
    println!("last_leg {last_leg}");

    if first_log == last_log - 1 {
        return latter + first_leg + last_leg;
    };

    // (9/2) * (((55/111)*(1000^(m+1) - 1000^n)) - (5/11)*(100^(m+1) - 100^n))
    let complete_legs = (9 / 2)
        * (((55 / 111) * (1000_u64.pow(last_log + 1) - 1000_u64.pow(first_log + 1)))
            - ((5 / 11) * (100_u64.pow(last_log + 1) - 100_u64.pow(first_log + 1))));

    println!("complete_legs {complete_legs}");

    return latter + first_leg + last_leg + complete_legs;
}

fn get_faulty(lower: &str, upper: &str) -> u64 {
    println!("lower {lower}");
    println!("upper {upper}");

    let lower_faultline_half = get_faultline_half(lower);
    let upper_faultline_half = get_faultline_half(upper);

    println!("lower_faultline_half {lower_faultline_half}");
    println!("upper_faultline_half {upper_faultline_half}");

    // We find the next faultlines in the bounds. We'll just find the lowest
    // faultline greater than or equal to the lower number, and vice versa.
    // Note that we'll have to correct for no faultlines being in bounds.
    let lower_val: u64 = lower.parse().unwrap();
    let upper_val: u64 = upper.parse().unwrap();

    println!("lower_val {lower_val}");
    println!("upper_val {upper_val}");

    let lower_faultline_val =
        lower_faultline_half * 10_u64.pow(lower_faultline_half.ilog10() + 1) + lower_faultline_half;
    let upper_faultline_val =
        upper_faultline_half * 10_u64.pow(upper_faultline_half.ilog10() + 1) + upper_faultline_half;

    println!("lower_faultline_val {lower_faultline_val}");
    println!("upper_faultline_val {upper_faultline_val}");

    let first_faultline_half = lower_faultline_half + (lower_val > lower_faultline_val) as u64;

    let last_faultline_half = upper_faultline_half - (upper_faultline_val > upper_val) as u64;

    println!("first_faultline {first_faultline_half}");
    println!("last_faultline {last_faultline_half}");
    // lower: 998, upper: 1012 with acc 132
    // lower_faultline_half 10
    // upper_faultline_half 10
    // lower_val 998
    // upper_val 1012
    // lower_faultline_val 1010
    // upper_faultline_val 1010
    // first_faultline 11
    // last_faultline 9
    // got value: 0

    match first_faultline_half.cmp(&last_faultline_half) {
        // No faultlines in bounds
        Ordering::Greater => 0,
        // Just one faultline in bounds, early exit, the function would still work
        Ordering::Equal => {
            first_faultline_half * 10_u64.pow(first_faultline_half.ilog10() + 1)
                + first_faultline_half
        }
        // Several faultlines in bounds
        Ordering::Less => add_faultlines_inclusive(first_faultline_half, last_faultline_half),
    }
}

fn main() -> Result<()> {
    let test = false;

    assert_eq!(add_faultlines_inclusive(1, 99), 495900);

    let input = load_input(test);

    let mut p1 = 0;
    let mut p2 = 0;

    // We iterate through possible faulty IDs, to save ourselves the headache of iterating through
    // every single number in a range. Giving us a headache with a quicker algorithm instead

    p1 = input
        .lines()
        .map(|line| {
            println!("{line}");
            line
        })
        .next()
        .unwrap()
        .split(',')
        .fold(0, |acc, part| {
            let (lower, upper) = part.split_once('-').unwrap();
            println!("lower: {lower}, upper: {upper} with acc {acc}");
            let value = get_faulty(lower, upper);
            println!("got value: {value}");
            println!("\n\n\n\n");
            acc + value
        });

    println!("p1: {p1}");
    println!("p2: {p2}");

    Ok(())
}

fn load_input(test: bool) -> String {
    let path = if let Some(arg) = std::env::args().nth(1) {
        if arg == "--default-input" {
            if test {
                "./input/sample_input.txt"
            } else {
                "./input/input.txt"
            }
            .to_string()
        } else {
            String::new()
        }
    } else {
        String::new()
    };
    if path != "" {
        read_to_string(&path).unwrap_or_else(|e| {
            eprintln!("Failed to read input file: {e}");
            std::process::exit(1);
        })
    } else {
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer).unwrap();
        buffer
    }
}
