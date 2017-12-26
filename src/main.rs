extern crate dseq_rs as dseq;

use std::env;
use std::process;

use dseq::Config;


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });

    if let Err(e) = dseq::run(config) {
        println!("{}", e);
        process::exit(1);
    }
}
