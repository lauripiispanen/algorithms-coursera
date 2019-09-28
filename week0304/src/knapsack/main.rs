extern crate clap;
use clap::{Arg, App};

use std::convert::TryInto;


use common::read_file::read_numeric_lines;

fn main() {
    let arg_matches = App::new("Huffman coding example that calculates maximum length of a codeword in given dictionary")
                        .arg(Arg::with_name("INPUT_FILE")
                            .help("Sets the input file to use")
                            .required(true)
                            .index(1))
                        .get_matches();

    let input_file = arg_matches.value_of("INPUT_FILE").unwrap();

    let mut i = read_numeric_lines::<u32>(input_file).map(|v| (v[0], v[1]));
    let (knapsack_size, num_items):(usize, usize) =
            i.next()
             .map(|(a, b)| (
                 a.try_into().unwrap(),
                 b.try_into().unwrap()))
             .unwrap();

    let input = i.collect::<Vec<(u32, u32)>>();

    let mut output:Vec<Vec<u32>> = Vec::with_capacity(num_items);

    output.push(vec![0; knapsack_size]);
    for i in 1..=num_items {
        output.push(Vec::with_capacity(num_items));
        let input_weight:usize = input[i - 1].0.try_into().unwrap();
        let input_value:u32 = input[i - 1].1;
        for x in 0..knapsack_size {
            let a:u32 = output[i - 1][x];
            if (x as i64 - input_weight as i64) < 0 {
                output[i].push(a);
            } else {
                let b:u32 = output[i - 1][x - input_weight];
                if a > b {
                    output[i].push(a);
                } else {
                    output[i].push(b + input_value);
                }
            }
        }
    }

    println!("{}", output[num_items][knapsack_size - 1]);
}
