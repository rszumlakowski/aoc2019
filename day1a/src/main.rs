use std::env;
use std::fs;

fn fuel(mass: i64) -> i64 {
    (mass / 3) - 2
}

fn calc(mass_str: &str) -> i64 {
    fuel(mass_str.parse::<i64>().unwrap())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filename = args.get(1).expect("No input filename provided");
    let contents = fs::read_to_string(input_filename).expect("Could not read input file");
    let fuel_sum: i64 = contents.lines().map(|line| calc(line)).sum();
    println!("{}", fuel_sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuel12() {
        assert_eq!(2, fuel(12))
    }

    #[test]
    fn fuel14() {
        assert_eq!(2, fuel(14))
    }

    #[test]
    fn fuel1969() {
        assert_eq!(654, fuel(1969))
    }

    #[test]
    fn fuel100756() {
        assert_eq!(33583, fuel(100756))
    }
}
