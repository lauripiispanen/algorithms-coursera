extern crate clap;
use clap::{Arg, App};




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
    let (knapsack_size, _):(u32, _) = i.next().unwrap();

    let input = i.collect::<Vec<(u32, u32)>>();

    println!("{}", knapsack_value(&input, knapsack_size));
}

type Value = u32;
type Weight = u32;

fn knapsack_value(input:&[(Value, Weight)], knapsack_size: u32) -> u32 {
    return match input.last() {
        None => 0,
        Some(last) => {
            let a = knapsack_value(&input[0..input.len() - 1], knapsack_size);
            if last.1 > knapsack_size {
                return a;
            }
            let b = knapsack_value(&input[0..input.len() - 1], knapsack_size - last.1) + last.0;
            if a > b {
                return a;
            } else {
                return b;
            }
        }
    }
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

        assert_eq!(8, super::knapsack_value(&v, 6))
    }
}