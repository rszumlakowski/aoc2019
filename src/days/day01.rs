use std::cmp::max;

pub fn calc_part_a(input: &str) -> i64 {
    input
        .lines()
        .map(|line| line.parse::<i64>().expect("Input not parseable as i64"))
        .map(|line| base_fuel(line))
        .sum()
}

pub fn calc_part_b(input: &str) -> i64 {
    input
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .map(|mass| 
             base_fuel(mass) + fuel_for_fuel(base_fuel(mass))
        )
        .sum()
}

fn base_fuel(mass: i64) -> i64 {
    max((mass / 3) - 2, 0)
}

fn fuel_for_fuel(mass: i64) -> i64 {
    let mut sum_fuel = 0;
    let mut f = mass;
    while f > 0 {
        f = base_fuel(f);
        sum_fuel += f;
    }
    sum_fuel
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_part_a() {
        assert_eq!(34241, calc_part_a("12\n14\n1969\n100756"));
    }

    #[test]
    fn fuel0to8() {
        for n in 0..9 {
            assert_eq!(0, base_fuel(n))
        }
    }

    #[test]
    fn test_fuel12() {
        assert_eq!(2, base_fuel(12))
    }

    #[test]
    fn test_fuel14() {
        assert_eq!(2, base_fuel(14))
    }

    #[test]
    fn test_fuel1969() {
        assert_eq!(654, base_fuel(1969))
    }

    #[test]
    fn test_fuel100756() {
        assert_eq!(33583, base_fuel(100756))
    }

    #[test]
    fn test_fuel_for_fuel_14() {
        assert_eq!(2, fuel_for_fuel(14));
    }

    #[test]
    fn test_fuel_for_fuel_1969() {
        assert_eq!(966, fuel_for_fuel(1969));
    }

    #[test]
    fn test_fuel_for_fuel_100756() {
        assert_eq!(50346, fuel_for_fuel(100756));
    }

    #[test]
    fn test_calc_part_b() {
        assert_eq!(51314, calc_part_b("14\n1969\n100756"));
    }
}
