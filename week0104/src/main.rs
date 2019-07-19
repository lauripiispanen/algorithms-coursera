extern crate rand;

use std::env;
use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;
use std::cmp::Ordering;

use rand::Rng;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 && args.len() != 3 {
        println!("ERROR: pass in (only) the name of the file as argument.");
        std::process::exit(1);
    }

    let filename = &args[1];

    if !Path::new(filename).is_file() {
        println!("ERROR: File {} does not exist", filename);
        std::process::exit(1);
    }

    let reps = parse_reps(&args);

    let f = File::open(filename).unwrap();
    let file = BufReader::new(&f);

    let mut nums:Vec<(u16, u16)> = file.lines()
                                .map(|r| r.unwrap())
                                .flat_map(|r| {
                                    let row_entries:Vec<u16> = r.split_whitespace()
                                            .map(|x| x.parse::<u16>().unwrap())
                                            .collect();

                                    let head = row_entries.first().unwrap().to_owned();

                                    return row_entries.iter()
                                                .skip(1)
                                                .map(move |entry| {
                                                    let first = head;
                                                    let second = entry.clone();
                                                    if first < second {
                                                        return (first, second);
                                                    } else {
                                                        return (second, first);
                                                    }
                                                })
                                                .collect::<Vec<(u16, u16)>>();
                                }).collect();

    nums.sort_by(cmp_nodes);
    nums.dedup();

    let num_lines = nums.len();

    let rng = rand::thread_rng();
    println!("Number of edges: {}", num_lines);

    let mut min = num_lines as u16;
    for _ in 1..reps {
        let current_min = min_cut(nums.clone(), rng);
        if current_min < min {
            min = current_min;
        }
    }

    println!("Min cut after {} repetitions: {}", reps, min);
}

fn parse_reps(args: &Vec<String>) -> u16 {
    if args.len() > 2 {
        return args.last().unwrap().parse::<u16>().unwrap();
    } else {
        return 100;
    }
}

fn cmp_nodes(a: &(u16, u16), b: &(u16, u16)) -> Ordering {
    if a.0 < b.0 {
        return Ordering::Less;
    } else if a.0 > b.0 {
        return Ordering::Greater;
    } else if a.1 < b.1 {
        return Ordering::Less;
    } else if a.1 > b.1 {
        return Ordering::Greater;
    } else {
        return Ordering::Equal;
    }
}

fn has_more_than_two_nodes(adjacency_list: &Vec<(u16, u16)>) -> bool {
    if adjacency_list.len() < 1 {
        return false;
    }
    let mut nodes:Vec<u16> = Vec::new();

    for (f, t) in adjacency_list {
        if !nodes.contains(&f) {
            nodes.push(*f);
        }
        if !nodes.contains(&t) {
            nodes.push(*t);
        }
        if nodes.len() > 2 {
            return true;
        }
    }
    return false;
}

fn min_cut(adjacency_list: Vec<(u16, u16)>, mut rng: rand::rngs::ThreadRng) -> u16 {
    if !has_more_than_two_nodes(&adjacency_list) {
        return adjacency_list.len() as u16;
    }
    // contract random edge
    let idx = rng.gen_range(0, adjacency_list.len());
    let (from, to) = adjacency_list[idx];
    let new_list = adjacency_list.iter()
        .map(|(f, t)| {
            let mut new_from = f;
            let mut new_to = t;
            if new_from == &to {
                new_from = &from;
            }
            if new_to == &to {
                new_to = &from;
            }
            return (*new_from, *new_to);
        })
        .filter(|(f, t)| f != t)
        .collect();

    return min_cut(new_list, rng);
}