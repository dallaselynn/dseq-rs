extern crate chrono;
#[macro_use]
extern crate clap;

use std::error::Error;
use chrono::naive::{NaiveDate};
use chrono::{Duration, Local};

pub struct Config {
    input_format: String,
    output_format: String,
    separator: String,
    step_size: i64,
    start_date: NaiveDate,
    end_date: NaiveDate,
}

impl Config {
    pub fn new(matches: clap::ArgMatches) -> Result<Config, &'static str> {
        let today = Local::today().naive_local();
        // default step size is 1
        let mut step_size: i64 = 1;
        let mut start_date = today;
        let mut end_date = today;
        
        // output_format, input_format and separator have defaults
        // so they should always be present
        // TODO: test this is a valid chrono format
        let input_format = match matches.value_of("input_format") {
            Some(arg) =>String::from(arg),
            None => return Err("No input format specified.")
        };

      let output_format = match matches.value_of("output_format") {
            Some(arg) =>String::from(arg),
            None => return Err("No output format specified.")
        };
        
        // this has a default so it should never not be present
        let separator = match matches.value_of("separator") {
            Some(arg) => String::from(arg),
            None => return Err("No separator specified.")
        };

        if matches.is_present("arg3") {
            // if there is a third arg then the input is START_DATE STEP_SIZE END_DATE
            step_size = match value_t!(matches, "arg2", i64) {
                Ok(n) => n,
                Err(_) => return Err("Invalid integer argument for step size")
            };

            // TODO: is there a better way to bake this into the Result above?
            if step_size == 0 {
                return Err("Step size can not be zero.");
            }

            let start_arg = matches.value_of("arg1").unwrap();
            let end_arg = matches.value_of("arg3").unwrap();

            start_date = match NaiveDate::parse_from_str(start_arg, input_format.as_str()) {
                Ok(d) => d,
                Err(_) => return Err("Failed to parse start date")
            };

            end_date = match NaiveDate::parse_from_str(end_arg, input_format.as_str()) {
                Ok(d) => d,
                Err(_) => return Err("Failed to parse end date")
            };            
            
        } else if matches.is_present("arg2") {
            // two args mean the values are START END - we can unwrap because
            // the else if condition checks for arg2 and arg1 is always required to be present
            let start_arg = matches.value_of("arg1").unwrap();
            let end_arg = matches.value_of("arg2").unwrap();
            
            start_date = match NaiveDate::parse_from_str(start_arg, input_format.as_str()) {
                Ok(d) => d,
                Err(_) => return Err("Failed to parse start date")
            };

            end_date = match NaiveDate::parse_from_str(end_arg, input_format.as_str()) {
                Ok(d) => d,
                Err(_) => return Err("Failed to parse end date")
            };
        } else {
            // one arg must be an integer number of dates to output starting today
            let days_to_output = match value_t!(matches, "arg1", i64) {
                Ok(n) => n,
                Err(_) => return Err("Invalid integer argument")
            };

            start_date = today;
            end_date = match start_date.checked_add_signed(Duration::days(days_to_output)) {
                Some(date) => date,
                None => return Err("Integer value too large.")
            };
        }

        Ok(Config { input_format, output_format, separator, step_size, start_date, end_date })        
    }
}

pub fn print_dates(config: Config) {
    
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
