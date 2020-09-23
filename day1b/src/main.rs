use std::cmp;
use std::env;
use std::fs;

fn base_fuel_mass(mass: i64) -> i64 {
    cmp::max((mass / 3) - 2, 0)
}

fn fuel_for_fuel(mass: i64) -> i64 {
    let mut sum_fuel = 0;
    let mut f = mass;
    while f > 0 {
        f = base_fuel_mass(f);
        sum_fuel += f;
    }
    sum_fuel
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filename = args.get(1).expect("No input filename provided");
    let contents = fs::read_to_string(input_filename).expect("Could not read input file");
    let fuel_sum: i64 = contents
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .map(|mass| 
             base_fuel_mass(mass) + fuel_for_fuel(base_fuel_mass(mass))
        )
        .sum();
    println!("{}", fuel_sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuel0to8() {
        for n in 0..9 {
            assert_eq!(0, base_fuel_mass(n))
        }
    }

    #[test]
    fn fuel9() {
        assert_eq!(1, base_fuel_mass(9))
    }

    #[test]
    fn fuel12() {
        assert_eq!(2, base_fuel_mass(12))
    }

    #[test]
    fn fuel14() {
        assert_eq!(2, base_fuel_mass(14))
    }

    #[test]
    fn fuel1969() {
        assert_eq!(654, base_fuel_mass(1969))
    }

    #[test]
    fn fuel100756() {
        assert_eq!(33583, base_fuel_mass(100756))
    }

    #[test]
    fn fuel_for_fuel_14() {
        assert_eq!(2, fuel_for_fuel(14));
    }

    #[test]
    fn fuel_for_fuel_1969() {
        assert_eq!(966, fuel_for_fuel(1969));
    }

    #[test]
    fn fuel_for_fuel_100756() {
        assert_eq!(50346, fuel_for_fuel(100756));
    }
}
