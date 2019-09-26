extern crate clap;
use clap::{Arg, App};

use std::cmp::Ordering;

use common::read_file::read_numeric_lines;
use common::binheap::BinHeap;

type DictionaryEntry = (usize, u32);

fn main() {
    let arg_matches = App::new("Huffman coding example that calculates maximum length of a codeword in given dictionary")
                        .arg(Arg::with_name("INPUT_FILE")
                            .help("Sets the input file to use")
                            .required(true)
                            .index(1))
                        .get_matches();

    let input_file = arg_matches.value_of("INPUT_FILE").unwrap();

    let mut i = read_numeric_lines::<u32>(input_file).map(|v| v[0]);
    let _:u32 = i.next().unwrap();

    let mut heap = BinHeap::<WeightedBinTree>::new_minheap();

    i.enumerate().map(|e| WeightedBinTree {
        max_depth: 0,
        min_depth: 0,
        left: None,
        right: None,
        total_weight: e.1 as u64,
        entry: Some(e)
    }).for_each(|e| heap.insert(e));

    while heap.len() > 1 {
        let a = heap.extract().unwrap();
        let b = heap.extract().unwrap();
        let c = join(a, b);
        heap.insert(c);
    }

    let root_node = heap.extract().unwrap();

    println!("Huffman tree max depth: {}", root_node.max_depth);
    println!("Huffman tree min depth: {}", root_node.min_depth);
}

#[derive(Eq)]
#[derive(Debug)]
struct WeightedBinTree {
    total_weight: u64,
    max_depth: usize,
    min_depth: usize,
    entry: Option<DictionaryEntry>,
    left: Option<Box<WeightedBinTree>>,
    right: Option<Box<WeightedBinTree>>,
}

impl Ord for WeightedBinTree {
    fn cmp(&self, other: &Self) -> Ordering {
        self.total_weight.cmp(&other.total_weight)
    }
}

impl PartialOrd for WeightedBinTree {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for WeightedBinTree {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

fn join(a: WeightedBinTree, b: WeightedBinTree) -> WeightedBinTree {
    let mut max_depth = a.max_depth;
    if b.max_depth > max_depth {
        max_depth = b.max_depth;
    }
    let mut min_depth = a.min_depth;
    if b.min_depth < min_depth {
        min_depth = b.min_depth;
    }
    return WeightedBinTree {
        min_depth: min_depth + 1,
        max_depth: max_depth + 1,
        total_weight: a.total_weight + b.total_weight,
        left: Some(Box::new(a)),
        right: Some(Box::new(b)),
        entry: None
    }
}