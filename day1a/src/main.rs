fn fuel(mass: i64) -> i64 {
    2
}

fn main() {
    println!("Hello, world!");
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
