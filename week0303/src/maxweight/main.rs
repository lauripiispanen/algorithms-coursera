extern crate clap;
use clap::{Arg, App};

use common::read_file::read_numeric_lines;

fn main() {
    let arg_matches = App::new("A script that calculates the max weight independent set of a path graph")
                        .arg(Arg::with_name("INPUT_FILE")
                            .help("Sets the input file to use")
                            .required(true)
                            .index(1))
                        .arg(Arg::with_name("DESIRED_VERTICES")
                            .help("query state of specific vertices (comma-separated)")
                            .required(true)
                            .index(2))
                        .get_matches();

    let input_file = arg_matches.value_of("INPUT_FILE").unwrap();
    let desired_vertices = arg_matches.value_of("DESIRED_VERTICES").unwrap();

    let mut i = read_numeric_lines::<u32>(input_file).map(|v| v[0]);
    let _:u32 = i.next().unwrap();

    let weights:Vec<u32> = i.collect();
    let mut results:Vec<u32> = Vec::with_capacity(weights.len());
    results.push(0);
    results.push(1);

    for i in 2..weights.len() {
        let prev = weights[i - 1];
        let cur = weights[i - 2] + weights[i];
        if prev > cur {
            results.push(prev);
        } else {
            results.push(cur);
        }
    }

    let mut included_nodes:Vec<usize> = Vec::new();
    let mut i = weights.len() - 1;
    while i >= 1 {
        if results[i - 1] >= results[i - 2] + weights[i] {
            i -= 1;
        } else {
            included_nodes.push(i);
            i -= 2;
            if i == 0 {
                included_nodes.push(i);
            }
        }
    }

    desired_vertices.split(',').map(|v| v.parse::<usize>().unwrap() - 1).for_each(|v| {
        if included_nodes.contains(&v) {
            print!("1");
        } else {
            print!("0");
        }
    });

}