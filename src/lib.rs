use std::error::Error;
use std::io::prelude::*;


pub struct Config {
    pub input: String,
    pub output: String,
    pub separator: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {

        // args has the exe in position one
        // so panic on more than 3 inputs
        if args.len() > 4 {
            return Err("Extra operands given.");
        }

        let input = args[1].clone();
        let output = args[2].clone();
        let separator = args[3].clone();

        Ok(Config { input, output, separator })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    println!("input {}", config.input);
    println!("output {}", config.output);
    println!("sep {}", config.separator);
    
    Ok(())
}

pub fn make_date_vec(start: &str, end: &str, step: i64) -> Vec<String> {
   vec![] 
}

pub fn print_dates(dates: Vec<String>, separator: &str, format: &str) {
    for date in dates.iter() {
        println!("{}{}", date, separator);
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_make_date_vec() {
        let start = "2017-01-01";
        let end = "2017-01-02";
        let step : i64 = 1;

        assert_eq!(
            vec!["2017-01-01", "2017-01-02"],
            make_date_vec(start, end, step)
        );
    }
}
