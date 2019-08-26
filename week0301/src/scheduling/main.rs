extern crate clap;
use clap::{Arg, App};

use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;

fn main() {
    let matches = App::new("Greedy algorithm for scheduling")
                        .arg(Arg::with_name("INPUT")
                            .help("Sets the input file to use")
                            .required(true)
                            .index(1))
                        .get_matches();

    let input_file = matches.value_of("INPUT").unwrap();
    println!("Value for input_file: {}", input_file);

    let f = File::open(input_file).unwrap();
    let file = BufReader::new(&f);

    let jobs:Vec<(u64, u64)> = file.lines()
                            .skip(1)
                            .map(|x| {
                                let entries:Vec<u64> = x.unwrap()
                                                           .split_whitespace()
                                                           .map(|x| x.parse::<u64>().unwrap())
                                                           .collect();
                                return (entries[0], entries[1]);
                            })
                            .collect();
    println!("Number of jobs: {}", jobs.len());
}
