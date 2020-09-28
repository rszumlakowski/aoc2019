use std::env;

use aoc2019::Config;
use aoc2019::load_input;

mod days;

fn main() {
    let args: Vec<String> = env::args().collect();
    match Config::new(&args) {
        Ok(config) => config_ok(&args, config),
        Err(e) => print_err(&args, e),
    };
}

fn config_ok(args: &Vec<String>, config: Config) {
    match load_input(config.day) {
        Ok(input) => {
            if config.day == 1 {
                println!("{}", days::day01::calc_part_a(&input));
                println!("{}", days::day01::calc_part_b(&input));
            } 
        },
        Err(e) => print_err(&args, &e)
    }
}

fn print_err(args: &Vec<String>, error: &str) {
    usage(args, error)
        .into_iter()
        .for_each(|line| println!("{}", line));
}

fn usage(args: &Vec<String>, error: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    result.push(format!("Error: {}", error));
    result.push(format!("Usage: {} <day_number>", base_name(args.get(0).unwrap())));
    result.push(format!("  day_number: 1 only"));
    result
}

fn base_name(path_string: &str) -> &str {
    if path_string.trim().len() <= 0 {
        return "";
    } else {
        path_string.split('/').last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usage_message() {
        let args: Vec<String> = vec![
            "/my/path/of/binary_name".to_string(),
            "99".to_string()
        ];
        let usage_message = usage(&args, "big huge error");
        assert!(usage_message.get(0).unwrap().contains("big huge error"));
        assert!(usage_message.get(1).unwrap().starts_with("Usage: binary_name"));
    }

    #[test]
    fn test_empty_base_name() {
        assert_eq!("", base_name(""));
    }
    
    #[test]
    fn test_unit_base_name() {
        assert_eq!("dude", base_name("dude"));
    }

    #[test]
    fn test_unit_base_name_ends_with_slash() {
        assert_eq!("", base_name("dude/"));
    }

    #[test]

    fn test_base_name_for_path() {
        assert_eq!("buddy", base_name("/path/to/my/buddy"));
    }
}
