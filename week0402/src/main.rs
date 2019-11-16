mod read_file;

extern crate clap;
use clap::{Arg, App};
use std::collections::HashMap;
use std::collections::BTreeSet;

use itertools::Itertools;

fn main() {
    let arg_matches = App::new("TSP with dynamic programming (Held-Karp algorithm)")
                        .arg(Arg::with_name("INPUT_FILE")
                            .help("Sets the input file to use")
                            .required(true)
                            .index(1))
                        .get_matches();

    let input_file = arg_matches.value_of("INPUT_FILE").unwrap();

    let mut i = read_file::read_numeric_lines::<f64>(input_file);

    let _:usize = i.next().map(|v| v[0] as usize).unwrap();
    //let n_vertices = num_vertices.try_into().unwrap();

    let vertices:Vec<Vertex> = i.map(|v| (v[0], v[1])).collect();

    let mut distance_matrix:Vec<Vec<f64>> = vec![vec![0.0; vertices.len()]; vertices.len()];
    for x in 0..vertices.len() {
        for y in 0..vertices.len() {
            distance_matrix[x][y] = dist(vertices[x], vertices[y]);
        }
    }

    held_karp(distance_matrix);
}

type Vertex = (f64, f64);

fn dist(a: Vertex, b: Vertex) -> f64 {
    let a_dist = (a.0 - b.0).abs();
    let b_dist = (a.1 - b.1).abs();
    return (a_dist.powi(2) + b_dist.powi(2)).sqrt();
}

fn held_karp(distances: Vec<Vec<f64>>) {
    // C contents:
    //   key:
    //     - visited nodes in this subset
    //     - the node (in this subset) we used to enter this subset
    //   value:
    //     - distance to this subset
    //     - the node previous to the one we ended here

    #![allow(non_snake_case)]
    let mut C:HashMap<(BTreeSet<usize>, usize), (f64, usize)> = HashMap::with_capacity(distances.len());
    for i in 1..distances.len() {
        let mut s = BTreeSet::new();
        s.insert(i);
        C.insert((s, i), (distances[0][i], 0));
    }

    for subset_size in 2..distances.len() {
        println!("Subset size {}/{}", subset_size, distances.len());
        let subsets = (1..(distances.len())).combinations(subset_size);
        for subset in subsets {
            for node in subset.iter() {
                let current_subset:BTreeSet<usize> = subset.iter().map(|i| *i).collect();
                let mut prev_subset:BTreeSet<usize> = current_subset.clone();
                prev_subset.remove(node);

                let mut candidate_solutions = vec![];
                for prev_node in prev_subset.iter() {
                    let prev_distance = C.get(&(prev_subset.to_owned(), *prev_node)).unwrap().0;
                    let new_distance = prev_distance + distances.get(*prev_node).and_then(|i| i.get(*node)).unwrap();
                    candidate_solutions.push((new_distance, *prev_node));
                }

                candidate_solutions.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
                C.insert((current_subset, *node), *candidate_solutions.get(0).unwrap());
            }
        }
    }

    // find lowest distance that visits all nodes and then returns to starting position
    let mut tour_distances:Vec<f64> = C.iter()
     .filter(|((visited, _enter_node), (_dist, _prev_node))| visited.len() == distances.len() - 1)
     .map(|((_visited, enter_node), (dist, _prev_node))| dist + distances[0][*enter_node])
     .collect();

    tour_distances.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

    println!("{:?}", tour_distances.get(0));
}