use std::error::Error;
use std::io::prelude::*;


pub struct Config {
    pub input: String,
    pub output: String,
    pub separator: String,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        // skip the program name
        args.next();

        if args.len() > 4 {
            return Err("Extra operands given.");
        } else if args.len() == 0 {
            return Err("Missing operand.");
        } else if args.len() == 1 {
            let num = match args.next() {
                Some(arg) => {
                    // one arg must be an integer and the start is today
                    let _foo: i64 = match arg.parse() {
                        Ok(n) => {
                            n
                        },
                        Err(_) => {
                            return Err("Invalid integer argument.");
                        },
                    };

                },
                None => return Err("Should never happen we checked the length!"),
            };
        }

        let input = String::from("2017-01-01");
        let output = String::from("2017-02-01");
        let separator = String::from(" ");

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
