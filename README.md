dseq
================

dseq is like the Unix utility seq but for outputting dates instead of numbers.

This is a  [Rust](http://www.rust-lang.org) version of the [C Version](https://bitbucket.org/dallaselynn/dseq)
version I wrote because I could no longer figure out how to get gnulib to work after not looking at the
code for a couple years.

Example Usage
----
```bash
# Print the next 10 days in YYYY-MM-DD format
$ dseq 10


# Print the last 10 days in MM/DD/YYYY format separated by colons
$ dseq -o %m/%d/%Y -10 -s :

# Print the days in January, 2015
$ dseq 2015-01-01 2015-01-31

# Print every fifth day between January 7th 2015 and May 9th 2015
$ dseq 2015-01-07 5 2015-05-09
```

Source Code Notes
--------------------------
Built with Rust 1.22 stable like any other cargo project, ie.

```bash
# or without --release for debugging
$ cargo build --release
$ cargo install
$ cargo uninstall dseq
$ cargo test
```
