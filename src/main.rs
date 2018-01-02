extern crate dseq;
extern crate clap;

use std::process;
use clap::{App, AppSettings, Arg};
use dseq::Args;

const USAGE: &str = "
    dseq [OPTION]... LAST
    dseq [OPTION]... FIRST LAST
    dseq [OPTION]... FIRST INCREMENT LAST";

fn main() {
    // default increment value - can be changed in 3 argument form
    let matches = App::new("dseq")
        // we need this so we can give a negative number like dseq -20
        .global_settings(&[AppSettings::AllowLeadingHyphen])
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
        // TODO: validate that this is a valid chrono format string.
        arg(Arg::with_name("output_format")
            .short("o")
            .long("output")
            .takes_value(true)
            .default_value("%Y-%m-%d")
            .help("print dates in this format")
        ).
        arg(Arg::with_name("input_format")
            .short("i")
            .long("input")
            .takes_value(true)
            .default_value("%Y-%m-%d")
            .help("give argument dates in this format")
        ).
        arg(Arg::with_name("arg1")
            .index(1)
            .required(true)
            .hidden(true)
        ).
        arg(Arg::with_name("arg2")
            .index(2)
            .hidden(true)
        ).
        arg(Arg::with_name("arg3")
            .index(3)
            .hidden(true)
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

    let args = Args::new(matches).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    dseq::print_dates(args);
}
