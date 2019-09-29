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
    let (knapsack_size, _):(usize, usize) =
            i.next()
             .map(|(a, b)| (
                 a.try_into().unwrap(),
                 b.try_into().unwrap()))
             .unwrap();

    let input = i.collect::<Vec<(u32, u32)>>();

    println!("{}", knapsack_value(input, knapsack_size));
}

type Value = u32;
type Weight = u32;

fn knapsack_value(input:Vec<(Value, Weight)>, knapsack_size: usize) -> u32 {
    let num_items = input.len();
    let mut output:Vec<Vec<u32>> = Vec::with_capacity(num_items);

    output.push(vec![0; knapsack_size + 1]);
    for i in 1..=num_items {
        output.push(Vec::with_capacity(num_items));
        let input_value:u32 = input[i - 1].0;
        let input_weight:usize = input[i - 1].1.try_into().unwrap();
        for x in 0..=knapsack_size {
            let a:u32 = output[i - 1][x];
            if (x as i64 - input_weight as i64) < 0 {
                output[i].push(a);
            } else {
                let b:u32 = output[i - 1][x - input_weight] + input_value;
                if a > b {
                    output[i].push(a);
                } else {
                    output[i].push(b);
                }
            }
        }
    }

    return output[num_items][knapsack_size];
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_knapsack() {
        let mut v = vec![];
        v.push((3, 4));
        v.push((2, 3));
        v.push((4, 2));
        v.push((4, 3));

        assert_eq!(8, super::knapsack_value(v, 6))
    }
}