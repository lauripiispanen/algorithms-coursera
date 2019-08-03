use std::env;
use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

use itertools::Itertools;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("ERROR: pass in (only) the name of the file as argument.");
        std::process::exit(1);
    }

    let filename = &args[1];

    if !Path::new(filename).is_file() {
        println!("ERROR: File {} does not exist", filename);
        std::process::exit(1);
    }

    let f = File::open(filename).unwrap();
    let file = BufReader::new(&f);

    let (forward_nodes, backward_nodes, all_nodes) = file.lines()
                    .map(|r| r.unwrap())
                    .map(|r| {
                        let mut row_entries = r
                                .split_whitespace()
                                .map(|x| x.parse::<u64>()
                                            .unwrap());

                        let from = row_entries.next().unwrap();
                        let to = row_entries.next().unwrap();

                        return (from, to);
                    })
                    .fold(
                        (HashMap::new(), HashMap::new(), Vec::new()),
                        |(mut acc, mut acc_rev, mut all_nodes), (from, to)| {
                        let to_nodes = acc.entry(from).or_insert(Vec::new());
                        to_nodes.push(to);
                        let from_nodes = acc_rev.entry(to).or_insert(Vec::new());
                        from_nodes.push(from);

                        all_nodes.push(from);
                        all_nodes.push(to);

                        return (acc, acc_rev, all_nodes);
                    });

    let mut visited_nodes = HashSet::new();

    let mut l_final_stack = VecDeque::new();
    let mut l_temp_stack = VecDeque::new();
    let mut visit_stack = VecDeque::new();

    for n in all_nodes {
        visit_stack.push_back(n);
        loop {
            if visit_stack.is_empty() {
                while !l_temp_stack.is_empty() {
                    l_final_stack.push_front(l_temp_stack.pop_back().unwrap());
                }
                break;
            }
            let node = visit_stack.pop_back().unwrap();

            if !visited_nodes.contains(&node) {
                visited_nodes.insert(node);
                match backward_nodes.get(&node) {
                    Some(out_nodes) => for node_out in out_nodes {
                        visit_stack.push_back(*node_out);
                    },
                    None => {}
                }
                l_temp_stack.push_back(node);
            }
        }
    }
    println!("Number of nodes: {}", l_final_stack.len());
    let mut visit_stack = VecDeque::new();
    let mut scc = HashMap::new();
    let mut root = None;

    loop {
        if visit_stack.is_empty() {
            if l_final_stack.is_empty() {
                break;
            }
            let n = l_final_stack.pop_front().unwrap();
            root = Some(n);
            visit_stack.push_back(n);
        }
        let node = visit_stack.pop_back().unwrap();
        if !scc.contains_key(&node) {
            scc.insert(node, root.unwrap());

            match forward_nodes.get(&node) {
                Some(in_nodes) => for node_in in in_nodes {
                    visit_stack.push_back(*node_in);
                },
                None => {}
            }
        }
    }

    let mut pairs = scc.drain()
       .collect::<Vec<(u64, u64)>>();

    pairs.sort_by(|(_, a_root), (_, b_root)| a_root.partial_cmp(b_root).unwrap());

    let mut scc_sizes:Vec<(u64, usize)> = pairs.into_iter()
         .group_by(|(_, root)| *root)
         .into_iter()
         .map(|(key, group)| (key, group.count()))
         .collect();

    scc_sizes.sort_by(|(_, a_size), (_, b_size)| b_size.partial_cmp(a_size).unwrap());

    let output = scc_sizes.into_iter().take(5).map(|(_, sz)| sz.to_string()).collect::<Vec<String>>().join(",");

    println!("{}", output);
}