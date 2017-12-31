extern crate dseq_rs as dseq;
extern crate clap;

use clap::{Arg, App};
use dseq::Config;


const USAGE: &str = "
    dseq [OPTION]... LAST
    dseq [OPTION]... FIRST LAST
    dseq [OPTION]... FIRST INCREMENT LAST";

// TODO: define TEMPLATE and set in App

fn main() {
    const DEFAULT_INCREMENT: i64 = 1;

    // default increment value - can be changed in 3 argument form
    let matches = App::new("dseq")
        .version("1.0")
        .about("Print dates from first to last, in steps of INCREMENT.")
        .author("Dallas Lynn <dallas@dallaslynn.com>")
        .usage(USAGE)
        .arg(Arg::with_name("separator")
             .short("s")
             .long("separator")
             .takes_value(true)
             .default_value("\n")
             .help("use STRING to separate dates (default: \\n)")
        ).
        // TODO: validate via custom clap validator or otherwise
        // that this is a valid format string.
        arg(Arg::with_name("output_format")
            .short("o")
            .long("output")
            .takes_value(true)
            .default_value("%Y-%m-%d")
            .help("print dates in this format")
        ).
        arg(Arg::with_name("arg1")
            .index(1)
            .required(true)
        ).
        arg(Arg::with_name("arg2")
            .index(2)
        ).
        arg(Arg::with_name("arg3")
            .index(3)
        ).
        after_help("If only LAST is given, first defaults to today.  If INCREMENT is omitted, it\n \
                    defaults to 1. If FIRST is later than LAST, the sequence will be printed\n \
                    backward.  This is different than the seq command.  INCREMENT can not be zero,\n \
                    that makes no sense.\n\n \
                    FORMAT arguments to input and output must be suitable for strftime and strptime.\n\
                    The default format for both input and output is YYYY-MM-DD.\n\n\
                    Examples:\n\
                    Print the next 10 days in YYYY-MM-DD format\n\
                    $ dseq 10\n\
                    Print the last 10 days starting today in MM/DD/YYYY format\n\
                    $ dseq -o %m/%d/%Y -10\n\
                    Print the days in January, 2015\n\
                    $ dseq 2015-01-01 2015-01-31\n\
                    Print every fifth day between January 7th 2015 and May 9th 2015\n\
                    $ dseq 2015-01-07 5 2015-05-09\n\
                    Print the next 10 days in your locale's date format, comma separated\n\
                    $ dseq -o %x -s : 10\n")
        .get_matches();


    // clap provides a value_t macro but we only have 1-3 positional args possible so
    // we do it by hand for now.

    // TODO: must be some more elegant way to do this.
    // if there is a third arg then the input is FIRST INCREMENT LAST
    if matches.is_present("arg3") {
        let first = matches.value_of("arg1").unwrap();
        let increment = matches.value_of("arg2").unwrap();
        let last = matches.value_of("arg3").unwrap();
        println!("3 values given {} {} {}", first, increment, last);
    } else if matches.is_present("arg2") {
        // two args mean the values are FIRST LAST
        let first = matches.value_of("arg1").unwrap();
        let increment = DEFAULT_INCREMENT;
        let last = matches.value_of("arg2").unwrap();
        println!("2 values given {} {}", first, last);
    } else {
        // first is today by default if no value given.
        // let first = matches.value_of("arg1");
        let increment = DEFAULT_INCREMENT;
        let last = matches.value_of("arg1").unwrap();
        println!("1 value given {}", last);
        // one argument given.
    }
}
