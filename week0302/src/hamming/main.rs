extern crate clap;
use clap::{Arg, App};

use common::read_file::read_numeric_lines;
use common::union_find::UnionFind;

use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let arg_matches = App::new("Clustering algorithm that finds the largest value of k such that there is a k-clustering with spacing at least --spacing")
                        .arg(Arg::with_name("INPUT_FILE")
                            .help("Sets the input file to use")
                            .required(true)
                            .index(1))
                        .arg(Arg::with_name("SPACING")
                            .help("Sets the spacing to use")
                            .index(2))
                        .get_matches();

    let input_file = arg_matches.value_of("INPUT_FILE").unwrap();
    let spacing:usize = arg_matches.value_of("SPACING").and_then(|x| x.parse::<usize>().ok()).unwrap_or(3);

    let mut i = read_numeric_lines::<String>(input_file);
    let bit_depth = &i.next().unwrap()[1].parse::<u8>().unwrap();

    let n:HashSet<u32> = i.map(|v| v.iter().map(|c| match c.as_ref() {
        "1" => true,
        _ => false
    }).collect()).map(|s:Vec<bool>| to_u32(&s[..])).collect();
    let nodes:HashMap<u32, usize> = n.iter().enumerate().map(|(i, n)| (*n,i)).collect();
    let max_spacing = calculate_cluster_count(nodes, *bit_depth, spacing);

    println!("{}", max_spacing);
}

fn to_u32(slice: &[bool]) -> u32 {
    slice.iter().fold(0, |acc, &b| acc*2 + b as u32)
}

fn add_bit_combinations<'a>(prefix: u32, start_bit: u8, end_bit: u8, max_bits: u8, v: &mut Vec<u32>) {
    if max_bits > 0 {
        for n in start_bit..=end_bit {
            let new_prefix = prefix | (1 << n);
            v.push(new_prefix);
            if max_bits > 1 {
                add_bit_combinations(new_prefix, start_bit + n + 1, end_bit, max_bits - 1, v);
            }
        }
    }
}

fn calculate_cluster_count(nodes: HashMap<u32, usize>, bit_depth: u8, min_spacing: usize) -> usize {
    let mut u = UnionFind::new(nodes.len());
    'main: for (_, n) in nodes.keys().enumerate() {
        let i_a = nodes.get(n).unwrap();
        let mut masks = vec![];
        add_bit_combinations(0, 0, bit_depth - 1, (min_spacing - 1) as u8, &mut masks);
        for m in masks {
            let neighbor = *n ^ m;
            match nodes.get(&neighbor) {
                Some(i_b) => {
                    u.union(*i_a, *i_b);
                },
                _ => {}
            }
        }
    }
    return u.num_sets();
}


#[cfg(test)]
mod tests {

    #[test]
    fn test_bit_combinations() {
        let mut v = vec![];
        super::add_bit_combinations(0, 0, 2, 2, &mut v);

        assert_eq!(vec![
            0b1u32,
            0b11u32,
            0b101u32,
            0b10u32,
            0b110u32,
            0b100u32
        ], v);
    }

}