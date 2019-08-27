extern crate clap;
use clap::{Arg, App};

use common::read_file::read_numeric_lines;

fn main() {
    let matches = App::new("Greedy algorithm for scheduling")
                        .arg(Arg::with_name("INPUT")
                            .help("Sets the input file to use")
                            .required(true)
                            .index(1))
                        .get_matches();

    let input_file = matches.value_of("INPUT").unwrap();

    let jobs:Vec<(u64,u64)> = read_numeric_lines(input_file)
                                    .map(|e| (e[0],e[1]))
                                    .collect();

    println!("Number of jobs: {}", jobs.len());
}
