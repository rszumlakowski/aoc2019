use std::fs;

pub struct Config {
    pub day: i64,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, String> {
        let day = Config::parse_day(args)?;
        Ok(Config{day})
    }

    fn parse_day(args: &Vec<String>) -> Result<i64, String> {
        args.get(1)
            .ok_or("No day number provided")?
            .parse::<i64>()
            .map_err(|_| "Could not parse day number as i64".to_string())
    }
}

pub fn load_input(day: i64) -> Result<String, String> {
    let filename = test_filename(day);
    let input = fs::read_to_string(&filename)
        .map_err(|e| format!("Error '{}' reading input file: '{}'", e, &filename))?
        .trim_end()
        .to_string();
    Ok(input)
}

fn test_filename(day: i64) -> String {
    format!("inputs/day{:02}.txt", day)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_new_config() {
        let args = vec![
            "aoc2019".to_string(),
            "1".to_string()
        ];
        let config = Config::new(&args).unwrap();
        assert_eq!(1, config.day);
    }

    #[test]
    fn test_new_config_day_number_unparseable() {
        let args: Vec<String> = vec![
            "aoc2019".to_string(),
            "not_an_i64".to_string()
        ];
        let result = Config::new(&args);
        assert_eq!(result.err(), Some("Could not parse day number as i64"));
    }

    #[test]
    fn test_new_config_empty_args() {
        let args: Vec<String> = vec![];
        let result = Config::new(&args);
        assert_eq!(result.err(), Some("No day number provided"));
    }

    #[test]
    fn test_test_filename() {
        assert_eq!(String::from("inputs/day01.txt"), test_filename(1));
    }

    #[test]
    fn test_load_input_valid() {
        assert_eq!(String::from("test data here"), load_input(0).unwrap());
    }

    #[test]
    fn test_load_input_file_not_found() {
        assert!(load_input(99).is_err())
    }
}
